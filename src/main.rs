extern crate elf;

fn main() {
    let elf = match elf::Elf::open("/proc/self/exe") {
        Ok(e) => e,
        Err(e) => panic!("Failed to open ELF file: {}", e),
    };

    println!("ELF Version: {}", elf.version());
    println!("ELF Class: {:?}", elf.class());
    println!("ELF Endianness: {:?}", elf.endianness());
    println!("ELF OS ABI: {:?}", elf.osabi());
    println!("ELF ABI Version: {}", elf.abi_version());
}
