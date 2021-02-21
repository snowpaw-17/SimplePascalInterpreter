use super::literal::Literal;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ARType{
    Program,
    Procedure,
}

#[derive(Clone, PartialEq)]
pub struct ActivationRecord {
    name : Literal,
    record_type: ARType,
    nesting_level : u32,
    members : HashMap<String, Literal>
}

impl ActivationRecord {
    pub fn from(name: Literal, record_type : ARType, nesting_level: u32) -> Self {
        ActivationRecord{
            name: name,
            record_type: record_type,
            nesting_level: nesting_level,
            members: HashMap::new()
        }
    }

    pub fn get_item(&self, item_name: &str) -> Option<&Literal> {
        self.members.get(&item_name.to_lowercase())
    }

    pub fn set_item(&mut self, item_name: &str, value: Literal) {
        self.members.insert(item_name.to_lowercase(), value);
    }

    pub fn get_name(&self) -> &str {
        return self.name.to_str().unwrap_or("")
    }

    pub fn get_nesting_level(&self) -> u32 {
        return self.nesting_level
    }
}

impl fmt::Debug for ActivationRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\tname: {}\n\ttype: {:?}\n\tlevel: {}\n\tmembers: {:?}\n", self.name.to_str().unwrap_or(""), self.record_type, self.nesting_level, &self.members)
    }
}

pub struct ARCallStack {
    pub records: Vec<ActivationRecord>
}

impl ARCallStack {
    pub fn new() -> Self {
        ARCallStack{ records: Vec::new()}
    }

    pub fn push(&mut self, record: ActivationRecord) {
        self.records.push(record);
    }

    pub fn pop(&mut self) -> Option<ActivationRecord> {
        self.records.pop()
    }

    pub fn peek(&self) -> Option<&ActivationRecord> {
        self.records.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut ActivationRecord> {
        self.records.last_mut()
    }
}

pub trait CallStack {
    type Item;

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}
