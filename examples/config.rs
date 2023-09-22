use anyhow::Result;
use xdiff::DiffConfig;

fn main() -> Result<()> {
    let content: &str = include_str!("../fixtures/test.yml");
    let config: DiffConfig = DiffConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}
