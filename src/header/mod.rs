mod builder;
pub mod constants;
mod error;
mod parser;
mod validator;


pub use error::HeaderError;
pub use builder::HeaderBuilder;
pub use parser::HeaderParser;
pub use validator::HeaderValidator;

pub struct ParsedHeader {
    pub name: String,
    pub size: u64,
    pub mode: u64,
    pub uid: u64,
    pub gid: u64,
    pub mtime: u64,
}
