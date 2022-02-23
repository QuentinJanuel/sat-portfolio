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

// fn make_bindings(name: &str) {
//     println!("cargo:rerun-if-changed=src/solver/{}/bindings.rs", name);
//     bindgen::Builder::default()
//         .clang_arg(format!("-Ic/{}-c-bindings", name))
//         .header(format!("c/wrappers/{}-wrapper.h", name))
//         .generate()
//         .expect(&format!("Could not create bindings to {}", name))
//         .write_to_file(format!("src/solver/{}/bindings.rs", name))
//         .expect(&format!("Couldn't write bindings for {}", name));
// }

fn main() {
    download(
        "https://raw.githubusercontent.com/madler/zlib/master/zlib.h",
        "c/lib/zlib.h",
    );
    download(
        "https://raw.githubusercontent.com/madler/zlib/master/zconf.h",
        "c/lib/zconf.h",
    );
    // Minisat
    cc::Build::new()
        .cpp(true)
        .include("c/minisat")
        .include("c/lib")
        .file("c/minisat/minisat/core/Solver.cc")
        .file("c/minisat/minisat/simp/SimpSolver.cc")
        .file("c/minisat/minisat/utils/System.cc")
        .file("c/minisat-c-bindings/minisat.cc")
        .compile("minisat");
    // Manysat
    cc::Build::new()
        .cpp(true)
        .include("c/lib")
        .include("c/manysat")
        .file("c/manysat/core/Solver.cc")
        .file("c/manysat/core/Cooperation.cc")
        .file("c/manysat-c-bindings/manysat.cc")
        .compile("manysat");
    // Glucose
    let mut includes = vec![
        "c/lib",
        "c/glucose",
    ];
    if cfg!(windows) {
        includes.push("c/pthread-win32/include");
        let arch = if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "x86") {
            "x86"
        } else {
            panic!("Unsupported architecture");
        };
        println!("cargo:rustc-link-search=c/pthread-win32/lib/{}", arch);
        println!("cargo:rustc-link-lib=pthreadVC3");
    }
    cc::Build::new()
        .cpp(true)
        .includes(includes)
        // .file("c/glucose/parallel/ParallelSolver.cc")
        // .file("c/glucose/core/Solver.cc")
        .file("c/glucose-c-bindings/glucose.cc")
        .compile("glucose");
    // Bindings
    // make_bindings("minisat");
    // make_bindings("manysat");
    // make_bindings("glucose");
}
