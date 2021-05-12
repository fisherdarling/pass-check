fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = std::env::args().nth(1).unwrap();

    let directory = std::fs::read_dir(target_path)?;
    pass_check::run(directory)
}
