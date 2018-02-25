use std::ascii::AsciiExt;
use std::error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Ascii(Vec<u8>);

#[derive(Debug, Eq, PartialEq)]
pub struct NotAsciiError(pub Vec<u8>);

impl error::Error for NotAsciiError {
    fn description(&self) -> &str {
        "not ascii string"
    }

    fn cause(&self) -> Option<&error::Error> { None }
}

impl fmt::Display for NotAsciiError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(<Self as error::Error>::description(self))
    }
}

impl Ascii {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
    }

    pub unsafe fn from_bytes_unchecked(bytes:Vec<u8>) -> Ascii{
        Ascii(bytes)
    }
}

impl From<Ascii> for String {
    fn from(ascii:Ascii) -> String {
        // If this module has no bugs,this is safe,because
        // well-formed ASCII text is also well-formed UTF-8.
        unsafe {String::from_utf8_unchecked(ascii.0)}

    }
}

fn main() {
    let bytes = b"ASCII and ya shall receive".to_vec();
    let ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);

    assert_eq!(string,"ASCII and ya shall receive");
}