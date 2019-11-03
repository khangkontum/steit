use std::{fmt, io};

use crate::Serialize;

use super::{
    log::{Entry, EntryKind, Logger},
    path::Path,
};

pub struct Runtime {
    logger: Logger,
    path: Path,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
            path: Path::root(),
        }
    }

    pub fn nested(&self, tag: u16) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.child(tag),
        }
    }

    pub fn parent(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            path: self.path.parent(),
        }
    }

    #[inline]
    pub fn log_update(&self, tag: u16, value: &impl Serialize) -> io::Result<()> {
        self.logger
            .log_entry(Entry::new(&self.path, EntryKind::Update { tag, value }))
    }

    #[inline]
    pub fn log_add(&self, item: &impl Serialize) -> io::Result<()> {
        self.logger
            .log_entry(Entry::new(&self.path, EntryKind::Add { item }))
    }

    #[inline]
    pub fn log_remove<T: Serialize>(&self, tag: u16) -> io::Result<()> {
        self.logger
            .log_entry(Entry::new(&self.path, EntryKind::Remove::<T> { tag }))
    }
}

impl PartialEq for Runtime {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime {{ path: {:?} }}", self.path)
    }
}