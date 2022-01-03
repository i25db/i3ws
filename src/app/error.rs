#[macro_export()]
macro_rules! check_error {
    ($res:expr, $msg:expr) => {{
        match $res {
            Ok(t) => t,
            Err(e) => {
                eprintln!($msg, e);
                std::process::exit(1);
            }
        }
    }};
}
