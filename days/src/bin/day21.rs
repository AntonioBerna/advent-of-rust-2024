use std::env::temp_dir;
use std::fs::File;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.path());
    }
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        for n in 0.. {
            let file_path = temp_dir().join(format!("/tmp/.tmp{}", n));
            if let Ok(file) = File::create_new(&file_path) {
                return Ok(Self {
                    file_path,
                    file,
                })
            }
        }
        Err(std::io::Error::last_os_error())
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write(data)?;
        Ok(())
    }
    
    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
      let mut buffer = String::new();
      file.read_to_string(&mut buffer)?;
      Ok(buffer)
    }
    
    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
    
    pub fn file(&self) -> &File {
        &self.file
    }
}