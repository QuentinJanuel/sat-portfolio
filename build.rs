fn main() {
    cc::Build::new()
        .cpp(true)
        .file("lib/main.cpp")
        .compile("main");
}
