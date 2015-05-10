// Borrowed from https://github.com/rust-lang/rust/blob/master/src/libstd/fs.rs
macro_rules! check { ($e:expr) => (
    match $e {
        Ok(t) => t,
        Err(e) => panic!("{} failed with: {}", stringify!($e), e),
    }
) }
