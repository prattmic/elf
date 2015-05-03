extern crate elf;

fn main() {
    let elf = elf::Elf::open("/proc/self/exe");
    match elf {
        Ok(_) => println!("Successfully opened ELF file!"),
        Err(e) => println!("Failed to open ELF file: {}", e),
    }
}
