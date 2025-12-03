use anyhow::Context;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write};
use std::path::Path;
use cocoon::{Cocoon};

pub fn encrypt(
     path: Option<&str>,
    byte_stream: &[u8],
    password: &str,
) -> anyhow::Result<()>{
    let key = password.as_bytes();

    let mut cocoon = Cocoon::new(key);

    let encrypted_data = cocoon
        .wrap(byte_stream)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Cocoon encryption failed: {:?}", e)))?;
    if let Some(path) = path{
        let mut output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        output_file.write_all(&encrypted_data)?;
    }else{
        let output_path = Path::new("encrypted.cocoon");
        let mut output_file = File::create(output_path)
            .with_context(|| format!("Could not create output file: {}", output_path.display()))?;
        output_file.write_all(&encrypted_data)?;
    }


    Ok(())
}

pub fn decrypt(
    path: Option<&str>,
    byte_stream: &[u8],
    password: &str,
) -> anyhow::Result<()> {
    let key = password.as_bytes();
    let cocoon = Cocoon::new(key);
    let decrypted_data = cocoon
        .unwrap(byte_stream)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Decryption failed (Check password/file integrity): {:?}", e)))?;

    if let Some(path) = path{
         let mut output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
         output_file.write_all(&decrypted_data)?;
    }else{
        let output_path = Path::new("decrypted.txt");
        let mut output_file = File::create(output_path)
            .with_context(|| format!("Could not create output file: {}", output_path.display()))?;
        output_file.write_all(&decrypted_data)?;
    }

    Ok(())
}