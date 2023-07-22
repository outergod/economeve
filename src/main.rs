use eyre::Result;

mod domain;

fn main() -> Result<()> {
    println!("{}", domain::database()?);
    Ok(())
}
