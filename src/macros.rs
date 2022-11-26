#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {};
}

macro_rules! unwrap_or_return {
    ( $a:expr $(,$b:expr)? ) => {
        match $a {
            Some(x) => x,
            None => return $($b)?,
        }
    };
}

macro_rules! own {
    ($( $var:ident ),*) => {
        let var = var.to_owned();
    };
}

pub(crate) use debug_print;
pub(crate) use own;
pub(crate) use unwrap_or_return;
