use std::process::Command;

fn main() {
    // Build tailwindcss at compile time
    let status = Command::new("npx")
        .arg("@tailwindcss/cli")
        .arg("-i")
        .arg("public/css/style.css")
        .arg("-o")
        .arg("public/css/tailwindcss.css")
        .status()
        .expect("failed to run npx @tailwindcss/cli");

    if !status.success() {
        panic!("tailwind CLI failed");
    }
}
