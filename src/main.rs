use std::env;
use anyhow::{Result, anyhow};

mod packer;
mod unpacker;
mod utils;

enum Commands {
    Make { folder: String },
    Unmake { archive: String },
}

fn parse_args() -> Result<Commands> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        return Err(anyhow!("Usage: {} [make|unmake] <path>", args[0]));
    }

    match args[1].to_lowercase().as_str() {
        "make" => Ok(Commands::Make { folder: args[2].clone() }),
        "unmake" => Ok(Commands::Unmake { archive: args[2].clone() }),
        _ => Err(anyhow!("Invalid command. Use 'make' or 'unmake'")),
    }
}

fn main() -> Result<()> {
    let command = parse_args()?;

    match command {
        Commands::Make { folder } => packer::pack(&folder)?,
        Commands::Unmake { archive } => unpacker::unpack(&archive)?,
    }

    Ok(())
}