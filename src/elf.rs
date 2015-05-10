use std::convert::AsRef;
use std::fs::File;
use std::io;
use std::path::Path;

use pread::Pread;

const ELF_MAGIC: u32 = 0x464c457f;  // .ELF

pub struct Elf {
    file: File,
}

impl Elf {
    pub fn open<F: AsRef<Path>>(filename: F) -> io::Result<Elf> {
        let f = try!(File::open(filename));
        let mut elf = Elf{file: f};

        let valid = try!(elf.check_magic());
        if !valid {
            return Err(io::Error::new(io::ErrorKind::Other, "Not an ELF file"));
        }

        Ok(elf)
    }

    fn check_magic(&mut self) -> io::Result<bool> {
        let mut buf = [0u8; 4];

        // Read the first 4 bytes
        let size = try!(self.file.pread(&mut buf, 0));
        if size != 4 {
            return Err(io::Error::new(io::ErrorKind::Other, "Unable to read 4 bytes"));
        }

        let magic = (buf[0] as u32) |
                    ((buf[1] as u32) << 8) |
                    ((buf[2] as u32) << 16) |
                    ((buf[3] as u32) << 24);

        Ok(magic == ELF_MAGIC)
    }
}

#[cfg(test)]
mod test {
    #[macro_use]
    mod macros;

    use super::*;

    #[test]
    fn open_elf() {
        check!(Elf::open("/proc/self/exe"));
    }

    #[test]
    fn open_non_elf() {
        if let Ok(_) = Elf::open("/proc/self/maps") {
            panic!("Non-ELF opened successfully!");
        }
    }
}
