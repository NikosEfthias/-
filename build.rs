use std::process::Command;
fn main() {
    Command::new("cp")
        .args(&[
            "./sciter/libsciter-gtk.so",
            "./target/x86_64-unknown-linux-gnu/debug/",
        ])
        .spawn()
        .unwrap();
}
