use pretty_printer::pretty_print;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::open("Cargo.toml").await?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).await?;

    pretty_print!(String::from_utf8_lossy(&buffer));

    Ok(())
}
