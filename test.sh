cargo rustc -- --crate-type cdylib
cp target/debug/libq3hi.so ~/.q3a/rust/qagamex86_64.so
RUST_LOG=trace ioq3ded +set fs_game rust +set vm_game 0 +map q3dm6
