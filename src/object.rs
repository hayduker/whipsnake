#[derive(Debug, PartialEq)]
pub enum Object {
    Float(f64),
    String(String),
    Bool(bool),
    None,
}