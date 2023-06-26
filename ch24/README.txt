$ rustc doc.rs --crate-type lib --out-dir build
$ rustdoc --test --extern doc="build/libdoc.rlib" doc.rs
