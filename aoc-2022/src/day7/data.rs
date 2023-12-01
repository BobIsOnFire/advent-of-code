use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
};

use aoc_common::util::iter::IteratorExtended;

type Inode = usize;

pub struct PlainFile {
    size: usize,
}

impl PlainFile {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

pub struct Directory {
    // This is just a 'cache' value for disk usage,
    last_disk_usage: RefCell<usize>,
    contents: HashMap<String, Inode>,
    parent: Inode,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            last_disk_usage: RefCell::new(0),
            contents: HashMap::new(),
            parent: 0,
        }
    }

    pub fn set_parent(&mut self, parent: Inode) {
        self.parent = parent
    }

    pub fn add_file(&mut self, name: String, file: Inode) -> Option<()> {
        if let Entry::Vacant(e) = self.contents.entry(name) {
            e.insert(file);
            Some(())
        } else {
            None
        }
    }

    pub fn find_file(&self, name: &String) -> Option<Inode> {
        self.contents.get(name).copied()
    }

    pub fn get_parent(&self) -> Inode {
        self.parent
    }

    pub fn list_directory(&self) -> impl Iterator<Item = Inode> + '_ {
        self.contents.values().copied()
    }

    pub fn get_last_disk_usage(&self) -> usize {
        *self.last_disk_usage.borrow()
    }

    pub fn set_last_disk_usage(&self, usage: usize) {
        self.last_disk_usage.replace(usage);
    }
}

pub enum File {
    PlainFile(PlainFile),
    Directory(Directory),
}

impl File {
    pub fn plain(size: usize) -> Self {
        Self::PlainFile(PlainFile::new(size))
    }

    pub fn directory() -> Self {
        Self::Directory(Directory::new())
    }

    pub fn as_directory(&self) -> Option<&Directory> {
        match self {
            Self::PlainFile(_) => None,
            Self::Directory(dir) => Some(dir),
        }
    }

    pub fn as_directory_mut(&mut self) -> Option<&mut Directory> {
        match self {
            Self::PlainFile(_) => None,
            Self::Directory(dir) => Some(dir),
        }
    }
}

pub struct FileSystem {
    inodes: Vec<File>,
    current_inode: Inode,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = File::directory();
        Self {
            inodes: vec![root],
            current_inode: Self::root(),
        }
    }

    pub fn root() -> Inode {
        0
    }

    pub fn cd_root(&mut self) {
        self.change_directory(Self::root());
    }

    pub fn cd_up(&mut self) {
        self.change_directory(self.cwd().get_parent());
    }

    pub fn cd_down(&mut self, name: &String) -> Option<()> {
        let inode = self.cwd().find_file(name)?;
        self.get_file(inode).as_directory()?;

        self.change_directory(inode);
        Some(())
    }

    pub fn create_file(&mut self, name: String, file: File) -> Option<()> {
        let inode = self.create_node(file);
        self.cwd_mut().add_file(name, inode)?;

        let current_inode = self.current_inode;
        if let Some(dir) = self.get_file_mut(inode).as_directory_mut() {
            dir.set_parent(current_inode);
        }

        Some(())
    }

    pub fn get_total_disk_usage(&self) -> usize {
        self.do_disk_usage_recursive(Self::root())
    }

    pub fn walk_directory<'a>(&'a self, dir: &'a Directory) -> impl Iterator<Item = &File> + 'a {
        dir.list_directory().map(|i| self.get_file(i))
    }

    pub fn walk_filesystem(&self) -> impl Iterator<Item = (usize, &File)> {
        let once = std::iter::once(self.get_file(Self::root()));

        once.recursive(|el| el.as_directory().map(|dir| self.walk_directory(dir)))
    }

    fn get_file(&self, inode: Inode) -> &File {
        self.inodes
            .get(inode)
            .expect("inode should exist in file system")
    }

    fn get_file_mut(&mut self, inode: Inode) -> &mut File {
        self.inodes
            .get_mut(inode)
            .expect("inode should exist in file system")
    }

    fn cwd(&self) -> &Directory {
        self.get_file(self.current_inode)
            .as_directory()
            .expect("current_inode should be a directory")
    }

    fn cwd_mut(&mut self) -> &mut Directory {
        self.get_file_mut(self.current_inode)
            .as_directory_mut()
            .expect("current_inode should be a directory")
    }

    fn change_directory(&mut self, inode: Inode) {
        self.current_inode = inode;
    }

    fn create_node(&mut self, file: File) -> Inode {
        self.inodes.push(file);
        self.inodes.len() - 1
    }

    fn do_disk_usage_recursive(&self, inode: Inode) -> usize {
        match self.get_file(inode) {
            File::PlainFile(file) => file.size,
            File::Directory(dir) => {
                let size = dir
                    .list_directory()
                    .map(|ch| self.do_disk_usage_recursive(ch))
                    .sum();
                dir.set_last_disk_usage(size);
                size
            }
        }
    }
}
