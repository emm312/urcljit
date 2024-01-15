use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, Hash)]
pub enum Opcode {
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Mov(Operand),
    Imm(Operand),
    Jmp(Operand),
    Rsh(Operand),
    Lsh(Operand),
    Lod(Operand),
    Str(Operand, Operand),
    Bge(LabelHash, Operand, Operand),
    Nor(Operand, Operand),
    In(Port),
    Out(Port, Operand),
    Brc(LabelHash, Operand, Operand),
    Bnc(LabelHash, Operand, Operand),
    Hlt,
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Instruction {
    pub opcode: Opcode,
    pub yielded: Option<Register>,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Register {
    Sp,
    Gpr(u8),
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Sp => write!(f, "sp"),
            Register::Gpr(n) => write!(f, "r{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Label(LabelHash),
}

/// A label is normally a string (not Copy)
/// if we simply hash the string it becomes copyable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabelHash(u64);

impl From<String> for LabelHash {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for LabelHash {
    fn from(value: &str) -> Self {
        let mut hasher = DefaultHasher::default();
        value.hash(&mut hasher);
        LabelHash(hasher.finish())
    }
}

impl Display for LabelHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".L{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Port {
    Text,
    X,
    Y,
    Colour,
    Numb,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Program {
    Label(LabelHash),
    Instruction(Instruction),
}

#[macro_export]
macro_rules! gen_instr {
    ($opc:ident, $dst:ident, $($ops:ident),*) => {
        Instruction {
            opcode: Opcode::$opc($($ops,)*),
            yielded: Some($dst),
        }
    };

    ($opc:ident, !, $($ops:ident),*) => {
        Instruction {
            opcode: Opcode::$opc($($ops,)*),
            yielded: None,
        }
    };
    ($opc:ident, !) => {
        Instruction {
            opcode: Opcode::$opc,
            yielded: None,
        }
    };
}
