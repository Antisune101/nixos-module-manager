use std::fs;
use std::path::Path;
use std::io::Result;

pub struct Module {
    id: String,
    status: bool,
    sub_modules: Vec<Module>
}

pub fn build_module_from_dir(dir: &Path) -> Result<Module> {
    let mut module: Module = Module{ id:format!("{}", dir.file_name().unwrap().to_str().unwrap()), status:false, sub_modules:Vec::new() };
    let files = fs::read_dir(dir)?;
    for file in files.into_iter() {
        let file = file?;
        let path = file.path();
        if path.is_dir() {
            module.sub_modules.push(build_module_from_dir(&path).unwrap());
        } else {
            if let Some((name, _suffix)) = file.file_name().to_str().unwrap().split_once(".") {
                if name != dir.file_name().unwrap().to_str().unwrap() {
                    module.sub_modules.push(Module{id: name.to_string(), status: false, sub_modules: Vec::new()})
                }
            }
        }
    }
    module.sub_modules.sort_by(|a, b| a.sub_modules.len().cmp(&b.sub_modules.len()));
    return Ok(module)

}

pub fn get_module_string(module: &Module, depth: i32) -> String {
    let mut s: String = format!("{}[ ] {}", "    ".repeat(depth as usize), module.id);
    for sub_mod in module.sub_modules.iter() {
        s = format!("{}\n{}", s, get_module_string(&sub_mod, depth + 1))
    }
    return s
}
