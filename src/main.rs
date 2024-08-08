use thiserror::Error;
use std::io::Read;

#[derive(Error, Debug)]
pub enum ForError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error")]
    HttpRequest(#[from] reqwest::Error)

}

type Result<T> = std::result::Result<T, ForError>;

fn main() -> Result<()> {
    let mut response = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    println!("Status: {}", response.status());
    println!("Headers: \n{:#?}", response.headers());
    println!("Body: \n{}", body);

    Ok(())
}
