use std::io;
use std::fs;

mod pascal_interpreter;

fn interpret_text(program_text: &str) -> Result<(), pascal_interpreter::error::RuntimeError> {
    let mut parser = pascal_interpreter::parser::Parser::from(program_text);
    let mut program = parser.parse()?;
    println!("Parse success");
      
    let mut syntax_analyzer = pascal_interpreter::semantic_analyzer::SemanticAnalyzer::new();
    syntax_analyzer.analyze(&mut program)?;
    println!("Syntax analysis success");
    
    let mut interpreter =  pascal_interpreter::interpreter::Interpreter::new();
    let result = interpreter.interpret(&mut program);
    println!("Memory is {:?}", interpreter.memory_tester);
    result.map(|_| ())
}

fn main() {
    loop {
        println!("load from file >>>");
        let mut filename = String::new();
        io::stdin()
            .read_line(&mut filename)
            .expect("Failed to read input");

        if filename.is_empty() {
            continue
        }

        let content = fs::read_to_string(&filename.trim_end());
        if content.is_ok() {
            let result = interpret_text(&content.unwrap());
            println!("Program result is {:?}", result);
        } else{
            println!("Failed to read file {}, reason:{:?}", &filename, &content);
        }
    }
}
