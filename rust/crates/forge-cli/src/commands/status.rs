use anyhow::Result;
use std::path::Path;

pub fn show() -> Result<()> {
    let forge_toml = Path::new("forge.toml");
    if forge_toml.exists() {
        let content = std::fs::read_to_string(forge_toml)?;
        println!("📋 Current project status:\n{}", content);
    } else {
        println!("ℹ️  No forge.toml found in current directory. Run `forge init <name>` to create one.");
    }
    Ok(())
}
