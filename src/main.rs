use std::fs;

fn main() -> std::io::Result<()> {
    fs::rename("examples/a.txt", "examples/c.txt")?; // Rename a.txt to b.txt
    fs::rename("examples/b.txt", "examples/a.txt")?;
    fs::rename("examples/c.txt", "examples/b.txt")?;
    Ok(())
}