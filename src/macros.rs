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

pub(crate) use debug_print;
pub(crate) use unwrap_or_return;
