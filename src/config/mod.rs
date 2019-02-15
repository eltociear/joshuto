extern crate serde;

pub mod config;
pub mod keymap;
pub mod mimetype;
pub mod preview;
pub mod theme;

pub use self::config::JoshutoConfig;
pub use self::keymap::JoshutoKeymap;
pub use self::mimetype::JoshutoMimetype;
pub use self::preview::JoshutoPreview;
pub use self::theme::{JoshutoColorTheme, JoshutoTheme};

use self::serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

use CONFIG_HIERARCHY;

// implemented by config file implementations to turn a RawConfig into a Config
trait Flattenable<T> {
    fn flatten(self) -> T;
}

// searches a list of folders for a given file in order of preference
pub fn search_directories<P>(filename: &str, directories: &[P]) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    for path in directories.iter() {
        let filepath = path.as_ref().join(filename);
        if filepath.exists() {
            return Some(filepath);
        }
    }
    None
}

// parses a config file into its appropriate format
fn parse_config_file<T, S>(filename: &str) -> Option<S>
where
    T: DeserializeOwned + Flattenable<S>,
{
    let file_path = search_directories(filename, &CONFIG_HIERARCHY)?;
    let file_contents = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading {} file: {}", filename, e);
            return None;
        }
    };
    let config = match toml::from_str::<T>(&file_contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing {} file: {}", filename, e);
            return None;
        }
    };
    Some(config.flatten())
}
