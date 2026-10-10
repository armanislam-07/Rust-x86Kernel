use std::process::Command;

fn main() {
    let status = Command::new("qemu-system-x86_64")
        .args(["-accel", "tcg"])
        .args([
            "-drive",
            &format!("format=raw,file={}", env!("KERNEL_IMAGE")),
        ])
        .status()
        .expect("failed to start QEMU");

    assert!(status.success(), "QEMU exited with an error");
}
