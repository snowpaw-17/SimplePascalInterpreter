use crate::pascal_interpreter::{
    ast::nodes::*,
    lexer::Lexer,
    token::*,
    error::RuntimeError,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn from(text: &'a str) -> Self {
        let mut lexer = Lexer::from(text);
        let token = lexer.get_next_token().unwrap();
        Parser {
            lexer: lexer,
            current_token: token
        }
    }

    // compare the current token type with the passed token
    // type and if they match then "eat" the current token
    // and assign the next token to the self.current_token,
    // otherwise raise an exception.
    fn eat(&mut self, token_type: TokenType) -> Result<(), RuntimeError> {
        if self.current_token.token_type() == token_type {
            self.current_token = self.lexer.get_next_token()?;
            return Ok(())
        }
        Err(RuntimeError::UnexpectedToken(self.current_token.clone(), token_type))
    }

    fn factor(&mut self) -> Result<Box<dyn VisitableNode>, RuntimeError> {
        match &self.current_token.token_type() {
            TokenType::IntegerConst | TokenType::FloatConst => {
                let result = NumNode::from(self.current_token.to_owned());
                self.eat(self.current_token.token_type())?;
                return Ok(Box::from(result));
            },
            TokenType::Lparen => {
                self.eat(TokenType::Lparen)?;
                let result = self.expr()?;
                self.eat(TokenType::Rparen)?;
                return Ok(result);
            },
            TokenType::Plus | TokenType::Minus => {
                let token = self.current_token.to_owned();
                self.eat(self.current_token.token_type())?;
                let arg = self.factor()?;
                return Ok(Box::from(UnaryOpNode::from(token, arg)));
            }
            TokenType::Identifier => {
                let variable = self.variable()?;
                return Ok(Box::from(variable))
            }
            _ => Err(RuntimeError::UnexpectedToken(self.current_token.clone(), TokenType::EOF))
        }
        
    }

    fn term(&mut self) -> Result<Box<dyn VisitableNode>, RuntimeError> {
        let ops = vec![TokenType::Multiply, TokenType::Division, TokenType::IntegerDivision];

        let mut node = self.factor()?;
        while ops.contains(&self.current_token.token_type()) {
            let token = self.current_token.to_owned();
            self.eat(token.token_type())?;
            let rhs = self.factor()?;
            let new_node = Box::from(BinaryOpNode::from(node, token, rhs));
            node = new_node
        }
        Ok(node)
    }

    fn expr(&mut self) -> Result<Box<dyn VisitableNode>, RuntimeError> {
        let ops = vec![TokenType::Plus, TokenType::Minus];

        let mut node = self.term()?;
        while ops.contains(&self.current_token.token_type()) {
            let token = self.current_token.to_owned();
            self.eat(token.token_type())?;
            let rhs = self.term()?;
            node = Box::from(BinaryOpNode::from(node, token, rhs));
        }
        Ok(node)   
    }

    fn compound_statement(&mut self, begin: TokenType, end: TokenType) -> Result<CompoundStatementNode, RuntimeError> {
        self.eat(begin)?;
        let statements = self.statement_list()?;
        self.eat(end)?;
       
        Ok(CompoundStatementNode::from(statements))
    }

    fn statement_list(&mut self) -> Result<Vec<Box<dyn VisitableNode>>, RuntimeError> {
        let node = self.statement()?;
        let mut statements = Vec::new();
        statements.push(node);
        
        while self.current_token.token_type() == TokenType::Semi {
            self.eat(TokenType::Semi)?;
            statements.push(self.statement()?)
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Box<dyn VisitableNode>, RuntimeError> {
        let result : Box<dyn VisitableNode> = match self.current_token.token_type() {
            TokenType::Begin => {
                let compound = self.compound_statement(TokenType::Begin, TokenType::End)?;
                Box::from(compound)
            },
            TokenType::Identifier => {
                let current_char = self.lexer.get_current_char();
                match current_char {
                    Some('(') => Box::from(self.proccall_statement()?),
                    _ => Box::from(self.assignment_statement()?)
                }
            },
            _ => Box::from(self.empty())
        };
        Ok(result)
       
    }

    fn variable(&mut self) -> Result<VarNode, RuntimeError> {
        let name = self.current_token.to_owned();
        self.eat(TokenType::Identifier)?;
        Ok(VarNode::from(name))
    }

    fn assignment_statement(&mut self) -> Result<AssignmentNode, RuntimeError> {
        let left = self.variable()?;
        self.eat(TokenType::Colon)?;
        self.eat(TokenType::Assignment)?;
        
        let right = self.expr()?;
        Ok(AssignmentNode::from(left, right))
    }

    fn empty(&self) -> NoOpNode {
        NoOpNode{}
    }

    fn program(&mut self) -> Result<ProgramNode, RuntimeError> {
        self.eat(TokenType::Program)?;
        
        let prog_name = self.current_token.literal().clone();
        self.eat(TokenType::Identifier)?;
        self.eat(TokenType::Semi)?;
        
        let block_node = self.block()?;
        self.eat(TokenType::Dot)?;

        Ok(ProgramNode::from(prog_name, block_node))
    }
    
    fn block(&mut self) -> Result<BlockNode, RuntimeError> {
        let declarations = self.declarations()?;
        let compound = self.compound_statement(TokenType::Begin, TokenType::End)?;

        Ok(BlockNode::from(declarations, compound))
    }

    fn declarations(&mut self) -> Result<Vec<Box<dyn VisitableNode>>, RuntimeError> {
        let mut declarations : Vec<Box<dyn VisitableNode>> = Vec::new();
        
        if self.current_token.token_type() == TokenType::Var {
            self.eat(TokenType::Var)?;
            
            while self.current_token.token_type() == TokenType::Identifier {
                let var_declarations = self.variable_declarations()?;
                declarations.extend(var_declarations);
                self.eat(TokenType::Semi)?;
            }
        }
                
       while self.current_token.token_type() == TokenType::Procedure {
            let proc_decl = self.procedure_declaration()?;
            declarations.push(Box::from(proc_decl));
       }

        Ok(declarations)
    }

    fn variable_declarations(&mut self) -> Result<Vec<Box<dyn VisitableNode>>, RuntimeError> {
        let mut variables : Vec<VarNode> = Vec::new();
        variables.push(VarNode::from(self.current_token.to_owned()));
        
        self.eat(TokenType::Identifier)?; 
        
        while self.current_token.token_type() == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            variables.push(VarNode::from(self.current_token.to_owned()));
            self.eat(TokenType::Identifier)?;
        }
        self.eat(TokenType::Colon)?;

        let type_spec = self.type_spec()?; 
        let mut variable_declarations : Vec<Box<dyn VisitableNode>> = Vec::new();
        for var in variables.into_iter() {
            let decl : Box<dyn VisitableNode> = Box::from(VarDeclNode::from(var, type_spec.clone()));
            variable_declarations.push(decl);
        }
        Ok(variable_declarations)
    }

    fn type_spec(&mut self) -> Result<TypeNode, RuntimeError> {
        match self.current_token.token_type() {
            TokenType::IntegerType | TokenType::FloatType => { 
                    let token = self.current_token.to_owned();
                    self.eat(token.token_type())?;
                    Ok(TypeNode::from(token))
            },
            _ => Err(RuntimeError::UnknownType(self.current_token.literal().to_str().unwrap_or("").to_string()))
        }
    }

    fn formal_parameters(&mut self) -> Result<Vec<ParamNode>, RuntimeError> {
        let mut param_tokens = Vec::new();
        param_tokens.push(self.current_token.clone());
        
        self.eat(TokenType::Identifier)?;

        while self.current_token.token_type() == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            param_tokens.push(self.current_token.clone());
            self.eat(TokenType::Identifier)?;
        }

        self.eat(TokenType::Colon)?;
        let type_node = self.type_spec()?;

        let mut param_nodes = Vec::new();
        for i in param_tokens {
            param_nodes.push(ParamNode::from(VarNode::from(i), type_node.clone()));
        }
        Ok(param_nodes)
     }

    fn formal_parameter_list(&mut self) -> Result<Vec<ParamNode>, RuntimeError> {
        if self.current_token.token_type() != TokenType::Identifier {
            return Ok(Vec::new())
        }
        let mut param_nodes = self.formal_parameters()?;

        while self.current_token.token_type() == TokenType::Semi {
            self.eat(TokenType::Semi)?;
            let params = self.formal_parameters()?;
            param_nodes.extend(params);
        }
        Ok(param_nodes)
    }

    fn procedure_declaration(&mut self) -> Result<ProcedureDeclNode, RuntimeError> {
        self.eat(TokenType::Procedure)?;
        let proc_name = self.current_token.literal().clone();
        self.eat(TokenType::Identifier)?;
        let mut params = Vec::new();

        if self.current_token.token_type() == TokenType::Lparen {
            self.eat(TokenType::Lparen)?;
            params = self.formal_parameter_list()?;
            self.eat(TokenType::Rparen)?;
        }
        self.eat(TokenType::Semi)?;
        let block_node = self.block()?;
        let proc_decl = ProcedureDeclNode::from(proc_name, params, block_node);
        self.eat(TokenType::Semi)?;
        return Ok(proc_decl);
    }

    fn proccall_statement(&mut self) -> Result<ProcedureCallNode, RuntimeError> {
         //"""proccall_statement : ID LPAREN (expr (COMMA expr)*)? RPAREN"""
        let proc_name = self.current_token.literal().clone();
        self.eat(TokenType::Identifier)?;
        self.eat(TokenType::Lparen)?;
       
        let mut actual_params : Vec<Box<dyn VisitableNode>>= Vec::new();
        if self.current_token.token_type() != TokenType::Rparen {
            let node = self.expr()?;
            actual_params.push(node);
        }
            
        while self.current_token.token_type() == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            let node = self.expr()?;
            actual_params.push(node);
        }  
        self.eat(TokenType::Rparen)?;

        Ok(ProcedureCallNode::from(proc_name, actual_params))
    }
       
    pub fn parse(&mut self) -> Result<ProgramNode, RuntimeError> {
		self.program()
	}
}
