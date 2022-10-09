#[cfg(test)]
mod tests {
    use rust_formal_verification::models::AndInverterGraph;
    use walkdir::WalkDir;

    fn read_aig(file_path: &str) {
        let _aig = AndInverterGraph::from_aig_path(file_path);
    }

    #[test]
    fn try_some_aig() {
        read_aig("/home/andrew/Desktop/formal_verification/hwmcc20benchmarks/hwmcc20/aig/2020/mann/rast-p00.aig");
        for aig_file_result in WalkDir::new("tests/hwmcc20_aig") {
            let aig_file = aig_file_result.unwrap();
            if aig_file.path().is_file() {
                let file_path = aig_file.path().display().to_string();
                println!("file_path = {}", file_path);
                read_aig(file_path.as_str());
            }
        }
    }
}
