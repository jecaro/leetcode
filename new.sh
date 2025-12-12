#!/usr/bin/env bash

# if no argument is given, exit with help message
if [ $# -eq 0 ]; then
    echo "Usage: $0 <crate-name>"
    exit 1
fi

crate_name=$1

# Root directory
dir=crates/$crate_name
mkdir "$dir"

cat > "$dir/Cargo.toml" <<EOL
[package]
name = "$crate_name"
version = "0.1.0"
edition = "2024"
[dependencies]
EOL

# src directory
src_dir="$dir/src"
mkdir "$src_dir"
cat > "$src_dir/lib.rs" <<EOL
pub fn ${crate_name}() {
    todo!()
}

#[cfg(test)]
mod ${crate_name} {

    #[test]
    fn example1() {
        todo!()
    }

    #[test]
    fn example2() {
        todo!()
    }
}
EOL
