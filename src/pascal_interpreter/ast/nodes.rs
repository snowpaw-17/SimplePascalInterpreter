use crate::pascal_interpreter::{
    literal::Literal,
    error::RuntimeError,
    symbols,
    token::{
        Token,
        TokenType 
    }
};

use std::fmt;
use std::fmt::Debug;

pub trait VisitableNode  {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor)  -> Result<Option<Literal>, RuntimeError>;
    fn box_clone(&self) -> Box<dyn VisitableNode>;
    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub trait NodeVisitor {
    fn visit_num(&mut self, _: &mut NumNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_binary_op(&mut self, _: &mut BinaryOpNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_unary_op(&mut self, _: &mut UnaryOpNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_compound(&mut self, _: &mut CompoundStatementNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_var(&mut self, _: &mut VarNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_assignment(&mut self, _: &mut AssignmentNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_no_op(&mut self, _: &mut NoOpNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_program(&mut self, _: &mut ProgramNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_block(&mut self, _: &mut BlockNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_var_decl(&mut self, _: &mut VarDeclNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_type(&mut self, _: &mut TypeNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_procedure_decl(&mut self, _: &mut ProcedureDeclNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_param(&mut self, _: &mut ParamNode) -> Result<Option<Literal>, RuntimeError>;
    fn visit_procedure_call(&mut self, _: &mut ProcedureCallNode) -> Result<Option<Literal>, RuntimeError>;
}

/// Node containing a singal integral token
#[derive(Clone, Debug)]
pub struct NumNode {
    pub token : Token
}
impl NumNode {
    pub fn from(token: Token) -> Self {
        NumNode{ token : token }
    }

}

impl VisitableNode for NumNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_num(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

/// Binary operation nodes - expr op exprs
pub struct BinaryOpNode {
    pub left_side: Box<dyn VisitableNode>,
    pub op: Token,
    pub right_side: Box<dyn VisitableNode>,
}

impl BinaryOpNode {
    pub fn from(left_side: Box<dyn VisitableNode>, op: Token, right_side: Box<dyn VisitableNode>) -> Self
    {
        BinaryOpNode{left_side: left_side, op: op, right_side: right_side }
    }

}

impl VisitableNode for BinaryOpNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_binary_op(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Clone for BinaryOpNode {
    fn clone(&self) -> Self {
        BinaryOpNode {
            left_side: self.left_side.box_clone(),
            op: self.op.clone(),
            right_side: self.right_side.box_clone(),
        }
    }
}

impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BinaryOp(lhs = ")?;
        self.left_side.box_fmt(f)?;
        write!(f, ", op = {:?}, rhs = ", self.op)?;
        self.right_side.box_fmt(f)
    }
}


/// Operation that takes operation token and single argument to work on
pub struct UnaryOpNode {
    pub op : Token,
    pub node: Box<dyn VisitableNode>
}

impl UnaryOpNode {
    pub fn from(token: Token, node : Box<dyn VisitableNode>) -> Self {
        UnaryOpNode{ op: token, node: node }
    }

}

impl VisitableNode for UnaryOpNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_unary_op(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Clone for UnaryOpNode {
    fn clone(&self) -> Self {
        UnaryOpNode {
            op: self.op.clone(),
            node: self.node.box_clone(),
        }
    }
}

impl fmt::Debug for UnaryOpNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnaryOp(op={:?}", self.op)?;
        self.node.box_fmt(f)?;
        write!(f, ")")
    }
}



/// Compound statement nodes contain multiple subsequent statements.
/// Does not return result when visited
pub struct CompoundStatementNode {
    pub child_statements: Vec<Box<dyn VisitableNode>>
}

impl CompoundStatementNode {
    pub fn from(children: Vec<Box<dyn VisitableNode>>) -> Self {
        CompoundStatementNode{child_statements: children }
    }
}

impl VisitableNode for CompoundStatementNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_compound(self)
    }
    
    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Clone for CompoundStatementNode {
    fn clone(&self) -> Self {
        let mut child_statements : Vec<Box<dyn VisitableNode>> = Vec::new();
        for statement in &self.child_statements {
            child_statements.push(statement.box_clone());
        }
        CompoundStatementNode {
            child_statements: child_statements
        }
    }
}

impl fmt::Debug for CompoundStatementNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compound(")?;
        for statement in &self.child_statements {
            statement.box_fmt(f)?;
            write!(f, ";")?;
        }
        Ok(())
    }
}


/// Var node stores identifier of a variable
#[derive(Clone, Debug)]
pub struct VarNode {
    pub name: Token,
}

impl VarNode {
    pub fn from(name: Token) -> Self {
        VarNode{name: name}
    }
}

impl VisitableNode for VarNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_var(self)
    }
    
    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

pub struct AssignmentNode {
    pub left: VarNode,
    pub right: Box<dyn VisitableNode>,

}

impl AssignmentNode {
    pub fn from(var: VarNode, expr: Box<dyn VisitableNode>) -> Self {
        AssignmentNode { left: var, right: expr}
    }
}

impl VisitableNode for AssignmentNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_assignment(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Clone for AssignmentNode {
    fn clone(&self) -> Self {
        AssignmentNode {
            left : self.left.clone(),
            right: self.right.box_clone()
        }
    }
}

impl fmt::Debug for AssignmentNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nAssignment node(left = ")?;
        self.left.fmt(f)?;
        write!(f, ";right = ")?;
        self.right.box_fmt(f)?;
        write!(f, ")")
    }
}


/// Empty statement node
#[derive(Clone, Debug)]
pub struct NoOpNode {}

impl VisitableNode for NoOpNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_no_op(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
    
}

#[derive(Clone, Debug)]
pub struct ProgramNode {
    pub name: Literal,
    pub block : BlockNode
}

impl ProgramNode {
    pub fn from(name: Literal, block: BlockNode) -> Self {
        ProgramNode{name: name, block: block}
    }
}

impl VisitableNode for ProgramNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_program(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}


/// Program block
pub struct BlockNode {
    pub declarations : Vec<Box<dyn VisitableNode>>,
    pub compound_statement : CompoundStatementNode
}

impl BlockNode {
    pub fn from(declarations: Vec<Box<dyn VisitableNode>>, compound_statement: CompoundStatementNode) -> Self {
        BlockNode{declarations: declarations, compound_statement: compound_statement}
    }
}

impl Clone for BlockNode {
    fn clone(&self) -> Self {
        let mut declarations : Vec<Box<dyn VisitableNode>> = Vec::new();
        for decl in &self.declarations {
            declarations.push(decl.box_clone());
        }
        BlockNode {
            declarations: declarations,
            compound_statement: self.compound_statement.clone()
        }
    }
}

impl VisitableNode for BlockNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_block(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for BlockNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block(decl = [")?;
        for decl in &self.declarations {
            decl.box_fmt(f)?;
            write!(f, ";")?;
        }
        write!(f, "]; compound = ")?;
        self.compound_statement.fmt(f)?;
        write!(f, ")")
    }
}


/// Variable declaration contains the initialization statments for a variable with its type
#[derive(Clone, Debug)]
pub struct VarDeclNode {
    pub var: VarNode,
    pub type_spec: TypeNode
}

impl VarDeclNode {
    pub fn from(var: VarNode, type_spec: TypeNode) -> Self {
        VarDeclNode{var: var, type_spec: type_spec}
    }
}

impl VisitableNode for VarDeclNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_var_decl(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}


#[derive(Clone, Debug)]
pub struct TypeNode {
    pub token : Token,
}

impl TypeNode {
    pub fn from(token: Token) -> Self {
        TypeNode{token: token}
    }

    pub fn get_type(&self) -> symbols::Type {
        match self.token.token_type() {
            TokenType::IntegerType => symbols::Type::Integer,
            TokenType::FloatType => symbols::Type::Float,
            _ => panic!("Unhandled type {:?}", self.token.token_type())
        }
    }
}

impl VisitableNode for TypeNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_type(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct ProcedureDeclNode {
    pub name: Literal,
    pub params : Vec<ParamNode>,
    pub block : BlockNode
}

impl ProcedureDeclNode {
    pub fn from(name: Literal, params: Vec<ParamNode>, block: BlockNode) -> Self {
        ProcedureDeclNode{name: name, params: params, block: block}
    }
}

impl VisitableNode for ProcedureDeclNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_procedure_decl(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct ParamNode {
    pub var: VarNode,
    pub param_type: TypeNode
}

impl ParamNode {
    pub fn from(var: VarNode, param_type : TypeNode) -> Self {
        ParamNode{var: var, param_type: param_type}
    }
}

impl VisitableNode for ParamNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_param(self)
    }

    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

pub struct ProcedureCallNode {
    pub name: Literal,
    pub actual_params: Vec<Box<dyn VisitableNode>>,
    pub proc_symbol : Option<symbols::Symbol>
}

impl ProcedureCallNode {
    pub fn from(name: Literal, actual_params : Vec<Box<dyn VisitableNode>>) -> Self {
        ProcedureCallNode{name: name, actual_params: actual_params, proc_symbol: None}
    }
}

impl VisitableNode for ProcedureCallNode {
    fn accept_visitor(&mut self, visitor: &mut dyn NodeVisitor) -> Result<Option<Literal>, RuntimeError> {
        visitor.visit_procedure_call(self)
    }
    
    fn box_clone(&self) -> Box<dyn VisitableNode> {
        Box::new((*self).clone())
    }

    fn box_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Clone for ProcedureCallNode {
    fn clone(&self) -> Self {
        let mut actual_params : Vec<Box<dyn VisitableNode>> = Vec::new();
        for decl in &self.actual_params {
            actual_params.push(decl.box_clone());
        }
        ProcedureCallNode {
            name : self.name.clone(),
            actual_params: actual_params,
            proc_symbol: self.proc_symbol.clone()
        }
    }
}


impl fmt::Debug for ProcedureCallNode {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProcedureCall({:?})", self.name)
    }
}