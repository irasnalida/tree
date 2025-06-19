# Tree

A simple Rust CLI tool that generates a tree view of a given directory and saves it to a `tree.txt` file. You can specify folders to ignore as extra arguments.

## Features

- Outputs a tree-like structure of directories and files
- Optionally ignores specified folders
- Saves output to `tree.txt`

## Build

```
cargo build --release
```

## Usage

```
tree.exe <DIR_PATH> folder1 folder2
```

This command will generate a `tree.txt` file in the current directory, excluding the specified folders.

## Output

Saved in a `tree.txt` file:

```
my_project/
├── Cargo.toml
├── src/
│   └── main.rs
```
