use std::path;
use std::thread;

#[derive(Debug)]
pub struct IOWorkerObserver {
    pub handle: thread::JoinHandle<()>,
    msg: String,
    src: path::PathBuf,
    dest: path::PathBuf,
}

impl IOWorkerObserver {
    pub fn new(handle: thread::JoinHandle<()>, src: path::PathBuf, dest: path::PathBuf) -> Self {
        Self {
            handle,
            src,
            dest,
            msg: String::new(),
        }
    }

    pub fn join(self) {
        self.handle.join();
    }
    pub fn set_msg(&mut self, msg: String) {
        self.msg = msg
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_str()
    }
    pub fn get_src_path(&self) -> &path::Path {
        self.src.as_path()
    }
    pub fn get_dest_path(&self) -> &path::Path {
        self.dest.as_path()
    }
}
