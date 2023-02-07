use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=pnpm-lock.yaml");
    println!("cargo:rerun-if-changed=node_modules");

    Command::new("pnpm")
        .arg("install")
        .spawn()
        .expect("Failed to run pnpm");
}
