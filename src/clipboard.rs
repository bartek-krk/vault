use cli_clipboard;

pub struct Clipboard {}

impl Clipboard {
    pub fn copy_to_clipboard(value: &str) -> () {
        cli_clipboard::set_contents(value.to_owned()).unwrap();
    }
}