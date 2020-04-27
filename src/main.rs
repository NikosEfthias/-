use sciter::window;
const UI: &[u8] = include_bytes!("../ui/index.html");
fn main() {
    let mut win = window::Builder::main_window()
        .closeable()
        .glassy()
        .resizeable()
        .with_title()
        .debug()
        .create();
    win.set_options(window::Options::DebugMode(true)).unwrap();
    win.set_title("ΤΑΧΥΔΡΟΜΟΣ");
    win.load_html(UI, None);
    win.run_app();
}
// vi: wildignore+=*/sciter/*
