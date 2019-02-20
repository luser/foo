extern crate cc;

fn main() {
    let mut build = cc::Build::new();
    if build.get_compiler().is_like_msvc() {
        // MSVC outputs timestamps in object files without this flag.
        build.flag("/Brepro");
    }
    build
        .file("foo.c")
        .compile("foo");
}
