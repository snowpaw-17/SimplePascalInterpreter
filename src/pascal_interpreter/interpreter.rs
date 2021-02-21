use crate::pascal_interpreter::{
    ast::nodes::*,
    token::TokenType,
    error::RuntimeError,
    literal::Literal,
    record,
    record::CallStack,
    symbols
};

pub struct Interpreter {
    callstack: record::ARCallStack,
    pub memory_tester: Vec<record::ActivationRecord>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { 
            callstack: record::ARCallStack::new(),
            memory_tester: Vec::new(),
        }
    }

    pub fn interpret(&mut self, program: &mut ProgramNode) -> Result<Option<Literal>, RuntimeError> {
        self.callstack = record::ARCallStack::new();
        self.visit_program(program)
    }
}

impl NodeVisitor for Interpreter {
    fn visit_num(&mut self, visitable: &mut NumNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(Some(visitable.token.literal().clone()))
    }

    fn visit_binary_op(&mut self, visitable: &mut BinaryOpNode) -> Result<Option<Literal>, RuntimeError> {
        
        let lhs = visitable.left_side.as_mut().accept_visitor(self)?;
        let rhs = visitable.right_side.as_mut().accept_visitor(self)?;
        
        let lhs = lhs.ok_or(RuntimeError::MissingArgument)?;
        let rhs = rhs.ok_or(RuntimeError::MissingArgument)?;

        let result = match visitable.op.token_type() {
            TokenType::Plus     => Ok(Some(lhs + rhs)),
            TokenType::Minus    => Ok(Some(lhs - rhs)),
            TokenType::Multiply => Ok(Some(lhs * rhs)),
            TokenType::IntegerDivision | TokenType::Division => {
                rhs.to_int().filter(|&val| val != 0).ok_or(RuntimeError::DivisionByZero)?;
                Ok(Some(lhs / rhs))
            }
            TokenType::Modulus => Ok(Some(lhs % rhs)),
            _ => Err(RuntimeError::UnhandledBinaryOp(visitable.op.clone()))
        };
        result
    }

    fn visit_unary_op(&mut self, visitable: &mut UnaryOpNode) -> Result<Option<Literal>, RuntimeError> {
        let arg = visitable.node.as_mut().accept_visitor(self)?
            .ok_or(RuntimeError::MissingArgument)?;

        match visitable.op.token_type() {
            TokenType::Plus     => return Ok(Some(Literal::from_int(0)+arg)),
            TokenType::Minus    => return Ok(Some(Literal::from_int(0)-arg)),
            _ => Err(RuntimeError::UnhandledUnaryOp(visitable.op.clone())),
        }
    }

    fn visit_compound(&mut self, visitable: &mut CompoundStatementNode) -> Result<Option<Literal>, RuntimeError> {
        for statement in visitable.child_statements.iter_mut() {
            statement.accept_visitor(self)?;
        }
        Ok(None)
    }

    fn visit_var(&mut self, visitable: &mut VarNode) -> Result<Option<Literal>, RuntimeError> {
        let var_name = visitable.name.literal().to_str().ok_or(RuntimeError::IllformedVarExpr)?;
        
        let ar = self.peek_mut().ok_or(RuntimeError::StackUnderflow)?;
        Ok(ar.get_item(var_name).map(|v| v.to_owned()))
    }    

     fn visit_assignment(&mut self, visitable: &mut AssignmentNode) -> Result<Option<Literal>, RuntimeError> {
        let var_name = visitable.left.name.literal().to_str().ok_or(RuntimeError::IllformedVarExpr)?;
        let expr_result = visitable.right.as_mut().accept_visitor(self)?
            .ok_or(RuntimeError::IllformedVarExpr)?;

        let ar = self.peek_mut().ok_or(RuntimeError::StackUnderflow)?;
        ar.set_item(var_name, expr_result);
        Ok(None)
    }

    fn visit_no_op(&mut self, _: &mut NoOpNode) -> Result<Option<Literal>, RuntimeError> {
       Ok(None)
    }

    fn visit_program(&mut self, visitable: &mut ProgramNode) -> Result<Option<Literal>, RuntimeError> {
        let ar = record::ActivationRecord::from(visitable.name.clone(), record::ARType::Program, 1);
        self.push(ar);
        self.visit_block(&mut visitable.block)?;
        self.pop();
        Ok(None)
    }

     fn visit_block(&mut self, visitable: &mut BlockNode) -> Result<Option<Literal>, RuntimeError> {
        for decl in &mut visitable.declarations {
            decl.accept_visitor(self)?;
        }
        self.visit_compound(&mut visitable.compound_statement)
    }

    fn visit_var_decl(&mut self, _: &mut VarDeclNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_type(&mut self, _: &mut TypeNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_procedure_decl(&mut self, _: &mut ProcedureDeclNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_param(&mut self, _: &mut ParamNode) -> Result<Option<Literal>, RuntimeError> {
        Ok(None)
    }

    fn visit_procedure_call(&mut self, visitable: &mut ProcedureCallNode) -> Result<Option<Literal>, RuntimeError> {
        
        match &mut visitable.proc_symbol {
            Some(symbols::Symbol::Procedure(formal_params, level, block_node)) => {
                let mut ar = record::ActivationRecord::from(visitable.name.clone(), record::ARType::Procedure, *level);

                for (formal, actual) in formal_params.iter_mut().zip(visitable.actual_params.iter_mut()) {
                    let eval_param = actual.accept_visitor(self)?;
                    let eval_param = eval_param.ok_or(RuntimeError::MissingArgument)?;
                    ar.set_item(&formal.0, eval_param);
                }

                self.push(ar);
                self.visit_block(block_node)?;
                self.pop();
                Ok(None)
                
            },
            _ => Err(RuntimeError::MissingProcedure)
        }
    }
}

impl record::CallStack for Interpreter {
    type Item = record::ActivationRecord;

    fn push(&mut self, record: record::ActivationRecord) {
        self.callstack.push(record);
    }

    fn pop(&mut self) -> Option<record::ActivationRecord> {
        
        let elem = self.callstack.pop();
        if elem.is_some() {
            self.memory_tester.push(elem.as_ref().unwrap().clone());
        }
        elem
    }

    fn peek(&self) -> Option<&record::ActivationRecord> {
        self.callstack.peek()
    }

    fn peek_mut(&mut self) -> Option<&mut record::ActivationRecord> {
        self.callstack.peek_mut()
    }

}