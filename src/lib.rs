pub mod nodes;

// LALRPOP generates the parser module
#[allow(clippy::all)]
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_simple_expression() {
        let input = "x + y";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_boolean_expression() {
        let input = "x > 0x5 && y < 0xa";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_multiple_expressions() {
        let input = "x == 0x5; y != 0xa; z >= 0x14";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_complex_expression() {
        let input = "(a + b) * c == d && e > f || g <= h";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_function_call() {
        let input = "foo(x, y, z) == 0x2a";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_field_literal() {
        let input = "0x123abc == 0x1c8";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_field_literal_parsing() {
        use nodes::FieldLiteral;
        use std::str::FromStr;

        // Test small hex value
        let lit = FieldLiteral::from_str("0x1").unwrap();
        assert_eq!(lit.value, [1, 0, 0, 0]);

        // Test larger hex value
        let lit = FieldLiteral::from_str("0xFF").unwrap();
        assert_eq!(lit.value, [0xFF, 0, 0, 0]);

        // Test value that spans into second u64
        let lit = FieldLiteral::from_str("0x10000000000000000").unwrap();
        assert_eq!(lit.value, [0, 1, 0, 0]);

        // Test max value for first u64
        let lit = FieldLiteral::from_str("0xFFFFFFFFFFFFFFFF").unwrap();
        assert_eq!(lit.value, [0xFFFFFFFFFFFFFFFF, 0, 0, 0]);

        // Test full 256-bit value
        let lit = FieldLiteral::from_str("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        assert_eq!(lit.value, [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]);

        // Test case insensitivity
        let lit1 = FieldLiteral::from_str("0xAbCdEf").unwrap();
        let lit2 = FieldLiteral::from_str("0xabcdef").unwrap();
        assert_eq!(lit1.value, lit2.value);
    }

    #[test]
    fn test_field_literal_errors() {
        use nodes::FieldLiteral;
        use std::str::FromStr;

        // Too long (more than 64 hex digits)
        let result = FieldLiteral::from_str("0x10000000000000000000000000000000000000000000000000000000000000000");
        assert!(result.is_err());

        // Invalid character
        let result = FieldLiteral::from_str("0xGG");
        assert!(result.is_err());
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x > 0x5) 0xa else 0xb";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_nested_if_expression() {
        let input = "if (x > 0x0) if (y > 0x0) 0x1 else 0x2 else 0x3";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_if_in_expression() {
        let input = "x + if (flag) 0xa else 0xb";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_if_comparison() {
        let input = "if (a == b) x else y > 0x10";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_if_with_compound() {
        let input = "if (x > 0x5) { y = 0xa; y } else { 0xb }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_compound_expression() {
        let input = "{ x = 0x1; y = 0x2; x + y }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_compound_no_statements() {
        let input = "{ 0x42 }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_nested_compound() {
        let input = "{ x = { y = 0x1; y + 0x2 }; x * 0x3 }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_compound_in_arithmetic() {
        let input = "a + { b = 0x5; b * 0x2 }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_compound_with_multiple_assignments_and_if() {
        let input = "{ x = 0x1; y = x + 0x2; z = if (y > 0x5) y else 0x0; z }";
        let program = grammar::ProgramParser::new().parse(input);
        assert!(program.is_ok());
    }

    #[test]
    fn test_public_vars() {
        let input = "public: x, y, z x > 0x5";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.public_vars, vec!["x", "y", "z"]);
        assert_eq!(program.private_vars.len(), 0);
    }

    #[test]
    fn test_private_vars() {
        let input = "private: a, b, c a + b == c";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.private_vars, vec!["a", "b", "c"]);
        assert_eq!(program.public_vars.len(), 0);
    }

    #[test]
    fn test_public_and_private_vars() {
        let input = "public: x, y private: z x + y == z";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.public_vars, vec!["x", "y"]);
        assert_eq!(program.private_vars, vec!["z"]);
    }

    #[test]
    fn test_program_with_multiple_expressions() {
        let input = "public: x private: y x > 0x5; y < 0xa; x + y == 0xf";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.public_vars, vec!["x"]);
        assert_eq!(program.private_vars, vec!["y"]);
        assert_eq!(program.expressions.len(), 3);
    }

    #[test]
    fn test_no_var_declarations() {
        let input = "x + y";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.public_vars.len(), 0);
        assert_eq!(program.private_vars.len(), 0);
    }

    #[test]
    fn test_simple_function_def() {
        let input = "fn add(x:Field, y:Field): Field { x + y } add(0x1, 0x2)";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[0].parameters.len(), 2);
        assert_eq!(program.functions[0].parameters[0].name, "x");
        assert_eq!(program.functions[0].parameters[1].name, "y");
    }

    #[test]
    fn test_function_no_params() {
        let input = "fn get_zero(): Field { 0x0 } get_zero()";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "get_zero");
        assert_eq!(program.functions[0].parameters.len(), 0);
    }

    #[test]
    fn test_multiple_functions() {
        let input = "fn add(x:Field, y:Field): Field { x + y } fn mul(a:Field, b:Field): Field { a * b } add(0x1, 0x2)";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[1].name, "mul");
    }

    #[test]
    fn test_full_program_with_functions() {
        let input = "public: x, y private: z fn add(a:Field, b:Field): Field { a + b } x + y == z; add(x, y) == z";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.public_vars, vec!["x", "y"]);
        assert_eq!(program.private_vars, vec!["z"]);
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.expressions.len(), 2);
    }

    #[test]
    fn test_function_with_compound_body() {
        let input = "fn compute(x:Field): Field { temp = x + 0x1; temp * 0x2 } compute(0x5)";
        let program = grammar::ProgramParser::new().parse(input).unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "compute");
    }
}
