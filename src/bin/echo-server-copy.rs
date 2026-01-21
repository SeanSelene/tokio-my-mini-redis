use tokio::{fs::File, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::open("foo.txt").await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
