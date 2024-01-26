use math::fields::f64::BaseElement;
use miden::utils::collections::BTreeMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MerkleStore {
    pub merkle_tree: Vec<String>,

    pub sparse_merkle_tree: Vec<(u64, String)>,
}

#[derive(Deserialize)]
pub struct Inputs {
    pub operand_stack: Option<Vec<BaseElement>>,
    pub advice_stack: Option<Vec<u64>>,
    pub advice_map: Option<BTreeMap<String, Vec<u64>>>,
    pub merkle_store: Option<Vec<MerkleStore>>,
}

impl Inputs {
    pub fn from_file(file: &str) -> Self {
        let file = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    pub fn new(
        operand_stack: Option<Vec<BaseElement>>,
        advice_stack: Option<Vec<u64>>,
        advice_map: Option<BTreeMap<String, Vec<u64>>>,
        merkle_store: Option<Vec<MerkleStore>>,
    ) -> Self {
        Inputs {
            operand_stack,
            advice_stack,
            advice_map,
            merkle_store,
        }
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            operand_stack: None,
            advice_stack: None,
            advice_map: None,
            merkle_store: None,
        }
    }
}
