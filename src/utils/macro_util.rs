#![allow(unused_macros,unused_imports)]

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



macro_rules! chrono {
    ($func:expr,$($arg:tt)*) => {{
        let now = std::time::Instant::now();
        let result = $func($($arg)*);
        let elapsed_time = now.elapsed();
        println!("test OK ✅, took {}", elapsed_time.as_secs_f64());
        result}
    };
    ($code:block)=>{{
        let now = std::time::Instant::now();
        let result = $code;
        let elapsed_time = now.elapsed();
        println!("test OK ✅, took {}", elapsed_time.as_secs_f64());
        result}
    }
}
pub(crate) use chrono;