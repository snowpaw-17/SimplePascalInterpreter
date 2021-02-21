use crate::pascal_interpreter::ast::nodes::BlockNode;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Type
{
    Integer,
    Float
}

#[derive(Clone)]
pub enum Symbol {
    Builtin(Type, u32),
    Var(Type, u32),
    Procedure(Vec<(String, Type)>, u32,  BlockNode), // params
}

//#[derive(Clone)]
pub struct ScopedSymbolTable {
    pub name: String,
    pub nesting_level: u32,
    pub symbols : HashMap<String, Symbol>,
    pub enclosing_scope: Rc<Option<ScopedSymbolTable>>
}

impl ScopedSymbolTable {
    pub fn from(name: String, level: u32, enclosing_scope: Rc<Option<ScopedSymbolTable>>) -> Self {
        let mut result = ScopedSymbolTable{
            name: name, 
            nesting_level: level, 
            symbols: HashMap::new(),
            enclosing_scope: enclosing_scope
        };
        result.init_builtin_symbols();
        result
    }

    pub fn define_symbol(&mut self, name : &str, value : Symbol) {
        self.symbols.insert(name.to_lowercase(), value);
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(&name.to_lowercase()).or_else(
            || self.enclosing_scope.as_ref().as_ref().and_then(|s| s.lookup_symbol(name))
        )
    }

    fn init_builtin_symbols(&mut self)  {
        self.symbols.insert(String::from("integer"), Symbol::Builtin(Type::Integer, self.nesting_level));
        self.symbols.insert(String::from("real"), Symbol::Builtin(Type::Float, self.nesting_level));
    }
}
