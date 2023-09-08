# c-rust

this repo wanna demo the pointer conversion usage in c-rust FFI calling.

below is the folder tree:
```
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── c_src
│   ├── gen.c
│   └── gen.h
└── src
    └── main.rs

2 directories, 6 files
```

in `c_src` we separate the header and impl.

we declare the necessary func as `extern C` in `main.rs`

```
// main.rs:5
extern "C" {
    fn for_each_interesting_number(callback: extern "C" fn(i32, *mut c_void), data: *mut c_void);
}
```

and for `build.rs` this is a build an entry, the content is really straightforward.
```
// build.rs
fn main() {
    cc::Build::new()
        .file("c_src/gen.c")
        .compile("my_library");
}
```

the detailed pretest shall be found in main.rs content.
