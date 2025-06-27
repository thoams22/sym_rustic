// src/ast/function.rs
#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Function {
    // 1 argument
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Sqrt,
    Exp,
    Ln,
    Log2,
    Log10,
    Abs,
    Ceil,
    Floor,
    // 2 arguments
    Log,
    Pow,
    Root,
}

impl Function {
    // Get the number of arguments a function takes
    pub fn number_of_arguments(&self) -> usize {
        match self {
            Function::Sin
            | Function::Cos
            | Function::Tan
            | Function::Asin
            | Function::Acos
            | Function::Atan
            | Function::Sinh
            | Function::Cosh
            | Function::Tanh
            | Function::Asinh
            | Function::Acosh
            | Function::Atanh
            | Function::Sqrt
            | Function::Exp
            | Function::Ln
            | Function::Log2
            | Function::Log10
            | Function::Abs
            | Function::Ceil
            | Function::Floor => 1,
            // Log(base, argument), Pow(order, arguments), Root(order, arguments)
            Function::Log | Function::Pow | Function::Root => 2,
        }
    }

    pub fn get_length(&self)-> usize {
        match self {
            Function::Ln => 2,
            Function::Sin | Function::Cos | Function::Tan | Function::Abs | Function::Exp | Function::Log | Function::Pow => {
                3
            }
            Function::Asin
            | Function::Acos
            | Function::Atan
            | Function::Sinh
            | Function::Cosh
            | Function::Tanh
            | Function::Sqrt
            | Function::Log2
            | Function::Ceil
            
            | Function::Root => 4,
            Function::Asinh
            | Function::Acosh
            | Function::Atanh 
            | Function::Log10
            | Function::Floor => 5

        }
    }
}


impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Function::Sin => write!(f, "sin"),
            Function::Cos => write!(f, "cos"),
            Function::Tan => write!(f, "tan"),
            Function::Asin => write!(f, "asin"),
            Function::Acos => write!(f, "acos"),
            Function::Atan => write!(f, "atan"),
            Function::Sinh => write!(f, "sinh"),
            Function::Cosh => write!(f, "cosh"),
            Function::Tanh => write!(f, "tanh"),
            Function::Asinh => write!(f, "asinh"),
            Function::Acosh => write!(f, "acosh"),
            Function::Atanh => write!(f, "atanh"),
            Function::Sqrt => write!(f, "sqrt"),
            Function::Exp => write!(f, "exp"),
            Function::Ln => write!(f, "ln"),
            Function::Log2 => write!(f, "log2"),
            Function::Log10 => write!(f, "log10"),
            Function::Abs => write!(f, "abs"),
            Function::Ceil => write!(f, "ceil"),
            Function::Floor => write!(f, "floor"),
            Function::Log => write!(f, "log"),
            Function::Pow => write!(f, "pow"),
            Function::Root => write!(f, "root"),
        }
    }
}