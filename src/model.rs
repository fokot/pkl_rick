use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pkl {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(HashMap<String, Pkl>),
    List(Vec<Pkl>),
}