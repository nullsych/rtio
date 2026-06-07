
use chrono::Local;
use std::io::{self, Write};

pub struct OutputWriter
{
    pub enable : bool,
    new_line : bool
}

impl OutputWriter
{
    pub fn new(enable : bool) -> Self
    {
        Self { enable: (enable), new_line: (true) }
    }

    pub fn write(&mut self, data : &[u8], stdout : &mut dyn Write) -> io::Result<()>
    {
        if !self.enable
        {
            let _ = stdout.write_all(data);
            let _ = stdout.flush();
            return Ok(());
        }
        else {
            for &d in data
            {
                if self.new_line {
                    write!(stdout, "[rtio {}] ", Local::now().format("%H:%M:%S"))?;
                    self.new_line = false;
                }

                stdout.write_all(&[d])?;
                stdout.flush()?;

                if d == b'\n' {
                    self.new_line = true;
                }
            }

            return Ok(());
        }
    }

}