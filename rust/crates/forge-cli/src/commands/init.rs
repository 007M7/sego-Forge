use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn run(name: &str) -> Result<()> {
    let dir = Path::new(name);
    if dir.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    fs::create_dir_all(dir)?;
    fs::write(
        dir.join("forge.toml"),
        format!(
            "[project]\nname = \"{}\"\nversion = \"0.1.0\"\n\n[workflow]\nphases = [\"context\", \"exec\", \"persist\"]\n",
            name
        ),
    )?;
    tracing::info!("Initialized forge project '{}'", name);
    println!("✅ Created forge project '{}'", name);
    Ok(())
}
