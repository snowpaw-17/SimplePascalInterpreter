use crate::pascal_interpreter::record::ActivationRecord as CallStackContent;
use crate::pascal_interpreter;

pub fn interpret_text(program_text: &str) -> Result<Vec<CallStackContent>, pascal_interpreter::error::RuntimeError> {
    let mut parser = pascal_interpreter::parser::Parser::from(program_text);
    let mut program = parser.parse()?;
    println!("Parse success");
      
    let mut syntax_analyzer = pascal_interpreter::semantic_analyzer::SemanticAnalyzer::new();
    syntax_analyzer.analyze(&mut program)?;
    println!("Syntax analysis success");
    
    let mut interpreter =  pascal_interpreter::interpreter::Interpreter::new();
    let result = interpreter.interpret(&mut program);
    println!("Memory is {:?}", interpreter.memory_tester);
    result.map(|_| interpreter.memory_tester)
}