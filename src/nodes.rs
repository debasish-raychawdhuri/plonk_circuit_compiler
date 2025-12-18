use std::{fmt::Debug, str::FromStr};

pub trait Node:Debug {}
pub trait ExpressionNode: Node {}
pub trait StatementNode: Node {}

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

impl Node for FieldLiteral {}
impl ExpressionNode for FieldLiteral {}

#[derive(Debug)]
pub struct BinaryOperationNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub operator: BinaryOperator,
}


#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    // Comparison operators
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    // Logical operators
    And,
    Or,
}

impl BinaryOperationNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>, operator: BinaryOperator) -> Self {
        BinaryOperationNode {
            left,
            right,
            operator,
        }
    }
    
}
impl Node for BinaryOperationNode {}

impl ExpressionNode for BinaryOperationNode {}

#[derive( Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}
#[derive( Debug)]
pub struct UnaryOperationNode {
    pub operand: Box<dyn Node>,
    pub operator: UnaryOperator,
}
impl UnaryOperationNode {
    pub fn new(operand: Box<dyn Node>, operator: UnaryOperator) -> Self {
        UnaryOperationNode { operand, operator }
    }
}
impl Node for UnaryOperationNode {}
impl ExpressionNode for UnaryOperationNode {}

#[derive( Debug)]
pub struct AssignmentNode {
    pub variable_name: String,
    pub value: Box<dyn Node>,
}

impl AssignmentNode {
    pub fn new(variable_name: String, value: Box<dyn Node>) -> Self {
        AssignmentNode {
            variable_name,
            value,
        }
    }
}
impl Node for AssignmentNode {}
impl StatementNode for AssignmentNode {}

#[derive( Debug)]
pub struct VariableNode {
    pub name: String,
}
impl VariableNode {
    pub fn new(name: String) -> Self {
        VariableNode { name }
    }
}
impl Node for VariableNode {}
impl ExpressionNode for VariableNode {}

#[derive( Debug)]
pub struct IfNode {
    pub condition: Box<dyn Node>,
    pub then_branch: Box<dyn Node>,
    pub else_branch: Option<Box<dyn Node>>,
}
    

impl IfNode {
    pub fn new(
        condition: Box<dyn Node>,
        then_branch: Box<dyn Node>,
        else_branch: Option<Box<dyn Node>>,
    ) -> Self {
        IfNode {
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl Node for IfNode {}
impl StatementNode for IfNode {}

struct WhileNode {
    pub condition: Box<dyn Node>,
    pub body: Box<dyn StatementNode>,
    pub max_iterations: u64,
}

#[derive( Debug)]
pub struct CompoundNode {
    pub statements: Vec<Box<dyn StatementNode>>,
}
impl Node for CompoundNode {}
impl StatementNode for CompoundNode {}

#[derive(Debug)]
pub struct Program {
    pub public_vars: Vec<String>,
    pub private_vars: Vec<String>,
    pub expressions: Vec<Box<dyn Node>>,
}

impl Program {
    pub fn new(public_vars: Vec<String>, private_vars: Vec<String>, expressions: Vec<Box<dyn Node>>) -> Self {
        Program {
            public_vars,
            private_vars,
            expressions
        }
    }
}

#[derive(Debug)]
pub struct CompoundExpression {
    pub statements: Vec<AssignmentNode>,
    pub expression: Box<dyn Node>,
}

impl CompoundExpression {
    pub fn new(statements: Vec<AssignmentNode>, expression: Box<dyn Node>) -> Self {
        CompoundExpression { statements, expression }
    }
}

impl Node for CompoundExpression {}
impl ExpressionNode for CompoundExpression {}

#[derive( Debug)]
struct FunctionDefinitionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<dyn StatementNode>,
}
impl Node for FunctionDefinitionNode {}
impl StatementNode for FunctionDefinitionNode {}

#[derive( Debug)]
pub struct FunctionCallNode {
    pub function_name: String,
    pub arguments: Vec<Box<dyn Node>>,
}

impl FunctionCallNode {
    pub fn new(function_name: String, arguments: Vec<Box<dyn Node>>) -> Self {
        FunctionCallNode {
            function_name,
            arguments,
        }
    }
}
impl Node for FunctionCallNode {}
impl StatementNode for FunctionCallNode {}

#[derive( Debug)]
struct ReturnNode {
    pub value: dyn ExpressionNode,
}
impl Node for ReturnNode {}
impl StatementNode for ReturnNode {}

#[derive( Debug)]
struct TableLookupNode {
    pub table_name: String,
    pub key: Box<dyn ExpressionNode>,
}
impl Node for TableLookupNode {}
impl ExpressionNode for TableLookupNode {}
