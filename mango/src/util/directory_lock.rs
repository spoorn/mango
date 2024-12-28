use bytes::Bytes;
use fs4::fs_std::FileExt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

static DUMMY: Bytes = Bytes::from_static(b"\xE2\x98\x83");

pub struct DirectoryLock {
    lock_file: File,
}
impl DirectoryLock {
    pub fn create(path: PathBuf) -> Self {
        let session_lock = path.join("session.lock");
        std::fs::create_dir_all(&path).expect("Failed to create session lock path");

        match File::create(&session_lock) {
            Ok(mut lock_file) => match lock_file.try_lock_exclusive() {
                Ok(()) => {
                    lock_file
                        .write_all(&DUMMY)
                        .expect("Failed to write to session lock file");
                    Self { lock_file }
                }
                Err(e) => {
                    panic!("Session file at {:?} already locked (possibly by other Minecraft instance?): {:?}", session_lock, e);
                }
            },
            Err(e) => panic!(
                "Failed to create session lock file at {:?}: {:?}",
                session_lock, e
            ),
        }
    }
}
impl Drop for DirectoryLock {
    fn drop(&mut self) {
        self.lock_file
            .unlock()
            .expect("Failed to unlock session lock file");
    }
}
