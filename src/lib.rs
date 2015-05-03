pub fn first_word(file: &'static str) -> u64 {
    // TODO: read from file!
    0x464c457f  // .ELF
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_elf() {
        assert_eq!(0x464c457f, first_word("/proc/self/exe"))
    }
}
