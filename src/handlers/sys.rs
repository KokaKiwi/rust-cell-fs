use error::Result;
use std::path::{Path, PathBuf};
use super::Handler;

#[derive(Debug)]
pub struct SysHandler {
    path: PathBuf,
}

impl SysHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> SysHandler {
        SysHandler {
            path: path.into(),
        }
    }
}

impl Handler for SysHandler {
    fn stat(&mut self, path: &Path) -> Result<::fs::stat::Stat> {
        use ::fs::stat;

        let path = self.path.join(path);

        let metadata = handler_try!(path.metadata());
        let file_type = if metadata.is_file() {
            stat::FileType::File
        } else {
            stat::FileType::Directory
        };
        let permissions = stat::Permissions {
            read: true,
            write: true,
        };

        Ok(stat::Stat {
            file_type: file_type,
            permissions: permissions,
            size: metadata.len() as usize,
        })
    }

    fn read_dir(&mut self, path: &Path) -> Result<::fs::dir::ReadDirIterator> {
        let path = self.path.join(path);

        let iter = handler_try!(path.read_dir());
        let iter = iter.filter_map(|entry| entry.ok());
        let iter = iter.filter_map(|entry| entry.file_name().into_string().ok());

        Ok(Box::new(iter))
    }

    fn open(&mut self, path: &Path) -> Result<::fs::file::File<::fs::file::Read>> {
        use std::fs::File;

        let path = self.path.join(path);
        let file = handler_try!(File::open(path));

        Ok(::fs::file::File::new(Box::new(file)))
    }

    fn exists(&mut self, path: &Path) -> bool {
        let path = self.path.join(path);
        path.exists()
    }
}
