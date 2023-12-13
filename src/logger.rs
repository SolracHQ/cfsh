/// Macro to log and exit the program
#[macro_export]
macro_rules! log_and_exit {
    ($($arg:tt)*) => {
        {
            eprintln!($($arg)*);
            std::process::exit(1);
        }
    };
}