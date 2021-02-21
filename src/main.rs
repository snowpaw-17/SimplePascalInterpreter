use std::io;
use std::fs;

mod pascal_interpreter;

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
            let result = pascal_interpreter::helper::interpret_text(&content.unwrap());
            println!("Program result is {:?}", result);
        } else{
            println!("Failed to read file {}, reason:{:?}", &filename, &content);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::pascal_interpreter::helper::interpret_text;
    use crate::pascal_interpreter::literal::Literal as ValueLiteral;
    use crate::pascal_interpreter::error::RuntimeError;

    #[test]
    fn test_basic() {
        let program = r#"
            PROGRAM Test;
            VAR
                a : INTEGER;
            BEGIN
               a := 10 * ((5 + 3) * 2) - 21;  
            END.
        "#;

        let result = interpret_text(program);
        assert!(result.is_ok());

        let memory = result.unwrap();
        let program_memory = memory.iter().find(|&callstack| callstack.get_name() == "Test").unwrap();
        let value = program_memory.get_item("a");
        assert_eq!(value, Some(&ValueLiteral::Int(139)));
    }

    #[test]
    fn test_invalid_syntax() {
        let program = r#"
            PROGRAM Test;
            VAR
                a : INTEGER;
            BEGIN
               a := 10 * ((5 + 3) * ) - 21;  
            END.
        "#;

        let result = interpret_text(program);
        
        match result {
            Err(RuntimeError::UnexpectedToken(_, _)) => (),
            Err(e) => panic!("Unexpected error {:?}", e),
            Ok(_) => panic!("Expected failure")
        }
    }

    #[test]
    fn test_procedure_decl() {
        let program = r#"
            PROGRAM Part12;
                VAR
                number : INTEGER;
                a, b   : INTEGER;
                y      : REAL;

                PROCEDURE P1;
                VAR
                a : REAL;
                k : INTEGER;
                PROCEDURE P2;
                VAR
                    a, z : INTEGER;
                BEGIN {P2}
                    z := 777;
                END;  {P2}
                BEGIN {P1}

                END;  {P1}

                BEGIN {Part12}
                number := 2;
                a := number ;
                b := 10 * a + 10 * number DIV 4;
                y := 20 / 7 + 3.14
                END.  {Part12}
        "#;

        let result = interpret_text(program);
        assert!(result.is_ok());

        let memory = result.unwrap();
        let program_memory = memory.iter().find(|&callstack| callstack.get_name() == "Part12").unwrap();
        assert_eq!(program_memory.get_item("a"), Some(&ValueLiteral::Int(2)));
        assert_eq!(program_memory.get_item("b"), Some(&ValueLiteral::Float(25.0)));
    }

    #[test]
    fn test_procedure_call() {
        let program = r#"
            PROGRAM Main;

                PROCEDURE Alpha(a : INTEGER; b : INTEGER);
                    VAR x : INTEGER;
                BEGIN
                    x := (a + b ) * 2;
                END;

            BEGIN { Main }

                Alpha(3 + 5, 7);  { procedure call }

            END.  { Main }
        "#;

        let result = interpret_text(program);
        assert!(result.is_ok());

        let memory = result.unwrap();
        let proc_memory = memory.iter().find(|&callstack| callstack.get_name() == "Alpha").unwrap();
        assert_eq!(proc_memory.get_item("a"), Some(&ValueLiteral::Int(8)));
        assert_eq!(proc_memory.get_item("b"), Some(&ValueLiteral::Int(7)));
        assert_eq!(proc_memory.get_item("x"), Some(&ValueLiteral::Int(30)));
    }

    #[test]
    fn test_missing_variable() {
        let program = r#"
            PROGRAM Main;

                PROCEDURE Alpha(a : INTEGER; b : INTEGER);
                    VAR x : INTEGER;
                BEGIN
                    x := (a + b ) * 2 + x;
                END;

            BEGIN { Main }

                Alpha(3 + 5, 7);  { procedure call }

            END.  { Main }
        "#;

        let result = interpret_text(program);
        assert_eq!(result, Err(RuntimeError::UndefinedVariable("x".to_string())));
    }

    #[test]
    fn test_procedure_nested_call() {
        let program = r#"
            PROGRAM Main;

                PROCEDURE Alpha(a : INTEGER; b : INTEGER);
                    VAR x : INTEGER;

                    PROCEDURE Beta(a : integer; b : INTEGER);
                        VAR x : INTEGER;
                    BEGIN
                        x := a * 10 + b * 2;
                    END;

                BEGIN
                    x := (a + b ) * 2;

                    beta(5, 10);      { procedure call }
                END;

            begin { Main }

                Alpha(3 + 5, 7);  { procedure call }

            END.  { Main }
        "#;

        let result = interpret_text(program);
        assert!(result.is_ok());

        let memory = result.unwrap();
        let proc_memory = memory.iter().find(|&callstack| callstack.get_name() == "Alpha").unwrap();
        assert_eq!(proc_memory.get_item("a"), Some(&ValueLiteral::Int(8)));
        assert_eq!(proc_memory.get_item("b"), Some(&ValueLiteral::Int(7)));
        assert_eq!(proc_memory.get_item("x"), Some(&ValueLiteral::Int(30)));

        let proc_memory = memory.iter().find(|&callstack| callstack.get_name() == "beta").unwrap();
        assert_eq!(proc_memory.get_item("a"), Some(&ValueLiteral::Int(5)));
        assert_eq!(proc_memory.get_item("b"), Some(&ValueLiteral::Int(10)));
        assert_eq!(proc_memory.get_item("x"), Some(&ValueLiteral::Int(70)));
    }

}