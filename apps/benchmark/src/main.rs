use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, BufReader};

const CHUNK_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");
    // let file_name = "../../temp/data-sample/f1/F12024-PartialRaceWithRestart.bin";
    // read_binary_file(file_name).await?;
    Ok(())
}

async fn read_binary_file<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break; // End of file
        }

        // Process the read data (for example, decoding it)
        println!("Read {} bytes", bytes_read);
    }

    Ok(())
}
