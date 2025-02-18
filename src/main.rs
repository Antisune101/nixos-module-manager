use std::path::Path;

mod app;
mod module;

use module::{
    build_module_from_dir,
    get_module_string
};

fn main() {
    let module_dir = Path::new("/home/antisune/.config/dotfiles/modules");
    let module = build_module_from_dir(module_dir).unwrap();
    println!("{}", get_module_string(&module, 0));
    println!("Hello, world!");
}
