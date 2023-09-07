fn main() {
    cc::Build::new()
        .file("c_src/gen.c")
        .compile("my_library");
}