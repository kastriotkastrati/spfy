use core::panic;

pub struct SetupStruct {
    pub dir_path: &'static std::path::Path,
}

impl SetupStruct {
    pub fn new(path: &'static std::path::Path) -> Self {
        return Self { dir_path: path };
    }
}

pub fn setup() -> SetupStruct {
    let path = std::path::Path::new("./.spfy-data");
    let already_has_folder = std::fs::metadata(path);

    if already_has_folder.is_ok() {
        let setup = SetupStruct::new(path);
        return setup;
    }

    let folder = std::fs::create_dir(path);
    if folder.is_err() {
        panic!("Could not generate initial folder to store images:(");
    }

    let setup = SetupStruct::new(path);
    return setup;
}
