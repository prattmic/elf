use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

pub trait Pread {
    fn pread(&mut self, buf: &mut [u8], offset: u64) -> Result<usize>;
}

impl Pread for File {
    // A smart implementation would actually use pread(2)...
    // Warning: File offset changed after call!
    fn pread(&mut self, buf: &mut [u8], offset: u64) -> Result<usize> {
        if let Err(e) = self.seek(SeekFrom::Start(offset)) {
            return Err(e);
        }

        self.read(buf)
    }
}

#[cfg(test)]
mod test {
    #[macro_use]
    mod macros;

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::str;

    #[test]
    fn read_offset() {
        // TODO: Better tempfile solution (mkstemp?)
        let mut file = check!(File::create("/tmp/read_offset_test.txt"));

        let hello = b"Hello World!";
        let size = check!(file.write(hello));
        if size != 12 {
            panic!("Did not write entire buffer.  Wrote {} bytes.", size);
        }

        // Re-open for reading
        let mut file = check!(File::open("/tmp/read_offset_test.txt"));

        // We should read "World!"
        let mut buf = [0u8; 6];
        let read = check!(file.pread(&mut buf, 6));
        if read != 6 {
            panic!("Did not read expected length.  Read {} bytes.", read);
        }

        let data = check!(str::from_utf8(&buf));
        if data != "World!" {
            panic!("Unexpected buffer contents: {}", data);
        }

        // TODO: delete the temp file...
    }
}
