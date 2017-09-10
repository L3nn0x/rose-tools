# ROSE Online Rust SDK
A Rust library for working with ROSE Online's file formats.


## Build
To compile the library and all the binaries simply run `cargo build --release`

## Library
[Documentation]()

This crate provides a Rust library that can be used in other projects. See
the documentation for more information

Add `roseon` as a dependency in your `Cargo.toml`
```toml
[dependencies]
roseon="*"

```
Use it in your project
```rust
extern crate roseon

use std::path::Path;
use roseon::vfs::VfsIndex;

let idx = VfsIndex::from_path(Path::new("/path/to/index.idx")).unwrap();

for vfs in idx.file_systems {
  for vfs_file in vfs.files {
    println!("File: {}", vfs_file.filepath);
  }
}
```

### Supported File formats
* IDX
* LIT

## Compatibility
* This code has only been tested against rose_129_129en and is not guaranteed 
to work with other versions of ROSE Online (e.g. naRose, jRose, etc.)
* Older versions of ROSE Online used the EUC-KR encoding for strings. This lib
converts strings to UTF-8 lossily. See [here](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)
for more information.

## Acknowledgements
Inspired by Jack Wakefield's [Revise](https://github.com/jackwakefield/Revise) 
library and all the contributors at [osRose](http://forum.dev-osrose.com/).
