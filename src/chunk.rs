//! Contains byte code instructions and chunk types.

use crate::token::Line;
use crate::value::Value;
use std::fmt::{self, Display, Formatter};

type ConstantIndex = u16;

/// A C-like enum that represents the different types of instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Instruction {
    /// Marks the end of a function's execution.
    Return,
    /// Pushes a constant value onto the stack from the constant pool.
    Constant(ConstantIndex),
    /// Pushes a true value onto the stack.
    True,
    /// Pushes a false value onto the stack.
    False,
    // Integer arithmetic instructions.
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Boolean arithmetic instructions
    Not,
    And,
    Or,
    Xor,
    // String instructions
    Concat,
    // Comparison instructions
    Lt,
    Gt,
    Le,
    Ge,
    IntEq,
    IntNe,
    BoolEq,
    BoolNe,
    StringEq,
    StringNe,
    PtrEq,
    PtrNe,
    /// Drops a primitive value placed one before the last value on the stack
    PrimitiveDropAbove,
    /// Drops a pointer value placed one before the uppermost value on the stack
    PtrDropAbove,
    /// Pushes a primitive value onto the stack.
    PrimitiveGetLocal(u16),
    /// Pushes a pointer value onto the stack.
    PtrGetLocal(u16),
    /// Jumps the specified number of instructions forward unconditionally.
    Jump(u16),
    /// Jumps the specified number of instructions forward, if the value on the
    /// top of the stack it consumes is a false.
    JumpIfFalse(u16),
    /// Pops a value and function off the stack and calls it with the value.
    Call,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // fallback to Debug's displaying
        write!(f, "{:?}", self)
    }
}

/// A sequence of byte code instructions with their parameters and line numbers.
#[derive(Debug, Default, Clone)]
pub struct Chunk {
    instructions: Vec<Instruction>,
    instruction_lines: Vec<Line>,
    constants: Vec<Value>,
    name: Option<String>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn name_mut(&mut self) -> Option<&mut String> {
        self.name.as_mut()
    }

    /// Adds a new instruction to the chunk with the given line information.
    pub fn write(&mut self, op: Instruction, line: Line) {
        self.instructions.push(op);
        self.instruction_lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value) -> ConstantIndex {
        let index = self.constants.len() as ConstantIndex;
        self.constants.push(value);
        index
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn instructions_mut(&mut self) -> &mut [Instruction] {
        &mut self.instructions
    }

    pub fn instruction_lines(&self) -> &[Line] {
        &self.instruction_lines
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }
}

pub mod prelude {
    pub use super::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_size() {
        assert_eq!(std::mem::size_of::<Instruction>(), 4);
    }
}
