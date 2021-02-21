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
    Builtin(Type),
    Var(Type),
    Procedure(Vec<(String, Type)>,  BlockNode), // params
}

//#[derive(Clone)]
pub struct ScopedSymbolTable {
    name: String,
    nesting_level: u32,
    pub symbols : HashMap<String, Symbol>,
    pub enclosing_scope: Rc<Option<ScopedSymbolTable>>
}

impl ScopedSymbolTable {
    pub fn from(name: String, level: u32, enclosing_scope: Rc<Option<ScopedSymbolTable>>) -> Self {
        ScopedSymbolTable{
            name: name, 
            nesting_level: level, 
            symbols: ScopedSymbolTable::init_builtin_symbols(),
            enclosing_scope: enclosing_scope
        }
    }

    pub fn define_symbol(&mut self, name : &str, value : Symbol) {
        self.symbols.insert(name.to_lowercase(), value);
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(&name.to_lowercase()).or_else(
            || self.enclosing_scope.as_ref().as_ref().and_then(|s| s.lookup_symbol(name))
        )
    }

    fn init_builtin_symbols() -> HashMap<String, Symbol> {
        // self.current_scope.insert(symbols::Symbol::Builtin(SymbolDefinition{name: "Integer", internal_type: symbols::Type::Integer}), )
        let mut result = HashMap::new();
        result.insert(String::from("integer"), Symbol::Builtin(Type::Integer));
        result.insert(String::from("real"), Symbol::Builtin(Type::Float));
        result
    }
}
