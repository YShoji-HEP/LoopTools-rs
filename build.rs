use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use tar::Archive;

fn main() {
    let version = std::env::var("LOOP_TOOLS_VERSION").unwrap_or("2.16".to_string());
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = Path::new(&out_dir).join(format!("LoopTools-{version}-src"));
    let tar_path = Path::new(&out_dir).join(format!("LoopTools-{version}.tar.gz"));
    if !tar_path.exists() {
        let url = format!("https://feynarts.de/looptools/LoopTools-{version}.tar.gz");
        let mut response = reqwest::blocking::get(url).expect("Failed to download");
        let mut file = File::create(&tar_path).expect("Failed to create tar file");
        response
            .copy_to(&mut file)
            .expect("Failed to save tar file");
    }
    if !build_dir.exists() {
        let tar_gz = File::open(&tar_path).expect("Failed to open tar file");
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&build_dir).expect("Failed to unpack");
    }

    let build_dir = build_dir.join(format!("LoopTools-{version}"));
    let dst = autotools::Config::new(&build_dir)
        .make_target("lib")
        .build();

    let status = Command::new("bash")
        .current_dir(format!("{out_dir}/build/build"))
        .arg(format!("./fcc"))
        .arg(format!("{manifest_dir}/src/bridge.c"))
        .arg(format!("-c"))
        .arg(format!("-I."))
        .arg(format!("-L."))
        .arg(format!("-looptools"))
        .status()
        .expect("Failed to execute process");
    if !status.success() {
        panic!("Script failed with status: {}", status);
    }

    let status = Command::new("ar")
        .arg("rcs")
        .arg(format!("{out_dir}/build/build/liblooptoolsbridge.a"))
        .arg(format!("{out_dir}/build/build/bridge.o"))
        .status()
        .expect("Failed to execute process");
    if !status.success() {
        panic!("Script failed with status: {}", status);
    }

    println!(
        "cargo:rustc-link-search=native={}/build/build",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=looptoolsbridge");
    println!("cargo:rustc-link-lib=static=ooptools");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=gcc");
    println!("cargo:rustc-link-lib=gcc_s");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");
}
