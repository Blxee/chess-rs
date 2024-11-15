use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend/");

    let status = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("frontend/")
        .status()
        .expect("[Error]: failed to run 'npm run build'");

    if !status.success() {
        panic!("[Error]: npm build failed");
    }
}
