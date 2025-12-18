# PLONK Circuit Compiler Grammar

## Overview
This grammar parses programs consisting of:
1. Optional public variable declarations
2. Optional private variable declarations
3. A list of boolean expressions that operate on 256-bit field elements

## Program Structure

A program consists of three parts:

1. **Public Variable Declarations** (optional):
   ```
   public: var1, var2, var3
   ```

2. **Private Variable Declarations** (optional):
   ```
   private: var4, var5, var6
   ```

3. **Boolean Expressions** (one or more, semicolon-separated):
   ```
   expr1; expr2; expr3
   ```

**Complete Example:**
```
public: x, y
private: z
x + y == z;
x > 0x0;
y < 0x100
```

## Literals
**Field Literals**: 256-bit integers in hexadecimal format only
- Format: `0x` or `0X` followed by 1-64 hexadecimal digits
- Examples: `0x1`, `0xFF`, `0xdeadbeef`, `0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`
- Stored internally as `[u64; 4]` in little-endian order:
  - `value[0]` = least significant 64 bits
  - `value[3]` = most significant 64 bits

## Operators (by precedence, highest to lowest)

1. **Unary Operators** (highest precedence)
   - `-` : Negation
   - `!` : Logical NOT

2. **Multiplicative Operators**
   - `*` : Multiplication
   - `/` : Division

3. **Additive Operators**
   - `+` : Addition
   - `-` : Subtraction

4. **Comparison Operators**
   - `==` : Equal
   - `!=` : Not equal
   - `<` : Less than
   - `<=` : Less than or equal
   - `>` : Greater than
   - `>=` : Greater than or equal

5. **Logical AND**
   - `&&` : Logical AND

6. **Logical OR** (lowest precedence)
   - `||` : Logical OR

## Syntax Elements

### Variables
- Start with letter or underscore
- Followed by letters, digits, or underscores
- Examples: `x`, `foo`, `my_var`, `_temp`

### Function Calls
- Format: `identifier(arg1, arg2, ...)`
- Arguments are expressions
- Examples: `foo(x, y)`, `bar(0x1, 0x2, x + y)`

### Expressions
- Field literals: `0x123`
- Variables: `x`
- Binary operations: `a + b`, `x > y`, `a && b`
- Unary operations: `-x`, `!flag`
- Parentheses for grouping: `(a + b) * c`
- Function calls: `foo(x, y)`
- If expressions: `if condition { then_expr } else { else_expr }`

### Programs
A program is a list of expressions separated by semicolons (`;`)

## Example Programs

```
// Program with public and private variables
public: x, y
private: z
x + y == z;
x > 0x0

// Simple comparison (no variable declarations)
x > 0x5

// Multiple boolean expressions
x == 0x1; y != 0x2; z >= 0x3

// Complex expression with arithmetic and boolean logic
(a + b) * c == d && e > f || g <= h

// Function calls
foo(x, y) == 0x2a && bar(z) != 0x0

// Large field values
0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF == 0x123456789ABCDEF

// Unary operators
!flag && -value > 0x0

// Nested expressions
(x + 0x1) * (y - 0x2) == z && foo(a, b) > 0x100

// If expressions
if x > 0x5 { 0xa } else { 0xb }

// Nested if expressions
if x > 0x0 { if y > 0x0 { 0x1 } else { 0x2 } } else { 0x3 }

// If in arithmetic expression
x + if flag { 0xa } else { 0xb } * 0x2

// If with complex conditions
if (a + b) > c && d == 0x0 { foo(x) } else { bar(y) }
```

## Usage

```rust
use plonk_circuit_compiler::grammar;

let input = "x > 0x5 && y < 0xa";
let program = grammar::ProgramParser::new()
    .parse(input)
    .expect("Failed to parse");

// program is of type Program, which contains Vec<Box<dyn Node>>
```

## Comments
The grammar supports two types of comments:
- Line comments: `// comment until end of line`
- Block comments: `/* comment can span multiple lines */`

## Whitespace
Whitespace (spaces, tabs, newlines) is automatically skipped and can be used freely for formatting.
