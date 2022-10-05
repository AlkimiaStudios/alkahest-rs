extern crate chrono;

pub fn __private_format_log(level: &str) -> String {
    let time = std::time::SystemTime::now();
    let dt: chrono::DateTime<chrono::offset::Local> = time.into();
    let formatted = dt.format("%Y-%m-%dT%H:%M:%S.%3f");
    let s: String = std::fmt::format(format_args!("{} {} |", formatted, level));
    s
}

#[macro_export]
macro_rules! error {
    () => {};
    ($($arg:tt)*) => {
        println!("{} {}", $crate::util::log::__private_format_log("\x1b[31;1m[ERROR]\x1b[0m"),
            format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    () => {};
    ($($arg:tt)*) => {
        println!("{} {}", $crate::util::log::__private_format_log("\x1b[33;1m[WARN]\x1b[0m "),
            format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    () => {};
    ($($arg:tt)*) => {
        println!("{} {}", $crate::util::log::__private_format_log("\x1b[32;1m[INFO]\x1b[0m "),
            format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    () => {};
    ($($arg:tt)*) => {
        println!("{} {}", $crate::util::log::__private_format_log("\x1b[34;1m[DEBUG]\x1b[0m"),
            format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! trace {
    () => {};
    ($($arg:tt)*) => {
        println!("{} {}", $crate::util::log::__private_format_log("\x1b[37;1m[TRACE]\x1b[0m"),
            format_args!($($arg)*))
    };
}
