use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Field,
    // Can add more types later if needed
}

impl Type {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "Field" | "field" => Ok(Type::Field),
            _ => Err("Unknown type - expected 'Field'"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

impl Parameter {
    pub fn new(name: String, param_type: Type) -> Self {
        Parameter { name, param_type }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldLiteral {
    // 256-bit value stored as 4 u64s in little-endian order
    // value[0] = least significant 64 bits
    // value[3] = most significant 64 bits
    pub value: [u64; 4],
}

impl FieldLiteral {
    pub fn new(value: [u64; 4]) -> Self {
        FieldLiteral { value }
    }
}

impl FromStr for FieldLiteral {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove any whitespace
        let s = s.trim();

        // Check if it starts with 0x or 0X
        let hex_str = if s.starts_with("0x") || s.starts_with("0X") {
            &s[2..]
        } else {
            s
        };

        // A 256-bit number has at most 64 hex digits
        if hex_str.len() > 64 {
            return Err(format!(
                "Hex string too long: {} digits (max 64 for 256-bit value)",
                hex_str.len()
            ));
        }

        if hex_str.is_empty() {
            return Err("Empty hex string".to_string());
        }

        // Parse the hex string into a 256-bit value
        let mut value = [0u64; 4];

        // Process the hex string from right to left (least significant digits first)
        let chars: Vec<char> = hex_str.chars().collect();

        for (i, &c) in chars.iter().rev().enumerate() {
            let digit_value = match c {
                '0'..='9' => (c as u64) - ('0' as u64),
                'a'..='f' => (c as u64) - ('a' as u64) + 10,
                'A'..='F' => (c as u64) - ('A' as u64) + 10,
                _ => return Err(format!("Invalid hex character: '{}'", c)),
            };

            // Each hex digit is 4 bits
            // Determine which u64 this digit belongs to (16 hex digits per u64)
            let u64_index = i / 16;
            let bit_offset = (i % 16) * 4;

            if u64_index < 4 {
                value[u64_index] |= digit_value << bit_offset;
            }
        }

        Ok(FieldLiteral { value })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    // Arithmetic operators (inlined)
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),

    // Comparison operators (inlined)
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),

    // Logical operators (inlined)
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    // Unary operators (inlined)
    Negate(Box<Expression>),
    Not(Box<Expression>),

    // Primary expressions
    Literal(FieldLiteral),
    Variable(String),

    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },

    Compound {
        statements: Vec<Statement>,
        expression: Box<Expression>,
    },

    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment {
        variable_name: String,
        value: Expression,
    },
}

#[derive(Debug)]
pub struct Program {
    pub public_vars: Vec<String>,
    pub private_vars: Vec<String>,
    pub functions: Vec<FunctionDefinition>,
    pub expressions: Vec<Expression>,
}

impl Program {
    pub fn new(
        public_vars: Vec<String>,
        private_vars: Vec<String>,
        functions: Vec<FunctionDefinition>,
        expressions: Vec<Expression>,
    ) -> Self {
        Program {
            public_vars,
            private_vars,
            functions,
            expressions,
        }
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Expression,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Expression,
    ) -> Self {
        FunctionDefinition {
            name,
            parameters,
            return_type,
            body,
        }
    }
}
