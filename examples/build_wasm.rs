use std::{
    fs,
    process::{Command, Output},
};

fn main() {
    let output = Command::new("rustup")
        .args(&["target", "add", "wasm32-unknown-unknown"])
        .output()
        .expect("Failed to add wasm target. is rustup installed?");
    print_output("Add wasm target", output);

    let output = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown"])
        .output()
        .expect("Failed to build wasm binary.");
    print_output("Build wasm binary", output);

    fs::copy(
        "./target/wasm32-unknown-unknown/debug/asteroids.wasm",
        "./examples/wasm/asteroids.wasm",
    )
    .expect("Failed to copy wasm binary to example folder");
    println!("wasm binary is in built and its place`");
}

fn print_output(message: &str, output: Output) {
    println!(
        "{}\n{}\nstdout\n{}\nstderr\n{}",
        message,
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
