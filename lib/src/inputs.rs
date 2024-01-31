use std::collections::{BTreeMap, HashMap};

use miden::{
    crypto::{MerkleStore, MerkleTree, NodeIndex, PartialMerkleTree, RpoDigest, SimpleSmt},
    math::Felt,
    AdviceInputs, MemAdviceProvider, Word,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MerkleData {
    /// String representation of a merkle tree. The merkle tree is represented as a vector of
    /// 32 byte hex strings where each string represents a leaf in the tree.
    #[serde(rename = "merkle_tree")]
    MerkleTree(Vec<String>),
    /// String representation of a Sparse Merkle Tree. The Sparse Merkle Tree is represented as a
    /// vector of tuples where each tuple consists of a u64 node index and a 32 byte hex string
    /// representing the value of the node.
    #[serde(rename = "sparse_merkle_tree")]
    SparseMerkleTree(Vec<(u64, String)>),
    /// String representation of a Partial Merkle Tree. The Partial Merkle Tree is represented as a
    /// vector of tuples where each tuple consists of a leaf index tuple (depth, index) and a 32
    /// byte hex string representing the value of the leaf.
    #[serde(rename = "partial_merkle_tree")]
    PartialMerkleTree(Vec<((u8, u64), String)>),
}

#[derive(Deserialize)]
pub struct Inputs {
    /// String representation of the initial operand stack, composed of chained field elements.
    pub operand_stack: Vec<u64>,
    /// Optional string representation of the initial advice stack, composed of chained field
    /// elements.
    pub advice_stack: Option<Vec<u64>>,
    /// Optional map of 32 byte hex strings to vectors of u64s representing the initial advice map.
    pub advice_map: Option<HashMap<String, Vec<u64>>>,
    /// Optional vector of merkle data which will be loaded into the initial merkle store. Merkle
    /// data is represented as 32 byte hex strings and node indexes are represented as u64s.
    pub merkle_store: Option<Vec<MerkleData>>,
}

impl Inputs {
    pub fn from_file(file: &str) -> Self {
        let file = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    pub fn new(
        operand_stack: Vec<u64>,
        advice_stack: Option<Vec<u64>>,
        advice_map: Option<HashMap<String, Vec<u64>>>,
        merkle_store: Option<Vec<MerkleData>>,
    ) -> Self {
        Inputs {
            operand_stack,
            advice_stack,
            advice_map,
            merkle_store,
        }
    }

    /// Parse advice map data from the input file.
    pub fn parse_advice_map(&self) -> Result<Option<HashMap<[u8; 32], Vec<Felt>>>, String> {
        let advice_map = match &self.advice_map {
            Some(advice_map) => advice_map,
            None => return Ok(None),
        };

        let map = advice_map
            .iter()
            .map(|(k, v)| {
                // decode hex key
                let mut key = [0u8; 32];
                hex::decode_to_slice(k, &mut key)
                    .map_err(|e| format!("failed to decode advice map key `{k}` - {e}"))?;

                // convert values to Felt
                let values = v
                    .iter()
                    .map(|v| {
                        Felt::try_from(*v).map_err(|e| {
                            format!("failed to convert advice map value `{v}` to Felt - {e}")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((key, values))
            })
            .collect::<Result<HashMap<[u8; 32], Vec<Felt>>, String>>()?;

        Ok(Some(map))
    }

    pub fn parse_merkle_store(&self) -> Result<Option<MerkleStore>, String> {
        let merkle_data = match &self.merkle_store {
            Some(merkle_data) => merkle_data,
            None => return Ok(None),
        };

        let mut merkle_store = MerkleStore::default();
        for data in merkle_data {
            match data {
                MerkleData::MerkleTree(data) => {
                    let leaves = Self::parse_merkle_tree(data)?;
                    let tree = MerkleTree::new(leaves)
                        .map_err(|e| format!("failed to parse a Merkle tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    println!(
                        "Added Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                }
                MerkleData::SparseMerkleTree(data) => {
                    let entries = Self::parse_sparse_merkle_tree(data)?;
                    let tree = SimpleSmt::with_leaves(u64::BITS as u8, entries)
                        .map_err(|e| format!("failed to parse a Sparse Merkle Tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    println!(
                        "Added Sparse Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                }
                MerkleData::PartialMerkleTree(data) => {
                    let entries = Self::parse_partial_merkle_tree(data)?;
                    let tree = PartialMerkleTree::with_leaves(entries)
                        .map_err(|e| format!("failed to parse a Partial Merkle Tree: {e}"))?;
                    merkle_store.extend(tree.inner_nodes());
                    println!(
                        "Added Partial Merkle tree with root {} to the Merkle store",
                        tree.root()
                    );
                }
            }
        }

        Ok(Some(merkle_store))
    }

    /// Parse and return merkle tree leaves.
    pub fn parse_merkle_tree(tree: &[String]) -> Result<Vec<Word>, String> {
        tree.iter()
            .map(|v| {
                let leaf = Self::parse_word(v)?;
                Ok(leaf)
            })
            .collect()
    }

    /// Parse and return Sparse Merkle Tree entries.
    fn parse_sparse_merkle_tree(tree: &[(u64, String)]) -> Result<Vec<(u64, Word)>, String> {
        tree.iter()
            .map(|(index, v)| {
                let leaf = Self::parse_word(v)?;
                Ok((*index, leaf))
            })
            .collect()
    }

    /// Parse and return Partial Merkle Tree entries.
    pub fn parse_partial_merkle_tree(
        tree: &[((u8, u64), String)],
    ) -> Result<Vec<(NodeIndex, RpoDigest)>, String> {
        tree.iter()
            .map(|((depth, index), v)| {
                let node_index = NodeIndex::new(*depth, *index).map_err(|e| {
                    format!(
                        "failed to create node index with depth {depth} and index {index} - {e}"
                    )
                })?;
                let leaf = Self::parse_word(v)?;
                Ok((node_index, RpoDigest::new(leaf)))
            })
            .collect()
    }

    pub fn parse_word(word_hex: &str) -> Result<Word, String> {
        let word_value = &word_hex[2..];
        let mut word_data = [0u8; 32];
        hex::decode_to_slice(word_value, &mut word_data)
            .map_err(|e| format!("failed to decode `Word` from hex {word_hex} - {e}"))?;
        let mut word = Word::default();
        for (i, value) in word_data.chunks(8).enumerate() {
            word[i] = Felt::try_from(value).map_err(|e| {
                format!("failed to convert `Word` data {word_hex} (element {i}) to Felt - {e}")
            })?;
        }
        Ok(word)
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            operand_stack: Vec::new(),
            advice_stack: None,
            advice_map: None,
            merkle_store: None,
        }
    }
}
