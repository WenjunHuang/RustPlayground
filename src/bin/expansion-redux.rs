//@formatter:off
macro_rules! capture_then_match_tokens{
    ($e:expr) => {match_tokens!($e)};
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {"got an addition"};
    (($i:ident)) => {"got an identifier"};
    ($($other:tt)*) => {"got something else"};
}

macro_rules! capture_then_what_is {
(#[$m:meta]) => {what_is!(#[$m])};
}
macro_rules! what_is {
(#[no_mangle]) => {"no_mangle attribute"};
(#[inline]) => {"inline attribute"};
($($tts:tt)*) => {concat!("something else (",stringify!($(tts)*)),")"}
}
//@formatter:on

fn main() {
    println!("{}\n{}\n{}\n",
             match_tokens!((caravan)),
             match_tokens!(3+6),
             match_tokens!(5));

    println!("{}\n{}\n{}",
             capture_then_match_tokens!((caravan)),
             capture_then_match_tokens!(3+6),
             capture_then_match_tokens!(5));
}