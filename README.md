> [!WARNING]
> This is intentionally terrible software.
> Do NOT use for actual data compression. Made with ❤️.

<div align="center">
  <picture>
    <img alt="BigAhhZip: More the Better!"
         src="https://github.com/SynthouS/BigAhhZip/blob/main/assets/bazGit.png"
         width="50%">
  </picture>

  An archiver with its own format (.baz), which is not archiving, but creates a large archive.
</div>

A **reverse archiver** that *guarantees* to make your files **2x bigger**. Perfect for:
- Pranking coworkers 
- Wasting disk space 
- Testing "how bad can code be?" scenarios 

## Installation

### From Source (Requires [Rust](https://www.rust-lang.org/))
```
git clone https://github.com/synthous/bigahhzip.git
cd bigahhzip
cargo build
```

# Usage
<h2>Create Bloated Archive</h2>

```bash
bigahhzip make ./your_folder  # Creates your_folder.baz (2x larger!)
```

```bash
bigahhzip unmake ./your_folder.baz  # Extracts to your_folder/
```

# How It Works
- Doubles every byte (adds null bytes)

- Preserves directory structure

- Uses custom .baz format (Big Annoying Zip)

# Example Workflow
```bash
# Create test files
mkdir test_data
echo "Hello World" > test_data/file.txt
```

<h3>Archive (2x size)</h3>

`bigahhzip make test_data  # Creates test_data.baz`

<h3>Verify disaster</h3>

`ls -lh test_data.baz  # Should be ~2x original size`

<h3>Extract</h3>

`bigahhzip unmake test_data.baz  # Creates test_data/ with original files`

# Why? 
- Demonstrate Rust file I/O

- Ultimate anti-productivity tool

- Because why not?

# License
see [LICENSE](LICENSE)
