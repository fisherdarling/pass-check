fn main() -> anyhow::Result<()> {
    let target_path = std::env::args().nth(1).unwrap();
    let entry_point = std::env::args().nth(2).unwrap();

    let directory = std::fs::read_dir(target_path)?;
    pass_check::run(directory, entry_point)
}
