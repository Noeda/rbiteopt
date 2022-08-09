extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/cbits/glue.cc")
        .flag("-std=c++11")
        .compile("librbiteoptglue.a");
}
