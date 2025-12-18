use plonk_circuit_compiler::grammar;

fn main() {
    // Example 0: Program with public and private variables
    let input0 = "public: x, y private: z x + y == z; x > 0x0";
    match grammar::ProgramParser::new().parse(input0) {
        Ok(program) => {
            println!("✓ Parsed: {}", input0);
            println!("  Public vars: {:?}", program.public_vars);
            println!("  Private vars: {:?}", program.private_vars);
            println!("  Expressions: {} boolean expressions", program.expressions.len());
        },
        Err(e) => println!("✗ Error parsing '{}': {:?}", input0, e),
    }

    // Example 1: Simple boolean expression (no var declarations)
    let input1 = "x > 0x5 && y < 0xa";
    match grammar::ProgramParser::new().parse(input1) {
        Ok(_program) => println!("✓ Parsed: {}", input1),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input1, e),
    }

    // Example 2: Multiple boolean expressions
    let input2 = "a == b; c != d; e >= f";
    match grammar::ProgramParser::new().parse(input2) {
        Ok(program) => println!("✓ Parsed: {}", input2),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input2, e),
    }

    // Example 3: Complex expression with arithmetic and boolean operators
    let input3 = "(a + b) * c == d && e > f || g <= h";
    match grammar::ProgramParser::new().parse(input3) {
        Ok(program) => println!("✓ Parsed: {}", input3),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input3, e),
    }

    // Example 4: Function calls
    let input4 = "foo(x, y) == 0x2a && bar(z) != 0x0";
    match grammar::ProgramParser::new().parse(input4) {
        Ok(program) => println!("✓ Parsed: {}", input4),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input4, e),
    }

    // Example 5: Field literals (hex only)
    let input5 = "0x123abc == 0x456; field1 + 0xdeadbeef > 0x3e8";
    match grammar::ProgramParser::new().parse(input5) {
        Ok(program) => println!("✓ Parsed: {}", input5),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input5, e),
    }

    // Example 6: Large 256-bit values
    let input6_large = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF == 0x123456789ABCDEF";
    match grammar::ProgramParser::new().parse(input6_large) {
        Ok(program) => println!("✓ Parsed: {}", input6_large),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input6_large, e),
    }

    // Example 7: Unary operators
    let input7 = "!flag && -value > 0x0";
    match grammar::ProgramParser::new().parse(input7) {
        Ok(_program) => println!("✓ Parsed: {}", input7),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input7, e),
    }

    // Example 8: If expressions with parentheses around condition
    let input8 = "if (x > 0x5) 0xa else 0xb";
    match grammar::ProgramParser::new().parse(input8) {
        Ok(_program) => println!("✓ Parsed: {}", input8),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input8, e),
    }

    // Example 9: Nested if expressions
    let input9 = "if (x > 0x0) if (y > 0x0) 0x1 else 0x2 else 0x3";
    match grammar::ProgramParser::new().parse(input9) {
        Ok(_program) => println!("✓ Parsed: {}", input9),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input9, e),
    }

    // Example 10: If in arithmetic expression
    let input10 = "x + if (flag) 0xa else 0xb";
    match grammar::ProgramParser::new().parse(input10) {
        Ok(_program) => println!("✓ Parsed: {}", input10),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input10, e),
    }

    // Example 11: Compound expressions
    let input11 = "{ x = 0x1; y = 0x2; x + y }";
    match grammar::ProgramParser::new().parse(input11) {
        Ok(_program) => println!("✓ Parsed: {}", input11),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input11, e),
    }

    // Example 12: If with compound expressions
    let input12 = "if (x > 0x5) { y = 0xa; y } else { 0xb }";
    match grammar::ProgramParser::new().parse(input12) {
        Ok(_program) => println!("✓ Parsed: {}", input12),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input12, e),
    }

    // Example 13: Nested compound and if
    let input13 = "{ a = if (x) 0x1 else 0x2; b = a + 0x3; b }";
    match grammar::ProgramParser::new().parse(input13) {
        Ok(_program) => println!("✓ Parsed: {}", input13),
        Err(e) => println!("✗ Error parsing '{}': {:?}", input13, e),
    }
}
