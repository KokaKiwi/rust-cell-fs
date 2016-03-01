use error::Result;
use std::fmt;
use std::path::Path;

#[macro_use] mod utils;
pub mod sys;
#[cfg(feature = "zip")] pub mod zip;

pub trait Handler: fmt::Debug {
    fn stat(&mut self, path: &Path) -> Result<::fs::stat::Stat>;
    fn read_dir(&mut self, path: &Path) -> Result<::fs::dir::ReadDirIterator>;
    fn open(&mut self, path: &Path) -> Result<::fs::file::File<::fs::file::Read>>;

    fn exists(&mut self, path: &Path) -> bool {
        self.stat(path).is_ok()
    }
}
