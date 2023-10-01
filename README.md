# readme

```
find . -name '*.so'   # Find all files with .so extension
```

[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
[libraries]
linux.debug.x86_64 = "res://./target/debug/libchess_mater.so"
linux.x86_64 = "res://./target/debug/libchess_mater.so"

`cargo build && cp ./target/debug/libchess_mater.so .`

