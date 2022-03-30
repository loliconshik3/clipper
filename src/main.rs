extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut last_content = ctx.get_contents().unwrap();
    loop {
        let content = ctx.get_contents().unwrap();
        if (content != last_content && content != "") {
            last_content = content;
            ctx.set_contents(last_content.to_owned()).unwrap();
        }
    }
}
