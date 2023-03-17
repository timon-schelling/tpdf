#[macro_export]
macro_rules! timpl_str {
    ($body:expr) => {
        ::std::fmt::format(format_args!("\"{}\"", $body))
    };
}

#[macro_export]
macro_rules! timpl_if {
    ($bool:expr, $body:tt) => {
        if $bool {
            timpl_proc::timpl!$body
        } else {
            "".to_string()
        }
    };
}

#[macro_export]
macro_rules! timpl_if_else {
    ($bool:expr, $body:tt, $else:tt) => {
        if $bool {
            timpl_proc::timpl!$body
        } else {
            timpl_proc::timpl!$else
        }
    };
}

#[macro_export]
macro_rules! timpl_map {
    ($items:expr, $ident:ident, $body:tt) => {
        ($items).map(|$ident| {
            timpl_proc::timpl!$body
        }).collect::<String>()
    };
}

#[macro_export]
macro_rules! timpl_map_ln {
    ($items:expr, $ident:ident, $body:tt) => {
        ($items).map(|$ident| {
            timpl_proc::timpl!$body
        }).collect::<Vec<String>>().join("\n")
    };
}
