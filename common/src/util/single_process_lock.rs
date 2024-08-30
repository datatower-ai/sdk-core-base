use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use fs2::FileExt;
use crate::util::error::macros::{runtime_error};
use crate::util::error::Result;

#[derive(Debug)]
pub struct SingleProcessLock {
    file: File,
    path: PathBuf,
}

impl SingleProcessLock {
    pub fn new(dir_path: String, prefix: String) -> Result<Self> {
        let path = PathBuf::from(dir_path)
            .join(format!("dt-sdk-{prefix}.lock"));
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path.clone());
        let file = match file {
            Ok(file) => Ok(file),
            Err(e) => runtime_error!("Failed to create/get lock file, {e}")
        }?;

        Ok(SingleProcessLock { file, path })
    }

    pub fn lock(self) -> Result<SingleProcessLocked> {
        if let Err(e) = self.file.try_lock_exclusive() {
            runtime_error!("Failed to lock, the lock might be acquired by other process (only single process is allowed)!, {e}")
        } else {
            Ok(SingleProcessLocked::new(self))
        }
    }

    fn unlock(&self) -> Result<()> {
        if let Err(e) = self.file.unlock() {
            runtime_error!("Failed to unlock, {e}")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct SingleProcessLocked {
    lock: SingleProcessLock
}

impl SingleProcessLocked {
    fn new(lock: SingleProcessLock) -> Self {
        SingleProcessLocked { lock }
    }
}

impl Drop for SingleProcessLocked {
    fn drop(&mut self) {
        self.lock.unlock().expect("Failed to drop/unlock SingleProcessLock");
        std::fs::remove_file(&self.lock.path).unwrap();
    }
}