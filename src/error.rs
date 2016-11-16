
#[macro_export]
macro_rules! error_r {
    ($($arg:tt)*) => (
        println!($($arg)*);
        return Err(format!($($arg)*));
    )
}

pub type Result<T> = ::std::result::Result<T, String>;
