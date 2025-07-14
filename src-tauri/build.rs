use std::process::Command;

fn main() {
    tauri_build::build();

    let date = chrono::Utc::now().format("%Y%m%d").to_string();

    let git_commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
}
