use std::collections::HashMap;

use crate::ast::{Expression, constant, numeral::Numeral};

pub trait PrettyPrints: std::fmt::Display {
    // Printing methods
    fn calculate_tree(&self, indent: usize) -> String;

    fn calculate_positions(
        &self,
        memoization: &mut HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    );

    fn get_below_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize;
    fn get_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize;
    fn get_length(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize;

    /// Print the expression in a multiline format
    ///
    /// # Exemple
    /// ```
    /// use sym_rustic::ast::Expression;
    ///
    /// let expr = Expression::Addition(vec![
    ///         Expression::Multiplication(vec![
    ///             Expression::Variable("x".to_string()),
    ///             Expression::integer(2)
    ///         ]),
    ///         Expression::integer(5)
    ///     ]);
    /// expr.print_tree(0)
    /// ```
    /// Addition:
    ///   + Multiplication:
    ///     * x
    ///     * 2
    ///   + 5
    ///
    fn print_tree(&self, indent: usize) {
        println!("{}", self.calculate_tree(indent))
    }

    /// Print like we would on paper
    ///
    /// # Exemple
    /// ```
    /// use sym_rustic::ast::Expression;
    ///
    /// let expr = Expression::Addition(vec![
    ///         Expression::Exponentiation(
    ///             Box::new(Expression::Variable("x".to_string())),
    ///             Box::new(Expression::integer(2))
    ///         ),
    ///         Expression::integer(5)
    ///     ]);
    /// expr.print_console();
    /// let result = " 2    \nx  + 5";
    /// ```
    fn print_console(&self) {
        println!("{}\n", self.get_processed())
    }

    
    fn get_processed(&self) -> String {
        let mut memoization: HashMap<Expression, (usize, usize)> = HashMap::new();
        let mut position: Vec<(String, (usize, usize))> = Vec::new();
        self.calculate_positions(&mut memoization, &mut position, (0, 0));

        let length = self.get_length(&mut memoization);
        let height = self.get_height(&mut memoization);

        let mut result: Vec<Vec<String>> = vec![vec![" ".to_string(); length]; height];
        for (line, (y, x)) in position {
            result[y][x] = line;
        }

        result
            .iter()
            .rev()
            .map(|line| line.join(""))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn calculate_parenthesis(
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
        left: bool,
        height: usize,
    ) {
        if height == 1 {
            position.push((
                if left {
                    "(".to_string()
                } else {
                    ")".to_string()
                },
                prev_pos,
            ))
        } else {
            position.push((
                if left {
                    "/".to_string()
                } else {
                    "\\".to_string()
                },
                (prev_pos.0 + height - 1, prev_pos.1),
            ));

            for i in 0..(height - 2) {
                position.push(("|".to_string(), (prev_pos.0 + height - i - 2, prev_pos.1)));
            }
            position.push((
                if !left {
                    "/".to_string()
                } else {
                    "\\".to_string()
                },
                (prev_pos.0, prev_pos.1),
            ));
        }
    }
}

// Print functions
impl PrettyPrints for Expression {
    fn calculate_tree(&self, indent: usize) -> String {
        match self {
            Expression::Negation(negation) => negation.calculate_tree(indent),
            Expression::Number(numeral) => numeral.calculate_tree(indent),
            Expression::Variable(variable) => variable.calculate_tree(indent),
            Expression::Constant(constant) => constant.calculate_tree(indent),
            Expression::Addition(addition) => addition.calculate_tree(indent),
            Expression::Multiplication(multiplication) => multiplication.calculate_tree(indent),
            Expression::Subtraction(subtraction) => subtraction.calculate_tree(indent),
            Expression::Division(division) =>division.calculate_tree(indent),
            Expression::Exponentiation(exponentiation) => exponentiation.calculate_tree(indent),
            Expression::Equality(equality) => equality.calculate_tree(indent),
            Expression::Complex(complex) => complex.calculate_tree(indent),
            Expression::Function(function) => function.calculate_tree(indent),
            Expression::Derivative(derivative) => derivative.calculate_tree(indent),
        }
    }

    fn calculate_positions(
        &self,
        memoization: &mut HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        match self {
            Expression::Negation(negation) => negation.calculate_positions(memoization, position, prev_pos),
            Expression::Number(numeral) => numeral.calculate_positions(memoization, position, prev_pos),
            Expression::Variable(variable) => variable.calculate_positions(memoization, position, prev_pos),
            Expression::Constant(constant) => constant.calculate_positions(memoization, position, prev_pos),
            Expression::Addition(addition) => addition.calculate_positions(memoization, position, prev_pos),
            Expression::Multiplication(multiplication) => multiplication.calculate_positions(memoization, position, prev_pos),
            Expression::Subtraction(subtraction) => subtraction.calculate_positions(memoization, position, prev_pos),
            Expression::Division(division) => division.calculate_positions(memoization, position, prev_pos),
            Expression::Exponentiation(exponentiation) => exponentiation.calculate_positions(memoization, position, prev_pos),
            Expression::Equality(equality) => equality.calculate_positions(memoization, position, prev_pos),
            Expression::Complex(complex) => complex.calculate_positions(memoization, position, prev_pos),
            Expression::Function(function) => function.calculate_positions(memoization, position, prev_pos),
            Expression::Derivative(derivative) => derivative.calculate_positions(memoization, position, prev_pos),
        }
    }

    fn get_below_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Expression::Negation(negation) => negation.get_below_height(memoization),
            Expression::Number(numeral) => numeral.get_below_height(memoization),
            Expression::Variable(variable) => variable.get_below_height(memoization),
            Expression::Constant(constant) => constant.get_below_height(memoization),
            Expression::Addition(addition) => addition.get_below_height(memoization),
            Expression::Multiplication(multiplication) => multiplication.get_below_height(memoization),
            Expression::Subtraction(subtraction) => subtraction.get_below_height(memoization),
            Expression::Division(division) => division.get_below_height(memoization),
            Expression::Exponentiation(exponentiation) => exponentiation.get_below_height(memoization),
            Expression::Equality(equality) => equality.get_below_height(memoization),
            Expression::Complex(complex) => complex.get_below_height(memoization),
            Expression::Function(function) => function.get_below_height(memoization),
            Expression::Derivative(derivative) => derivative.get_below_height(memoization),
        }
    }

    fn get_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        if let Some((height, _length)) = memoization.get(self) {
            if *height != 0 {
                return *height;
            }
        }
        
        let height = match self {
            Expression::Negation(negation) => negation.get_height(memoization),
            Expression::Number(numeral) => numeral.get_height(memoization),
            Expression::Variable(variable) => variable.get_height(memoization),
            Expression::Constant(constant) => constant.get_height(memoization),
            Expression::Addition(addition) => addition.get_height(memoization),
            Expression::Multiplication(multiplication) => multiplication.get_height(memoization),
            Expression::Subtraction(subtraction) => subtraction.get_height(memoization),
            Expression::Division(division) => division.get_height(memoization),
            Expression::Exponentiation(exponentiation) => exponentiation.get_height(memoization),
            Expression::Equality(equality) => equality.get_height(memoization),
            Expression::Complex(complex) => complex.get_height(memoization),
            Expression::Function(function) => function.get_height(memoization),
            Expression::Derivative(derivative) => derivative.get_height(memoization),
        };

        if let Some((h, _l)) = memoization.get_mut(self) {
            *h = height;
        } else {
            memoization.insert(self.clone(), (height, 0));
        }

        height
    }

    fn get_length(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        if let Some((_height, length)) = memoization.get(self) {
            if *length != 0 {
                return *length;
            }
        }
        
        let length = match self {
            Expression::Negation(negation) => negation.get_length(memoization),
            Expression::Number(numeral) => numeral.get_length(memoization),
            Expression::Variable(variable) => variable.get_length(memoization),
            Expression::Constant(constant) => constant.get_length(memoization),
            Expression::Addition(addition) => addition.get_length(memoization),
            Expression::Multiplication(multiplication) => multiplication.get_length(memoization),
            Expression::Subtraction(subtraction) => subtraction.get_length(memoization),
            Expression::Division(division) => division.get_length(memoization),
            Expression::Exponentiation(exponentiation) => exponentiation.get_length(memoization),
            Expression::Equality(equality) => equality.get_length(memoization),
            Expression::Complex(complex) => complex.get_length(memoization),
            Expression::Function(function) => function.get_length(memoization),
            Expression::Derivative(derivative) => derivative.get_length(memoization),
        };

        if let Some((_h, l)) = memoization.get_mut(self) {
            *l = length;
        } else {
            memoization.insert(self.clone(), (0, length));
        }

        length
    }

    fn calculate_parenthesis(
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
        left: bool,
        height: usize,
    ) {
        if height == 1 {
            position.push((
                if left {
                    "(".to_string()
                } else {
                    ")".to_string()
                },
                prev_pos,
            ))
        } else {
            position.push((
                if left {
                    "/".to_string()
                } else {
                    "\\".to_string()
                },
                (prev_pos.0 + height - 1, prev_pos.1),
            ));

            for i in 0..(height - 2) {
                position.push(("|".to_string(), (prev_pos.0 + height - i - 2, prev_pos.1)));
            }
            position.push((
                if !left {
                    "/".to_string()
                } else {
                    "\\".to_string()
                },
                (prev_pos.0, prev_pos.1),
            ));
        }
    }
}
