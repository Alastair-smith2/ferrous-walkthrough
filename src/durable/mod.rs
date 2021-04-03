use std::fs::File;
use std::io::prelude::*;
// std::fs::File type has a sync_all method which ensures that all data is written to disk. This method is not called by default: syncing is slow and OS has good heuristics for optimistically delaying syncing.

// In this assignment, we’ll implement a DurableFile wrapper for File, which helps to ensure that applications calls sync_all. Specifically, DurableFile tracks syncs and writes.
// If a DurableFile is dropped with an outstanding write, its Drop panics. Not calling sync_all before disposing the file is considered a bug in this case. Panic helps to elevate silent potential data loss into a loud failure.

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub struct DurableFile {
    file: File,
    needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile {
            file,
            needs_sync: true,
        }
    }

    pub fn close(&mut self) -> Result<()> {
        self.flush()
    }
}

impl std::io::Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.needs_sync = true;
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.sync_all()?;
        self.file.flush()?;
        self.needs_sync = false;
        Ok(())
    }
}

impl std::ops::Drop for DurableFile {
    fn drop(&mut self) {
        if self.needs_sync {
            panic!("File needed syncing")
        }
    }
}

impl From<File> for DurableFile {
    fn from(file: File) -> Self {
        DurableFile {
            file,
            needs_sync: true,
        }
    }
}

#[test]
#[should_panic(expected = "File needed syncing")]
fn smoke_test() {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = File::create(dir.path().join("foo.txt")).unwrap();
    let _durable_file = DurableFile::new(file);
}

#[test]
fn should_work() -> Result<()> {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = File::create(dir.path().join("foo.txt")).unwrap();
    let mut durable_file = DurableFile::new(file);
    durable_file.write(b"Hello, world!")?;
    durable_file.close()?;
    Ok(())
}

#[test]
fn should_work_with_convert() -> Result<()> {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file = File::create(dir.path().join("foo.txt")).unwrap();
    let mut durable_file = DurableFile::from(file);
    durable_file.write(b"Hello, world!")?;
    durable_file.close()?;
    Ok(())
}
