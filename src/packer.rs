use std::fs::File;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, Component};
use walkdir::{WalkDir, DirEntry};
use anyhow::{anyhow, Context, Result};

const MAX_PATH_LENGTH: usize = 32768; // 32KB для путей

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn validate_path(path: &Path) -> Result<()> {
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => 
                return Err(anyhow!("Absolute paths are not allowed")),
            Component::ParentDir => 
                return Err(anyhow!("Parent directory components are not allowed")),
            _ => {}
        }
    }
    Ok(())
}

pub fn pack(source_dir: &str) -> Result<()> {
    let source_path = Path::new(source_dir);
    
    if !source_path.is_dir() {
        return Err(anyhow!("Source is not a directory"));
    }

    let archive_name = format!("{}.baz", source_path.file_name()
        .ok_or(anyhow!("Invalid directory name"))?
        .to_str()
        .ok_or(anyhow!("Non-UTF8 directory name"))?);

    let mut writer = BufWriter::new(File::create(&archive_name)?);

    // Write header
    writer.write_all(b"BAZ")?;
    writer.write_all(&[0x02u8])?; // Version 2
    writer.write_all(&[0u8; 8])?; // Placeholder for file count

    let mut file_count = 0u64;
    for entry in WalkDir::new(source_path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok()) 
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            let relative_path = path.strip_prefix(source_path)?;
            
            validate_path(relative_path)?;
            
            let path_str = relative_path.to_str()
                .ok_or(anyhow!("Non-UTF8 path"))?;
            
            if path_str.len() > MAX_PATH_LENGTH {
                return Err(anyhow!("Path too long: {}", path_str));
            }

            // Write file header
            writer.write_all(&(path_str.len() as u64).to_be_bytes())?;
            writer.write_all(path_str.as_bytes())?;
            let file_size = entry.metadata()?.len();
            writer.write_all(&file_size.to_be_bytes())?;

            // Write file content
            process_file(path, &mut writer)?;
            file_count += 1;
        }
    }

    // Update file count
    writer.seek(SeekFrom::Start(4))?;
    writer.write_all(&file_count.to_be_bytes())?;

    Ok(())
}

fn process_file<W: Write>(path: &Path, writer: &mut W) -> Result<()> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open: {:?}", path))?;

    let mut buffer = [0u8; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let processed: Vec<u8> = buffer[..bytes_read]
            .iter()
            .flat_map(|&b| [b, 0u8])
            .collect();
        
        writer.write_all(&processed)?;
    }

    Ok(())
}