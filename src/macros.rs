#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {};
}

macro_rules! some_or_return {
    ( $a:expr $(,$b:expr)? ) => {
        match $a {
            Some(x) => x,
            None => return $($b)?,
        }
    };
}

macro_rules! ok_or_return {
    ( $a:expr $(,$b:expr)? ) => {
        match $a {
            Ok(x) => x,
            Err(_) => return $($b)?,
        }
    };
}

macro_rules! own {
    ($( $var:ident ),*) => {
        $(
        let $var = $var.to_owned();
        )*
    };
}

pub(crate) use debug_print;
pub(crate) use ok_or_return;
pub(crate) use own;
pub(crate) use some_or_return;
