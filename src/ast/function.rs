use crate::{
    ast::{numeral::Numeral, Expr, Expression, SimplifyError},
    explanation::FormattingObserver,
};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Function {
    pub name: FunctionType,
    pub args: Vec<Expression>,
    pub simplified: bool,
}

// Constructor
impl Function {
    pub fn new(name: FunctionType, args: Vec<Expression>, simplified: bool) -> Self {
        Self {
            name,
            args,
            simplified,
        }
    }
}

impl Expr for Function {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let args: Vec<Expression> = self
            .args
            .iter_mut()
            .map(|arg| arg.simplify(explanation))
            .collect::<Result<Vec<Expression>, _>>()?;
        Ok(Expression::Function(Function::new(self.name.clone(), args, true)))
    }

    fn is_equal(&self, other: &Function) -> bool {
        self.name == other.name && Expression::compare_expression_vectors(&self.args, &other.args)
    }

    fn contains_var(&self, variable: &str) -> bool {
        match self.name.number_of_arguments() {
            1 => self.args[0].contains_var(variable),
            2 => self.args[1].contains_var(variable),
            _ => panic!("Sould'nt have more than 2 arguments"),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let args: Vec<String> = self.args.iter().map(|arg| arg.to_string()).collect();
        write!(f, "{}({})", self.name, args.join(", "))
    }
}

impl Function {
    pub fn simplify_function(
        &mut self,
        func: Function,
        args: Vec<Expression>,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        let mut rule = "";

        let result = match func.name {
            FunctionType::Pow => {
                match (&args[0], &args[1]) {
                    // 0^0 => ZeroExponentiationZero
                    (
                        Expression::Number(Numeral::Integer(0)),
                        Expression::Number(Numeral::Integer(0)),
                    ) => Err(SimplifyError::ZeroExponentiationZero),
                    // a^0 => 1
                    (Expression::Number(Numeral::Integer(0)), _) => {
                        rule = "using a^0 => 1";
                        Ok(Expression::integer(1))
                    }
                    // 1^x
                    (Expression::Number(_), Expression::Number(Numeral::Integer(1))) => {
                        rule = "using 1^x => 1";
                        Ok(Expression::integer(1))
                    }
                    // a^1 => a
                    (Expression::Number(Numeral::Integer(1)), lhs) => {
                        rule = "using a^1 => a";
                        Ok(lhs.clone())
                    }
                    _ => Ok(Expression::pow(args[1].clone(), args[0].clone())),
                }
            }
            a => Ok(Expression::Function(Function::new(a, args, true)))
            ,
        };

        if !rule.is_empty() {
            if let Some(explanation) = explanation {
                explanation.push(format!("Simplifiyng Exponentiation {}", rule,));
            }
        }

        result
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum FunctionType {
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

impl FunctionType {
    // Get the number of arguments a function takes
    pub fn number_of_arguments(&self) -> usize {
        match self {
            FunctionType::Sin
            | FunctionType::Cos
            | FunctionType::Tan
            | FunctionType::Asin
            | FunctionType::Acos
            | FunctionType::Atan
            | FunctionType::Sinh
            | FunctionType::Cosh
            | FunctionType::Tanh
            | FunctionType::Asinh
            | FunctionType::Acosh
            | FunctionType::Atanh
            | FunctionType::Sqrt
            | FunctionType::Exp
            | FunctionType::Ln
            | FunctionType::Log2
            | FunctionType::Log10
            | FunctionType::Abs
            | FunctionType::Ceil
            | FunctionType::Floor => 1,
            // Log(base, argument), Pow(order, arguments), Root(order, arguments)
            FunctionType::Log | FunctionType::Pow | FunctionType::Root => 2,
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            FunctionType::Ln => 2,
            FunctionType::Sin
            | FunctionType::Cos
            | FunctionType::Tan
            | FunctionType::Abs
            | FunctionType::Exp
            | FunctionType::Log
            | FunctionType::Pow => 3,
            FunctionType::Asin
            | FunctionType::Acos
            | FunctionType::Atan
            | FunctionType::Sinh
            | FunctionType::Cosh
            | FunctionType::Tanh
            | FunctionType::Sqrt
            | FunctionType::Log2
            | FunctionType::Ceil
            | FunctionType::Root => 4,
            FunctionType::Asinh
            | FunctionType::Acosh
            | FunctionType::Atanh
            | FunctionType::Log10
            | FunctionType::Floor => 5,
        }
    }
}

impl std::fmt::Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FunctionType::Sin => write!(f, "sin"),
            FunctionType::Cos => write!(f, "cos"),
            FunctionType::Tan => write!(f, "tan"),
            FunctionType::Asin => write!(f, "asin"),
            FunctionType::Acos => write!(f, "acos"),
            FunctionType::Atan => write!(f, "atan"),
            FunctionType::Sinh => write!(f, "sinh"),
            FunctionType::Cosh => write!(f, "cosh"),
            FunctionType::Tanh => write!(f, "tanh"),
            FunctionType::Asinh => write!(f, "asinh"),
            FunctionType::Acosh => write!(f, "acosh"),
            FunctionType::Atanh => write!(f, "atanh"),
            FunctionType::Sqrt => write!(f, "sqrt"),
            FunctionType::Exp => write!(f, "exp"),
            FunctionType::Ln => write!(f, "ln"),
            FunctionType::Log2 => write!(f, "log2"),
            FunctionType::Log10 => write!(f, "log10"),
            FunctionType::Abs => write!(f, "abs"),
            FunctionType::Ceil => write!(f, "ceil"),
            FunctionType::Floor => write!(f, "floor"),
            FunctionType::Log => write!(f, "log"),
            FunctionType::Pow => write!(f, "pow"),
            FunctionType::Root => write!(f, "root"),
        }
    }
}
