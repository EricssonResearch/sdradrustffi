use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let sdrad_srcdir = Path::new("./secure-rewind-and-discard/src");
    let output_prefix = env::var("OUT_DIR").unwrap();
    let libsdrad_path = Path::new(&output_prefix);

    Command::new("git")
            .args(&["checkout", "sdrad_ffi"])
            .current_dir("./secure-rewind-and-discard/")
            .status()
            .unwrap();
    
    Command::new("sh")
            .arg("-c")
            .arg("make")
            .env("OUTPUT_PREFIX", &output_prefix)
            .current_dir(&sdrad_srcdir)
            .status().unwrap();

    println!(r"cargo:rustc-link-search={}", &libsdrad_path.display());
    println!(r"cargo:rustc-env=LD_LIBRARY_PATH={}", &libsdrad_path.display());
}
