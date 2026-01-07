use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        Some("install") => install(),
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("Available commands: install");
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!("Available commands: install");
            std::process::exit(1);
        }
    }
}

fn install() {
    // Build release binary
    let status = Command::new("cargo")
        .args(["build", "--release", "-p", "tracey"])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    // Copy to ~/.cargo/bin
    let home = std::env::var("HOME").expect("HOME not set");
    let src = "target/release/tracey";
    let dst = format!("{}/.cargo/bin/tracey", home);

    std::fs::copy(src, &dst).expect("Failed to copy binary");
    println!("Installed tracey to {}", dst);
}
