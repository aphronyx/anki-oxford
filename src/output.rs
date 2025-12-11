use std::{
    fs::File,
    io::{Result, Stdout, Write},
};

pub enum Output {
    Stdout(Stdout),
    File(File),
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match *self {
            Self::Stdout(ref mut stdout) => stdout.write(buf),
            Self::File(ref mut file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match *self {
            Self::Stdout(ref mut stdout) => stdout.flush(),
            Self::File(ref mut file) => file.flush(),
        }
    }
}
