use std::convert::AsRef;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

const ELF_MAGIC: u32 = 0x464c457f;  // .ELF

pub struct Elf {
    file: File,
}

impl Elf {
    pub fn open<F: AsRef<Path>>(filename: F) -> io::Result<Elf> {
        let mut f = try!(File::open(filename));
        let first_word = try!(first_word(&mut f));

        if first_word != ELF_MAGIC {
            return Err(io::Error::new(io::ErrorKind::Other, "Not an ELF file"));
        }

        Ok(Elf{file: f})
    }
}

fn first_word(file: &mut File) -> io::Result<u32> {
    let mut buf: [u8; 4] = [0, 0, 0, 0];

    // Read the first 4 bytes
    let size = try!(file.read(&mut buf));
    if size != 4 {
        return Err(io::Error::new(io::ErrorKind::Other, "Unable to read 4 bytes"));
    }

    Ok((buf[0] as u32) |
       ((buf[1] as u32) << 8) |
       ((buf[2] as u32) << 16) |
       ((buf[3] as u32) << 24))
}

#[cfg(test)]
mod test {
    use super::*;

    // Borrowed from https://github.com/rust-lang/rust/blob/master/src/libstd/fs.rs
    macro_rules! check { ($e:expr) => (
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with: {}", stringify!($e), e),
        }
    ) }

    #[test]
    fn open_elf() {
        check!(Elf::open("/proc/self/exe"));
    }
}
