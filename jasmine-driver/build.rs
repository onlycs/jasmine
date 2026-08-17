use std::process;

fn main() {
    let sysroot = process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .expect("run `rustc --print sysroot`")
        .stdout;

    let sysroot = String::from_utf8(sysroot).unwrap();
    let sysroot = sysroot.trim();

    println!("cargo:rustc-link-search=native={sysroot}/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{sysroot}/lib");
}
