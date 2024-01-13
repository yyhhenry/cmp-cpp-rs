use anyhow::{anyhow, Result};
fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?.join("data");
    // read dir
    let read_dir = std::fs::read_dir(current_dir)?;
    for entry in read_dir {
        let file_name = entry?.file_name();
        let file_name = file_name
            .to_str()
            .ok_or(anyhow!("Invalid UTF-8 sequence"))?;
        println!("Filename: {}", file_name);
    }
    Ok(())
}
