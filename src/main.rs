use std::env;
use std::path::Path;
use std::ffi::OsStr;
use time::Timespec;
use std::io;
use log::info;
use simple_logger;
use log;

mod drakey;
mod drakey_logger;
use libc::{ENOENT, ENOSYS};
use fuse:: {
    Request,
    Filesystem,
    ReplyAttr,
    ReplyCreate,
    ReplyOpen,
    ReplyEntry,
    FileAttr,
    FileType,
    ReplyDirectory
};

struct FileSys {
    ctr: Counter
}

struct Counter {
    val: u32
}

impl Counter {
    pub fn inc(&mut self) {
        self.val += 1;
    }

    pub fn new() -> Counter {
        return Counter { val: 0 };
    }
}

impl Filesystem for FileSys {
    fn init(&mut self, req: &Request) -> Result<(), i32> {
        println!("init happening");
        Ok(())
    }
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        self.ctr.inc();
        log::warn!("getattr ino={}, ctr={}", ino, self.ctr.val);

        let ts = Timespec::new(0, 0);

        let attr = FileAttr {
            ino,
            size: 0,
            blocks: 0,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::Directory,
            perm: 0o777,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0
        };

        let ttl = Timespec::new(1, 0);

        match ino {
            8 => reply.error(ENOENT),
            _ => reply.attr(&ttl, &attr)
        }
        
    }

    fn open(&mut self, req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        log::warn!("opened! req={:?} ino={}, flags={}", req, ino, flags);
        reply.opened(333, 5);
    }

    // parent here is a given inode
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        self.ctr.inc();
        log::warn!("lookup: parent={}, name={:?} ctr={}", parent, name, self.ctr.val);
        let ttl = Timespec::new(1, 0);
        let ts = time::now().to_timespec();
        let test = name == "thingy";
        let kind = if test {
            FileType::Directory
        } else {
            FileType::RegularFile
        };

        let ino = if test {
            println!("was true");
            3
        } else {
            println!("was false");
            5
        };

        println!("{}", ino);
        let attr = FileAttr {
            ino,
            size: 0,
            blocks: 0,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind,
            perm: 0o777,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0
        };

        match name.to_str().unwrap() {
            "thingss" => reply.error(ENOENT),
            _ => reply.entry(&ttl, &attr, 0)
        }
    }

    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate) {

        log::warn!("created: parent={}, name={:#?}, mode={}, flags={}", parent, name, mode, flags);
        let ts = Timespec::new(0, 0);
        let ttl = time::now().to_timespec();
        let attr = FileAttr {
            ino: 8,
            size: 10,
            blocks: 10,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::RegularFile,
            perm: 0o777,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0
        };

        reply.created(&ttl, &attr, 4, 33, 0);
    }


    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        self.ctr.inc();
        log::warn!("readdir! ino={}, fh={}, offset={} ctr={}", ino, fh, offset, self.ctr.val);

        if offset == 0 {
            reply.add(1, 0, FileType::Directory, &Path::new("."));
            reply.add(1, 1, FileType::Directory, &Path::new(".."));
            reply.add(2, 2, FileType::RegularFile, &Path::new("naughty_things"));
            reply.add(3, 3, FileType::Directory, &Path::new("thingy"));
            reply.ok();
        } else if offset == 1 {
reply.add(1, 1, FileType::Directory, &Path::new(".."));
            reply.add(1, 1, FileType::Directory, &Path::new(".."));
            reply.add(2, 2, FileType::RegularFile, &Path::new("naughty_things"));
            reply.add(3, 3, FileType::Directory, &Path::new("thingy"));
            reply.ok();

        } else if offset == 2 {
            reply.add(2, 2, FileType::RegularFile, &Path::new("naughty_things"));
            reply.add(3, 3, FileType::Directory, &Path::new("thingy"));
            reply.ok();
        } else if offset == 3 {

            //reply.add(3, 3, FileType::Directory, &Path::new("thingy"));
            reply.ok();
        }
    }
}
fn main() {
    drakey_logger::init();
    let mnt = match env::args().nth(1) {
        Some(path) => path,
        None => String::from("/tmp/ftest_sys")
    };

    unsafe {

        let fs = FileSys { ctr: Counter::new() };
    let sys = fuse::spawn_mount(fs, &mnt, &[]).unwrap();
    let mut input = String::new();

    log::warn!("{:?}", sys);
    io::stdin().read_line(&mut input)
        .expect("invalid input");

    }
    info!("hey there");
    println!("all done!");

}
