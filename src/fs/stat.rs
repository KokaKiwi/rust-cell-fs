
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
}

impl Permissions {
    pub fn readonly(&self) -> bool {
        self.read && !self.write
    }

    pub fn writeonly(&self) -> bool {
        self.write && !self.read
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stat {
    pub file_type: FileType,
    pub permissions: Permissions,
    pub size: usize,
}
