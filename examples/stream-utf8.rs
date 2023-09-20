use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{Read, Write};

use error_iterator::{EIterator, ToEIter};

fn main() -> Result<(), AppError> {
    let mapper: HashMap<char, char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .zip("ДВСDЁҒGНІЈКLМПОРQЯЅТЦЏШХЧZавсdёfgніјкlмпорqгѕтцѵшхчz".chars())
        .collect();
    let convert_char = |c| *mapper.get(&c).unwrap_or(&c);

    let stdin = io::stdin();
    let stdout = io::stdout();

    let input = stdin
        .lock()
        .bytes()
        .eiter()
        .map_error(AppError::IOError)
        .iter()
        .collect::<Vec<_>>();
    // .decode_utf8()
    // .map(convert_char)
    // .encode_utf8()
    // .write_to(stdout.lock())?;
    stdout.lock().write(format!("{:?}", input).as_bytes())?;

    Ok(())
}

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    // DecodeUtf8Error(DecodeUtf8Error),
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}
// impl From<DecodeUtf8Error> for AppError {
//     fn from(e: DecodeUtf8Error) -> AppError {
//         AppError::DecodeUtf8Error(e)
//     }
// }
impl std::fmt::Display for AppError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IOError(e) => e.fmt(fmt),
            // AppError::DecodeUtf8Error(e) => e.fmt(fmt),
        }
    }
}
impl Error for AppError {
    fn description(&self) -> &str {
        "AppError"
    }
    // fn source(&self) -> Option<&dyn Error + 'static> {
    //     match self {
    //         AppError::IOError(e) => Some(e),
    // AppError::DecodeUtf8Error(e) => e.cause(),
    // }
    // }
}
