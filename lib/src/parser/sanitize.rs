pub fn sanitize(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '#');
            let code = parts.next().unwrap_or("").trim();
            if !code.is_empty() {
                Some(format!("{}\n", code))
            } else {
                None
            }
        })
        .collect()
}
