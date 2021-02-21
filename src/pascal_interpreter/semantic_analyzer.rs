use crate::pascal_interpreter::{
    ast::nodes::*,
    error::RuntimeError,
    literal::Literal,
    symbols,
};

use std::rc::Rc;

pub struct SemanticAnalyzer {
    current_scope : Rc<Option<symbols::ScopedSymbolTable>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer { 
            current_scope: Rc::from(None),
        }
    }

    pub fn analyze(&mut self, program: &mut ProgramNode) -> Result<Option<Literal>, RuntimeError> {
        self.current_scope = Rc::from(None);
        self.visit_program(program)
    }

    fn define_symbol(&mut self, name: &str, value: symbols::Symbol) {
        let table = Rc::get_mut(&mut self.current_scope).unwrap();

        table.as_mut().unwrap().define_symbol(name, value);
    }

    fn lookup_symbol(&self, name : &str) -> Option<&symbols::Symbol> {
        self.current_scope.as_ref().as_ref().and_then(|s| s.lookup_symbol(name))
    }

    fn lookup_symbol_current_scope_only(&self, name : &str) -> Option<&symbols::Symbol> {
        //self.current_scope.lookup_symbol(name)
        self.current_scope.as_ref().as_ref().and_then(|s| s.symbols.get(name))
    }

    fn get_current_scope_level(&self) -> u32 {
        self.current_scope.as_ref().as_ref().map(|s| s.nesting_level).unwrap_or(0u32)
    }
}

impl NodeVisitor for SemanticAnalyzer {
    fn visit_num(&mut self, _: &mut NumNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_binary_op(&mut self, visitable: &mut BinaryOpNode) -> Result<Option<Literal>, RuntimeError> {
        visitable.left_side.accept_visitor(self)?;
        visitable.right_side.accept_visitor(self)
    }

    fn visit_unary_op(&mut self, _: &mut UnaryOpNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_compound(&mut self, visitable: &mut CompoundStatementNode) -> Result<Option<Literal>, RuntimeError> {
        for statement in visitable.child_statements.iter_mut() {
            statement.accept_visitor(self)?;
        }
        Ok(None)
    }

    fn visit_var(&mut self, visitable: &mut VarNode) -> Result<Option<Literal>, RuntimeError> {
        let var_name = visitable.name.literal().to_str().ok_or(RuntimeError::IllformedVarExpr)?;
        self.lookup_symbol(var_name).ok_or(RuntimeError::UndefinedVariable(var_name.to_owned()))?;
        Ok(None)
    }    

     fn visit_assignment(&mut self, visitable: &mut AssignmentNode) -> Result<Option<Literal>, RuntimeError> {
         visitable.right.accept_visitor(self)?;
         self.visit_var(&mut visitable.left)
    }

    fn visit_no_op(&mut self, _: &mut NoOpNode) -> Result<Option<Literal>, RuntimeError> {
       Ok(None)
    }

    fn visit_program(&mut self, visitable: &mut ProgramNode) -> Result<Option<Literal>, RuntimeError> {
        let program_scope = Rc::from(
            Some(symbols::ScopedSymbolTable::from(String::from("global"), self.get_current_scope_level() + 1, Rc::from(None)))
        );
        self.current_scope = program_scope;
        self.visit_block(&mut visitable.block)?;
        
        self.current_scope = self.current_scope.as_ref().as_ref().unwrap().enclosing_scope.clone();

        Ok(None)
    }

     fn visit_block(&mut self, visitable: &mut BlockNode) -> Result<Option<Literal>, RuntimeError> {
        for decl in &mut visitable.declarations {
            decl.accept_visitor(self)?;
        }
        self.visit_compound(&mut visitable.compound_statement)?;
        Ok(None)  
    }

    fn visit_var_decl(&mut self, visitable: &mut VarDeclNode) -> Result<Option<Literal>, RuntimeError> {
        let type_name = visitable.type_spec.token.literal().to_str().ok_or(RuntimeError::IllformedVarExpr)?;
        let type_symbol = self.lookup_symbol(type_name);
        
        let variable_name = visitable.var.name.literal().to_str().ok_or(RuntimeError::IllformedVarExpr)?;
        let existing_var = self.lookup_symbol_current_scope_only(variable_name);
        
        match existing_var {
            Some(_) => Err(RuntimeError::VariableRedefinition(variable_name.to_lowercase())),
            None => Ok(())
            
        }?;
        
        let level = self.get_current_scope_level();
        match &type_symbol {
            Some(symbols::Symbol::Builtin(internal_type, _level)) => { 
                let var_symbol = symbols::Symbol::Var(internal_type.clone(), level);
                self.define_symbol(variable_name, var_symbol);
                Ok(None)
            },
            Some(_) => Err(RuntimeError::IllformedVarExpr),
            None => Err(RuntimeError::UnknownType(type_name.to_owned()))
         }        
    }

    fn visit_type(&mut self, _: &mut TypeNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_procedure_decl(&mut self, visitable: &mut ProcedureDeclNode) -> Result<Option<Literal>, RuntimeError> {
       let mut params : Vec<(String, symbols::Type)> = Vec::new();
        for i in &visitable.params {
            let param_name = i.var.name.literal().to_str().unwrap();
            let param_type = i.param_type.get_type();
            params.push((param_name.to_owned(), param_type));
            
        }

        let proc_name = visitable.name.to_str().unwrap();
        let procedure_symbol = symbols::Symbol::Procedure(params.clone(), self.get_current_scope_level(), visitable.block.clone());
        self.define_symbol(proc_name, procedure_symbol);
    
        let procedure_scope = Rc::from(
            Some(symbols::ScopedSymbolTable::from(proc_name.to_lowercase(), self.get_current_scope_level() + 1, self.current_scope.clone()))
        );
        self.current_scope = procedure_scope;
       
        for (param_name, param_type) in &params {
            let var_symbol = symbols::Symbol::Var(param_type.clone(), self.get_current_scope_level());
            self.define_symbol(&param_name, var_symbol);
        }

        self.visit_block(&mut visitable.block)?;
        self.current_scope = self.current_scope.as_ref().as_ref().unwrap().enclosing_scope.clone();
        Ok(None)
    }

    fn visit_param(&mut self, _: &mut ParamNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_procedure_call(&mut self, visitable: &mut ProcedureCallNode) -> Result<Option<Literal>, RuntimeError> {
        for param in &mut visitable.actual_params {
            param.accept_visitor(self)?;
        }
        let proc_symbol = self.current_scope.as_ref().as_ref().unwrap().lookup_symbol(visitable.name.to_str().unwrap()).map(|s| s.clone());

        visitable.proc_symbol = proc_symbol;
        Ok(None)
    }
}
