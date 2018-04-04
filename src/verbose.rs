macro_rules! verbose_log {
    ($v:expr, $level:expr, $fmt:expr) => {
        if $level <= $v {
            println!($fmt);
        }
    };
    ($v:expr, $level:expr, $fmt:expr, $($arg:tt)*) => {
        if $level <= $v {
            println!($fmt, $($arg)*);
        }
    };
}

macro_rules! verbose_elog {
    ($v:expr, $level:expr, $fmt:expr) => {
        if $level <= $v {
            eprintln!($fmt);
        }
    };
    ($v:expr, $level:expr, $fmt:expr, $($arg:tt)*) => {
        if $level <= $v {
            eprintln!($fmt, $($arg)*);
        }
    };
}

macro_rules! v1 {
    ($level:expr, $fmt:expr) => {
        verbose_log!(1, $level, $fmt);
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        verbose_log!(1, $level, $fmt, $($arg)*);
    };
}

macro_rules! v2 {
    ($level:expr, $fmt:expr) => {
        verbose_log!(2, $level, $fmt);
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        verbose_log!(2, $level, $fmt, $($arg)*);
    };
}

macro_rules! v3 {
    ($level:expr, $fmt:expr) => {
        verbose_log!(3, $level, $fmt);
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        verbose_log!(3, $level, $fmt, $($arg)*);
    };
}

macro_rules! ve1 {
    ($level:expr, $fmt:expr) => {
        verbose_elog!(1, $level, $fmt);
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        verbose_elog!(1, $level, $fmt, $($arg)*);
    };
}

// macro_rules! ve2 {
//     ($level:expr, $fmt:expr) => {
//         verbose_elog!(3, $level, $fmt);
//     };
//     ($level:expr, $fmt:expr, $($arg:tt)*) => {
//         verbose_elog!(2, $level, $fmt, $($arg)*);
//     };
// }

// macro_rules! ve3 {
//     ($level:expr, $fmt:expr) => {
//         verbose_elog!(3, $level, $fmt);
//     };
//     ($level:expr, $fmt:expr, $($arg:tt)*) => {
//         verbose_elog!(3, $level, $fmt, $($arg)*);
//     };
// }
