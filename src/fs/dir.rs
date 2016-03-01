use std::marker::PhantomData;
use std::path::PathBuf;

pub type ReadDirIterator = Box<Iterator<Item = String>>;
pub struct ReadDir<'a> {
    path: PathBuf,
    iter: ReadDirIterator,
    _phantom: PhantomData<&'a ReadDirIterator>,
}

impl<'a> ReadDir<'a> {
    pub fn new<P: Into<PathBuf>>(path: P, iter: ReadDirIterator) -> ReadDir {
        ReadDir {
            path: path.into(),
            iter: iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for ReadDir<'a> {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|entry| DirEntry {
            dir: self.path.clone(),
            name: entry,
        })
    }
}

#[derive(Debug, Hash)]
pub struct DirEntry {
    dir: PathBuf,
    name: String,
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.dir.join(self.name())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
