use std::convert::AsRef;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn first_word<F: AsRef<Path>>(file: F) -> io::Result<u32> {
    let mut f = try!(File::open(file));
    let mut buf: [u8; 4] = [0, 0, 0, 0];

    // Read the first 4 bytes
    let size = try!(f.read(&mut buf));
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

    #[test]
    fn detect_elf() {
        let word = first_word("/proc/self/exe");

        match word {
            Ok(w) => assert_eq!(0x464c457f, w), // .ELF
            Err(e) => panic!("failed with: {}", e),
        }
    }
}
