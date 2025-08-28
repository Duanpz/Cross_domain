use fuser::{FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyEmpty, ReplyEntry, ReplyOpen, Request, TimeOrNow};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct FuseFileSystem {
    files: Arc<parking_lot::Mutex<HashMap<String, Vec<u8>>>>,
    next_ino: u64,
}

impl FuseFileSystem {
    pub fn new() -> Self {
        Self {
            files: Arc::new(parking_lot::Mutex::new(HashMap::new())),
            next_ino: 1,
        }
    }

    fn next_inode(&mut self) -> u64 {
        let inode = self.next_ino;
        self.next_ino += 1;
        inode
    }
}

impl Filesystem for FuseFileSystem {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let name_str = name.to_string_lossy().to_string();
        let path = format!("/{}/{}", parent, name_str);
        
        let files = self.files.lock();
        if files.contains_key(&path) {
            let attr = FileAttr {
                ino: self.next_inode(),
                size: files.get(&path).map(|v| v.len() as u64).unwrap_or(0),
                blocks: 0,
                atime: SystemTime::now(),
                mtime: SystemTime::now(),
                ctime: SystemTime::now(),
                crtime: SystemTime::now(),
                kind: FileType::RegularFile,
                perm: 0o644,
                nlink: 1,
                uid: 0,
                gid: 0,
                rdev: 0,
                blksize: 512,
                padding: 0,
            };
            reply.entry(&Duration::from_secs(1), &attr, 0);
        } else {
            reply.error(libc::ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let files = self.files.lock();
        let attr = FileAttr {
            ino,
            size: files.get(&format!("/{}", ino)).map(|v| v.len() as u64).unwrap_or(0),
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 512,
            padding: 0,
        };
        reply.attr(&Duration::from_secs(1), &attr);
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, size: u32, reply: ReplyData) {
        let files = self.files.lock();
        let data = files.get(&format!("/{}", ino));
        match data {
            Some(d) => {
                let start = offset as usize;
                let end = (offset + size as i64) as usize;
                let data_slice = &d[start..end.min(d.len())];
                reply.data(data_slice);
            },
            None => reply.error(libc::ENOENT),
        }
    }

    fn write(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, data: &[u8], _flags: u32, reply: ReplyWrite) {
        let mut files = self.files.lock();
        let path = format!("/{}", ino);
        let existing_data = files.get_mut(&path).unwrap_or_else(|| {
            files.insert(path.clone(), Vec::new());
            files.get_mut(&path).unwrap()
        });
        
        let offset_usize = offset as usize;
        if offset_usize >= existing_data.len() {
            existing_data.resize(offset_usize + data.len(), 0);
        }
        
        existing_data[offset_usize..offset_usize + data.len()].copy_from_slice(data);
        reply.written(data.len() as u32);
    }

    fn open(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        reply.opened(0, 0);
    }

    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, _flags: u32, reply: ReplyCreate) {
        let name_str = name.to_string_lossy().to_string();
        let path = format!("/{}/{}", parent, name_str);
        
        let mut files = self.files.lock();
        files.insert(path.clone(), Vec::new());
        
        let attr = FileAttr {
            ino: self.next_inode(),
            size: 0,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 512,
            padding: 0,
        };
        reply.created(&Duration::from_secs(1), &attr, 0, 0, 0);
    }

    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name_str = name.to_string_lossy().to_string();
        let path = format!("/{}/{}", parent, name_str);
        let mut files = self.files.lock();
        files.remove(&path);
        reply.ok();
    }
}