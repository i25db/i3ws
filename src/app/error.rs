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

#[macro_export()]
macro_rules! safe_panic {
    () => {{
        {
            eprintln!();
            std::process::exit(1);
        }
    }};
    ($msg:expr) => {{
        {
            eprintln!($msg);
            std::process::exit(1);
        }
    }};
    ($($exprs:expr),+) => {{
        {
            eprintln!($($exprs),+);
            std::process::exit(1);
        }
    }};
}

#[macro_export()]
macro_rules! check_some {
    ($res:expr, $($msg:expr),+) => {
        match $res {
            Some(v) => v,
            None => {
                eprintln!($($msg),+);
                std::process::exit(1);
            }
        }
    };
    ($res:expr, $def:path) => {
        match $res {
            Some(v) => v,
            None => $def(),
        }
    };
}
