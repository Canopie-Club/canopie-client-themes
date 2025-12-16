#[cfg(feature = "embed")]
pub mod embed {

    use include_dir::{Dir, File, include_dir};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Resources {
        map: HashMap<String, Dir<'static>>,
    }

    pub static DEFAULT_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static");

    impl Resources {
        pub fn new() -> Self {
            Resources {
                map: HashMap::new(),
            }
        }

        pub fn add_dir(&mut self, name: &str, dir: Dir<'static>) {
            self.map.insert(name.to_string(), dir);
        }

        pub fn has_resource(&self, path: &str, base: Option<&str>) -> bool {
            let base_dir = base.map(|b| self.map.get(b)).flatten();
            let base_result = base_dir.map(|b| b.get_file(path)).flatten();

            match base_result {
                Some(_) => true,
                None => DEFAULT_DIR.get_file(path).is_some(),
            }
        }

        pub fn get_resource(&self, path: &str, base: Option<&str>) -> Option<&File> {
            let base_dir = base.map(|b| self.map.get(b)).flatten();
            let base_result = base_dir.map(|b| b.get_file(path)).flatten();

            match base_result {
                Some(file) => Some(file),
                None => DEFAULT_DIR.get_file(path),
            }
        }
    }
}
