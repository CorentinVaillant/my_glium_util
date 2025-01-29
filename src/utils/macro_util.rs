#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}
#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        ()
    };
}

pub(crate) use debug_println;
