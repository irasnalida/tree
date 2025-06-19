use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <directory-path> [ignore-folder-1] [ignore-folder-2] ...",
            args[0]
        );
        std::process::exit(1);
    }

    let root = PathBuf::from(&args[1]);
    if !root.is_dir() {
        eprintln!("Error: '{}' is not a directory.", root.display());
        std::process::exit(1);
    }

    let ignore_dirs: HashSet<String> = args[2..].iter().cloned().collect();

    let output_path = root.join("tree.txt");
    let mut output_file = File::create(&output_path)?;

    writeln!(
        output_file,
        "{}{}",
        root.file_name().unwrap().to_string_lossy(),
        "/"
    )?;
    write_tree(&root, "", true, &ignore_dirs, &mut output_file)?;

    println!("Tree written to: {}", output_path.display());
    Ok(())
}

fn write_tree(
    path: &Path,
    prefix: &str,
    _is_last: bool,
    ignore_dirs: &HashSet<String>,
    output: &mut File,
) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let len = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_default();
        let is_dir = path.is_dir();
        let is_last = i == len - 1;

        let branch = if is_last { "└── " } else { "├── " };
        writeln!(
            output,
            "{}{}{}{}",
            prefix,
            branch,
            name,
            if is_dir { "/" } else { "" }
        )?;

        if is_dir && !ignore_dirs.contains(&name) {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            write_tree(&path, &new_prefix, is_last, ignore_dirs, output)?;
        }
    }

    Ok(())
}
