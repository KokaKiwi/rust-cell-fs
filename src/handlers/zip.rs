extern crate zip;

use error::Result;
use self::zip::ZipArchive;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use super::Handler;

pub struct ZipHandler {
    path: PathBuf,
    archive: ZipArchive<File>,
}

impl ZipHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<ZipHandler> {
        let path = path.into();

        let file = handler_try!(File::open(&path));
        let archive = handler_try!(ZipArchive::new(file));

        Ok(ZipHandler {
            path: path,
            archive: archive,
        })
    }
}

impl fmt::Debug for ZipHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ZipHandler")
         .field("path", &self.path)
         .finish()
    }
}

impl Handler for ZipHandler {
    fn stat(&mut self, path: &Path) -> Result<::fs::stat::Stat> {
        use super::utils;

        for index in 0..self.archive.len() {
            let entry = self.archive.by_index(index).unwrap();
            let e_path = Path::new(entry.name());

            if e_path.starts_with(path) {
                // Test the two paths equality (== means it's a file)
                let stat = if e_path == path {
                    ::fs::stat::Stat {
                        file_type: ::fs::stat::FileType::File,
                        permissions: ::fs::stat::Permissions {
                            read: true,
                            write: false,
                        },
                        size: entry.size() as usize,
                    }
                } else {
                    ::fs::stat::Stat {
                        file_type: ::fs::stat::FileType::Directory,
                        permissions: ::fs::stat::Permissions {
                            read: true,
                            write: false,
                        },
                        size: 0,
                    }
                };
                return Ok(stat);
            }
        }

        Err(utils::error(ZipHandlerError::InvalidPath(path.into())))
    }

    fn read_dir(&mut self, path: &Path) -> Result<::fs::dir::ReadDirIterator> {
        use std::collections::HashSet;
        use std::ffi::OsStr;

        let mut items = HashSet::new();

        for index in 0..self.archive.len() {
            let entry = self.archive.by_index(index).unwrap();
            let e_path = Path::new(entry.name());

            if let Ok(rel_path) = e_path.strip_prefix(path) {
                if let Some(name) = rel_path.iter().next().and_then(OsStr::to_str) {
                    items.insert(name.to_owned());
                }
            }
        }

        Ok(Box::new(items.into_iter()))
    }

    fn open(&mut self, path: &Path) -> Result<::fs::file::File<::fs::file::Read>> {
        let name = handler_try!(zip_name(path));

        let entry = handler_try!(self.archive.by_name(name));
        let entry = Box::new(entry);

        Ok(::fs::file::File::new(entry))
    }
}

fn zip_name(path: &Path) -> ::std::result::Result<&str, ZipHandlerError> {
    match path.to_str() {
        Some(name) => Ok(name),
        None => Err(ZipHandlerError::InvalidPath(path.into())),
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ZipHandlerError {
        InvalidPath(path: PathBuf) {
            description("Invalid path (non UTF-8)")
            display("Invalid path `{}`", path.display())
        }
    }
}
