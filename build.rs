extern crate gcc;

fn main() {
    gcc::compile_library("liblibrary.a", &["src/library.c"]);
}
