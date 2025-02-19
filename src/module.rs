use std::fs;
use std::path::Path;
use std::io::Result;

pub struct Module {
    id: String,
    path: String,
    status: bool,
    sub_modules: Vec<Module>
}

impl Module {
    pub fn from_dir(dir: &Path) -> Result<Module> {
        let mut module: Module = Module{ id:format!("{}", dir.file_name().unwrap().to_str().unwrap()), path: "".to_string(), status:false, sub_modules:Vec::new() };
        let files = fs::read_dir(dir)?;
        for file in files.into_iter() {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                module.sub_modules.push(Module::from_dir(&path).unwrap());
            } else {
                if let Some((name, _suffix)) = file.file_name().to_str().unwrap().split_once(".") {
                    if name != dir.file_name().unwrap().to_str().unwrap() {
                        module.sub_modules.push(Module{id: name.to_string(), path: file.path().to_str().unwrap().to_string(), status: false, sub_modules: Vec::new()});
                    } else {
                        module.path = file.path().to_str().unwrap().to_string();
                    }
                }
            }
        }
        module.sub_modules.sort_by(|a, b| a.sub_modules.len().cmp(&b.sub_modules.len()));
        return Ok(module)

    }

    pub fn get_module_string(&self, depth: i32) -> String {
        let mut s: String = format!("{}[ ] {}", "    ".repeat(depth as usize), self.id);
        for sub_mod in self.sub_modules.iter() {
            s = format!("{}\n{}", s, sub_mod.get_module_string(depth + 1))
        }
        return s
    }

    pub fn get_module_paths(&self) -> Option<Vec<&String>> {
        // TODO: Put this back when status selection is working
        // if !module.status {
        //     return None
        // }

        let mut path_list = vec![&self.path];
        for sub_mod in self.sub_modules.iter() {
            if let Some(mut paths) = sub_mod.get_module_paths() {
                path_list.append(&mut paths);
            }
        }
        return Some(path_list)
    
    }
}
