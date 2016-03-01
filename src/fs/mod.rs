use error::{Error, Result};
use handlers::Handler;
use uuid::Uuid;
use std::path::{Path, PathBuf};
use std::slice::IterMut;

pub mod dir;
pub mod file;
pub mod stat;

#[derive(Debug)]
pub struct CellFs {
    mount_points: Vec<MountPoint>,
}

impl CellFs {
    pub fn new() -> CellFs {
        CellFs {
            mount_points: vec![],
        }
    }

    pub fn mount<P: Into<PathBuf>>(&mut self, path: P, handler: Box<Handler>) -> Uuid {
        let mount_point = MountPoint::new(path, handler);
        let id = mount_point.id();
        self.mount_points.push(mount_point);

        id
    }

    pub fn unmount(&mut self, id: Uuid) {
        self.mount_points.retain(|mount_point| mount_point.id() != id);
    }

    fn search<P: Into<PathBuf>>(&mut self, path: P, exists: bool) -> Search {
        Search::new(path, self, exists)
    }

    pub fn exists<P: AsRef<Path>>(&mut self, path: P) -> bool {
        let path = path.as_ref();
        self.search(path, true).count() > 0
    }

    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> Vec<dir::DirEntry> {
        use itertools::Itertools;
        use std::hash::{Hash, SipHasher, Hasher};

        fn hash<T: Hash>(t: &T) -> u64 {
            let mut s = SipHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let path = path.as_ref();
        self.search(path, true).rev()
            .filter_map(|(mount_point, path)| mount_point.read_dir(path).ok())
            .flat_map(|iter| iter)
            .unique_by(hash)
            .collect()
    }

    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<file::File<file::Read>> {
        let path = path.as_ref();
        match self.search(path, true).last() {
            Some((mount_point, path)) => mount_point.open(path),
            None => Err(Error::NotFound(path.into())),
        }
    }
}

#[derive(Debug)]
pub struct MountPoint {
    id: Uuid,
    path: PathBuf,
    handler: Box<Handler>,
}

impl MountPoint {
    fn new<P: Into<PathBuf>>(path: P, handler: Box<Handler>) -> MountPoint {
        MountPoint {
            id: Uuid::new_v4(),
            path: path.into(),
            handler: handler,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn handler(&mut self) -> &mut Handler {
        &mut *self.handler
    }

    fn exists<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.handler().exists(path.as_ref())
    }

    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<dir::ReadDir> {
        let path = path.as_ref();
        self.handler().read_dir(path).map(|iter| dir::ReadDir::new(path, iter))
    }

    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<file::File<file::Read>> {
        self.handler().open(path.as_ref())
    }
}

struct Search<'a> {
    path: PathBuf,
    iter: IterMut<'a, MountPoint>,
    exists: bool,
}

impl<'a> Search<'a> {
    fn new<P: Into<PathBuf>>(path: P, fs: &mut CellFs, exists: bool) -> Search {
        Search {
            path: path.into(),
            iter: fs.mount_points.iter_mut(),
            exists: exists,
        }
    }

    fn map_item(&self, mount_point: &'a mut MountPoint) -> Option<(&'a mut MountPoint, PathBuf)> {
        self.path.strip_prefix(&mount_point.path.clone()).ok()
            .and_then(|path| {
                if self.exists && !mount_point.exists(&path) {
                    return None;
                }

                Some((mount_point, path.into()))
            })
    }
}

impl<'a> Iterator for Search<'a> {
    type Item = (&'a mut MountPoint, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mount_point) = self.iter.next() {
            if let Some(item) = self.map_item(mount_point) {
                return Some(item);
            }
        }

        None
    }
}

impl<'a> DoubleEndedIterator for Search<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(mount_point) = self.iter.next_back() {
            if let Some(item) = self.map_item(mount_point) {
                return Some(item);
            }
        }

        None
    }
}
