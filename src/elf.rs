use std::convert::AsRef;
use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

use pread::Pread;

#[derive(Copy, Clone, Debug)]
pub enum Class {
    Class32,
    Class64,
    Unknown,
}

impl Default for Class {
    fn default() -> Class { Class::Unknown }
}

#[derive(Copy, Clone, Debug)]
pub enum Endianness {
    Little,
    Big,
    Unknown,
}

impl Default for Endianness {
    fn default() -> Endianness { Endianness::Unknown }
}

#[derive(Copy, Clone, Debug)]
pub enum OsABI {
    None,
    Linux,
    Unknown,
}

impl Default for OsABI {
    fn default() -> OsABI { OsABI::Unknown }
}

#[derive(Default)]
struct ElfIdent {
    mag0: u8,
    mag1: u8,
    mag2: u8,
    mag3: u8,
    class: Class,
    endianness: Endianness,
    version: u8,
    osabi: OsABI,
    abiversion: u8,
}

pub struct Elf {
    file: File,

    ident: ElfIdent,
}

impl Elf {
    pub fn open<F: AsRef<Path>>(filename: F) -> io::Result<Elf> {
        let f = try!(File::open(filename));
        let mut elf = Elf{
            file: f,
            ident: Default::default(),
        };

        elf.ident = try!(elf.read_ident());

        if !elf.check_magic() {
            return Err(io::Error::new(io::ErrorKind::Other, "Not an ELF file"));
        }

        Ok(elf)
    }

    fn read_ident(&mut self) -> io::Result<ElfIdent> {
        // ELF_IDENT is the first 16 bytes of the file
        let mut buf = [0u8; 16];

        let size = try!(self.file.pread(&mut buf, 0));
        if size != 16 {
            return Err(
                io::Error::new(io::ErrorKind::Other,
                               fmt::format(format_args!("Read {} bytes, expected 16", size))));
        }

        let class = match buf[4] {
            1 => Class::Class32,
            2 => Class::Class64,
            _ => Class::Unknown,
        };

        let endianness = match buf[5] {
            1 => Endianness::Little,
            2 => Endianness::Big,
            _ => Endianness::Unknown,
        };

        let os = match buf[7] {
            0 => OsABI::None,
            3 => OsABI::Linux,
            _ => OsABI::Unknown,
        };

        Ok(ElfIdent{
            mag0: buf[0],
            mag1: buf[1],
            mag2: buf[2],
            mag3: buf[3],
            class: class,
            endianness: endianness,
            version: buf[6],
            osabi: os,
            abiversion: buf[8]
        })
    }

    fn check_magic(&mut self) -> bool {
        self.ident.mag0 == 0x7f &&
            self.ident.mag1 == ('E' as u8) &&
            self.ident.mag2 == ('L' as u8) &&
            self.ident.mag3 == ('F' as u8)
    }

    pub fn version(&self) -> u8 {
        self.ident.version
    }

    pub fn class(&self) -> Class {
        self.ident.class
    }

    pub fn endianness(&self) -> Endianness {
        self.ident.endianness
    }

    pub fn osabi(&self) -> OsABI {
        self.ident.osabi
    }

    pub fn abi_version(&self) -> u8 {
        self.ident.abiversion
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
