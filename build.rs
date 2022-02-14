use std::{
    io,
    fs::{
        self,
        File,
    },
    path::Path,
};

fn download(url: &str, to: &str) {
    let path = Path::new(to);
    println!("cargo:rerun-if-changed={}", path.display());
    if path.is_file() {
        println!("File already exists: {}", to);
        return;
    }
    let parent = path.parent().unwrap();
    let mut resp = reqwest::blocking::get(url)
        .expect(&format!("Failed to get response for {}", url));
    fs::create_dir_all(parent)
        .expect("Failed to create parent directory");
    let mut out = File::create(path)
        .expect(&format!("Failed to create file {}", to));
    io::copy(&mut resp, &mut out)
        .expect(&format!("Failed to download {}", to));
}

fn main() {
    download(
        "https://raw.githubusercontent.com/madler/zlib/master/zlib.h",
        "c/lib/zlib.h",
    );
    download(
        "https://raw.githubusercontent.com/madler/zlib/master/zconf.h",
        "c/lib/zconf.h",
    );
    cc::Build::new()
        .cpp(true)
        .include("c/minisat")
        .include("c/lib")
        .file("c/minisat/minisat/core/Solver.cc")
        .file("c/minisat/minisat/simp/SimpSolver.cc")
        .file("c/minisat/minisat/utils/System.cc")
        // .define("__STDC_LIMIT_MACROS", None)
        // .define("__STDC_FORMAT_MACROS", None)
        .compile("minisat");
}
