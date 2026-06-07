use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub trait Logger {
    fn log(&mut self, msg: &[u8]);
}

pub struct NoOpLogger;

pub struct FileLogger {
    file: BufWriter<File>,
    buffer: Vec<u8>
}

impl FileLogger {
    pub fn open(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self {
            file: BufWriter::new(file),
            buffer: vec![],
        })
    }

    fn _exists(path: &str) -> bool {
        let p = Path::new(path);
        return p.exists() && p.is_file();
    }
}

impl Logger for FileLogger {
    fn log(&mut self, content: &[u8]) {

        // receive a part of string, e.g. "hello wor"
        self.buffer.extend_from_slice(content);

        // if end of string is received e.g. "ld\r\n"
        while let Some(pos) = self.buffer.iter().position(|&x| x == b'\n' ) {

            // refine the whole string, e.g. "hello world\r\n"
            let mut line = self.buffer.drain(..=pos).collect::<Vec<u8>>();

            // if lasts with \r\n
            while matches!(line.last(), Some( b'\r' | b'\n') )
            {
                // delete last
                line.pop();
            }

            let _ = self.file.write_all(&line);
            let _ = self.file.write_all(b"\n");
            let _ = self.file.flush();
        }
    }
}

impl Logger for NoOpLogger {
    fn log(&mut self, _content: &[u8]) {
        // do nothing
    }
}

impl Drop for FileLogger {
    fn drop(&mut self) {
        let _ = self.file.flush();
    }
}