use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use anyhow::{anyhow, Context, Result};
use crate::utils::unprocess_data;

const MAX_PATH_LENGTH: usize = 32768;
const MAX_FILE_SIZE: u64 = 16 * 1024 * 1024 * 1024; // 16GB

pub fn unpack(archive_path: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(archive_path)?);
    
    // Validate header
    let mut magic = [0u8; 3];
    reader.read_exact(&mut magic)?;
    if magic != *b"BAZ" {
        return Err(anyhow!("Not a BAZ archive"));
    }

    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    if version[0] != 0x02 {
        return Err(anyhow!("Unsupported version: {}", version[0]));
    }

    // Read file count
    let mut count_buf = [0u8; 8];
    reader.read_exact(&mut count_buf)?;
    let file_count = u64::from_be_bytes(count_buf);

    // Sanity checks
    if file_count > 1_000_000 {
        return Err(anyhow!("Suspicious file count: {}", file_count));
    }

    let output_dir = Path::new(archive_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(anyhow!("Invalid archive name"))?;
    
    fs::create_dir_all(output_dir)?;

    for _ in 0..file_count {
        // Read path length
        let mut path_len_buf = [0u8; 8];
        reader.read_exact(&mut path_len_buf)?;
        let path_len = u64::from_be_bytes(path_len_buf);

        if path_len > MAX_PATH_LENGTH as u64 {
            return Err(anyhow!(
                "Path length {} exceeds maximum allowed {} bytes",
                path_len,
                MAX_PATH_LENGTH
            ));
        }

        // Read path
        let mut path_bytes = vec![0u8; path_len as usize];
        reader.read_exact(&mut path_bytes)?;
        let path = String::from_utf8(path_bytes)
            .context("Invalid UTF-8 in path")?;

        // Validate path
        if path.contains("..") || path.starts_with('/') || path.starts_with('\\') {
            return Err(anyhow!("Invalid path: {}", path));
        }

        // Read file size
        let mut size_buf = [0u8; 8];
        reader.read_exact(&mut size_buf)?;
        let original_size = u64::from_be_bytes(size_buf);

        if original_size > MAX_FILE_SIZE {
            return Err(anyhow!(
                "File size {} exceeds maximum allowed {} bytes",
                original_size,
                MAX_FILE_SIZE
            ));
        }

        // Create output path
        let output_path = Path::new(output_dir).join(&path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Stream file content
        let mut output_file = File::create(&output_path)?;
        let mut remaining = original_size * 2;
        let mut buffer = vec![0u8; 4096];

        while remaining > 0 {
            let read_size = (buffer.len() as u64).min(remaining) as usize;
            reader.read_exact(&mut buffer[..read_size])?;
            
            let processed = unprocess_data(&buffer[..read_size]);
            output_file.write_all(&processed)?;
            
            remaining -= read_size as u64;
        }
    }

    Ok(())
}