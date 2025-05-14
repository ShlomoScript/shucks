
#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    None
}