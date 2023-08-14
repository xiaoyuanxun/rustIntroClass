macro_rules! custom_log {
    (debug, $($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}", format_args!($($arg)*));
        }
    };
    (info, $($arg:tt)*) => {
        println!("[INFO] {}", format_args!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        eprintln!("[WARN] {}", format_args!($($arg)*));
    };
    (error, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    };
}

fn main() {
    custom_log!(debug, "This is a debug message");
    custom_log!(info, "This is an info message");
    custom_log!(warn, "This is a warning message");
    custom_log!(error, "This is an error message");
}

