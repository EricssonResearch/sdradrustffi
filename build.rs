use std::process::Command;
use std::path::Path;

fn main() {
    let sdrad_srcdir = Path::new("./secure-rewind-and-discard/src");

    Command::new("sh")
            .arg("-c")
            .arg("make")
            .current_dir(&sdrad_srcdir)
            .status().unwrap();

    println!(r"cargo:rustc-link-search={}", &sdrad_srcdir.display());
    println!(r"cargo:rustc-env=LD_LIBRARY_PATH={}", &sdrad_srcdir.display());
}
