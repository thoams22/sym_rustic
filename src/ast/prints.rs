use std::collections::HashMap;

use super::{Expression, constant, numeral::Numeral};

// Print functions
impl Expression {
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
    /// print!("{}", expr.print_tree(0))
    /// ```
    /// Addition:
    ///   + Multiplication:
    ///     * x
    ///     * 2
    ///   + 5
    ///
    pub fn print_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        match self {
            Expression::Addition(terms) => {
                if terms.is_empty() {
                    "0".to_string()
                } else if terms.len() == 1 {
                    terms[0].print_tree(indent)
                } else {
                    let mut result = String::from("Addition:\n");
                    for (i, term) in terms.iter().enumerate() {
                        result.push_str(&format!(
                            "{}{}{}",
                            next_indent_str,
                            "+ ",
                            term.print_tree(next_indent)
                        ));
                        if i < terms.len() - 1 {
                            result.push('\n');
                        }
                    }
                    result
                }
            }
            Expression::Multiplication(terms) => {
                if terms.is_empty() {
                    "1".to_string()
                } else if terms.len() == 1 {
                    terms[0].print_tree(indent)
                } else {
                    let mut result = String::from("Multiplication:\n");
                    for (i, term) in terms.iter().enumerate() {
                        result.push_str(&format!(
                            "{}{}{}",
                            next_indent_str,
                            "* ",
                            term.print_tree(next_indent)
                        ));
                        if i < terms.len() - 1 {
                            result.push('\n');
                        }
                    }
                    result
                }
            }
            Expression::Subtraction(lhs, rhs) => {
                format!(
                    "Subtraction:\n{}{}\n{}- {}",
                    next_indent_str,
                    lhs.print_tree(next_indent),
                    next_indent_str,
                    rhs.print_tree(next_indent)
                )
            }
            Expression::Division(lhs, rhs) => {
                format!(
                    "Division:\n{}{}\n{}/ {}",
                    next_indent_str,
                    lhs.print_tree(next_indent),
                    next_indent_str,
                    rhs.print_tree(next_indent)
                )
            }
            Expression::Exponentiation(lhs, rhs) => {
                format!(
                    "Exponentiation:\n{}{}\n{}^ {}",
                    next_indent_str,
                    lhs.print_tree(next_indent),
                    next_indent_str,
                    rhs.print_tree(next_indent)
                )
            }
            Expression::Equality(lhs, rhs) => {
                format!(
                    "Equality:\n{}{}\n{}= {}",
                    next_indent_str,
                    lhs.print_tree(next_indent),
                    next_indent_str,
                    rhs.print_tree(next_indent)
                )
            }
            Expression::Complex(real, imag) => {
                format!(
                    "Complex:\n{}{}\n{}i {}",
                    next_indent_str,
                    real.print_tree(next_indent),
                    next_indent_str,
                    imag.print_tree(next_indent)
                )
            }
            Expression::Variable(name) => name.to_string(),
            Expression::Constant(constant) => constant.to_string(),
            Expression::Negation(expr) => {
                format!(
                    "Negation:\n{}- {}",
                    next_indent_str,
                    expr.print_tree(indent)
                )
            }
            Expression::Function(func, args) => {
                let mut result = format!("{}(", func);
                for (i, arg) in args.iter().enumerate() {
                    result.push_str(&arg.print_tree(0));
                    if i < args.len() - 1 {
                        result.push_str(", ");
                    }
                }
                result.push(')');
                result
            }
            Expression::Number(num) => {
                format!("{}", num)
            }
            Expression::Derivative(expr, variable, order) => {
                format!(
                    "Derivative{}:\n{}{}\n{}' {}",
                    if *order > 1 {
                        format!("({})", order)
                    } else {
                        "".to_owned()
                    },
                    next_indent_str,
                    expr.print_tree(next_indent),
                    next_indent_str,
                    variable,
                )
            }
        }
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
    pub fn print_console(&self) {
        println!("{}\n", self.print_aa())
    }

    pub fn print_aa(&self) -> String {
        let mut memoization: HashMap<Expression, (usize, usize)> = HashMap::new();
        let mut position: Vec<(String, (usize, usize))> = Vec::new();
        self.calculate_aa(&mut memoization, &mut position, (0, 0));

        // println!("{:?}", position);

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

    fn calculate_aa(
        &self,
        memoization: &mut HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        match self {
            Expression::Addition(expressions) => {
                let mut pos = prev_pos;
                let below_height = self.get_below_height(memoization);

                expressions.iter().enumerate().for_each(|(i, x)| {
                    let new_height = pos.0 + below_height - x.get_below_height(memoization);
                    x.calculate_aa(memoization, position, (new_height, pos.1));
                    pos.1 += x.get_length(memoization);
                    if i < expressions.len() - 1 {
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push(("+".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                    }
                });
            }
            Expression::Multiplication(expressions) => {
                let mut pos = prev_pos;
                let below_height = self.get_below_height(memoization);

                expressions.iter().enumerate().for_each(|(i, x)| {
                    let new_height = pos.0 + below_height - x.get_below_height(memoization);
                    x.calculate_aa(memoization, position, (new_height, pos.1));
                    pos.1 += x.get_length(memoization);
                    if i < expressions.len() - 1 {
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push(("*".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                    }
                });
            }
            Expression::Subtraction(a, b) => {
                let mut pos = prev_pos;
                a.calculate_aa(memoization, position, pos);
                pos.1 += a.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("-".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                b.calculate_aa(memoization, position, pos);
            }
            Expression::Division(n, d) => {
                let length = self.get_length(memoization);
                let bottom_height = d.get_height(memoization);

                let bottom_length = d.get_length(memoization);
                let top_length = n.get_length(memoization);

                let (span, top) = if top_length > bottom_length {
                    ((top_length - bottom_length) / 2, false)
                } else {
                    ((bottom_length - top_length) / 2, true)
                };

                let mut pos = prev_pos;

                if !top {
                    pos.1 += span;
                    d.calculate_aa(memoization, position, pos);
                    pos.1 -= span;
                } else {
                    d.calculate_aa(memoization, position, pos);
                }

                pos.0 += bottom_height;

                for _ in 0..length {
                    position.push(("-".to_string(), pos));
                    pos.1 += 1;
                }

                pos.1 -= length;
                pos.0 += 1;

                if top {
                    pos.1 += span;
                    n.calculate_aa(memoization, position, pos);
                    pos.1 -= span;
                } else {
                    n.calculate_aa(memoization, position, pos);
                }
            }
            Expression::Exponentiation(b, e) => {
                let mut pos = prev_pos;
                if matches!(
                    **b,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_, _)
                        | Expression::Complex(_, _)
                        | Expression::Division(_, _)
                ) {
                    Self::calculate_parenthesis(position, pos, true, b.get_height(memoization));
                    pos.1 += 1;
                }
                b.calculate_aa(memoization, position, pos);
                pos.1 += b.get_length(memoization);
                if matches!(
                    **b,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_, _)
                        | Expression::Complex(_, _)
                        | Expression::Division(_, _)
                ) {
                    Self::calculate_parenthesis(position, pos, false, b.get_height(memoization));
                    pos.1 += 1;
                }
                pos.0 += b.get_height(memoization);
                e.calculate_aa(memoization, position, pos);
            }
            Expression::Equality(a, b) => {
                let mut pos = prev_pos;
                a.calculate_aa(memoization, position, pos);
                pos.1 += a.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("=".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                b.calculate_aa(memoization, position, pos);
            }
            Expression::Complex(a, b) => {
                let mut pos = prev_pos;
                a.calculate_aa(memoization, position, pos);
                pos.1 += a.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("+".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                b.calculate_aa(memoization, position, pos);
                pos.1 += b.get_length(memoization);
                position.push(("i".to_string(), pos));
            }
            Expression::Variable(name) => {
                for (i, c) in name.chars().enumerate() {
                    position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
                }
            }
            Expression::Constant(constant) => {
                for (i, c) in constant.to_string().chars().enumerate() {
                    position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
                }
            }
            Expression::Negation(a) => {
                let mut pos = prev_pos;
                position.push(("-".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                a.calculate_aa(memoization, position, pos);
            }
            Expression::Function(function, expressions) => {
                let mut pos = prev_pos;
                let height = self.get_height(memoization);

                function.to_string().chars().for_each(|c| {
                    position.push((c.to_string(), pos));
                    pos.1 += 1;
                });
                Self::calculate_parenthesis(position, pos, true, height);
                pos.1 += 1;
                expressions.iter().for_each(|x| {
                    x.calculate_aa(memoization, position, pos);
                    pos.1 += x.get_length(memoization);
                });
                Self::calculate_parenthesis(position, pos, false, height);
            }
            Expression::Number(Numeral::Integer(num)) => {
                for (i, c) in num.to_string().chars().enumerate() {
                    position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
                }
            }
            Expression::Number(Numeral::Rational(num, den)) => Expression::Division(
                Box::new(Expression::integer(*num)),
                Box::new(Expression::integer(*den)),
            )
            .calculate_aa(memoization, position, prev_pos),
            Expression::Derivative(expr, var, order) => {
                let length = var.len()
                    + 2
                    + if *order == 1 {
                        0
                    } else {
                        order.to_string().len()
                    };

                let below_height = self.get_below_height(memoization);

                let span = var.len() / 2;

                let mut pos = prev_pos;

                let new_height = pos.0 + below_height;
                pos.0 = new_height - (1 + if *order == 1 { 0 } else { 1 });

                // d
                position.push(("d".to_string(), pos));
                pos.1 += 2;
                // d var
                for (i, c) in var.chars().enumerate() {
                    position.push((c.to_string(), (pos.0, pos.1 + i)));
                }
                if *order != 1 {
                    //     order
                    // d var
                    pos.1 += var.len();
                    pos.0 += 1;
                    for (i, c) in order.to_string().chars().enumerate() {
                        position.push((c.to_string(), (pos.0, pos.1 + i)));
                    }
                    pos.1 -= var.len();
                }
                pos.1 -= 2;

                pos.0 += 1;
                // ---------
                //     order
                // d var
                for _ in 0..length {
                    position.push(("-".to_string(), pos));
                    pos.1 += 1;
                }

                pos.1 -= length;
                pos.0 += 1;
                //    d
                // ---------
                //     order
                // d var
                pos.1 += span;
                position.push(("d".to_string(), pos));
                if *order != 1 {
                    //    order
                    //   d
                    // ----------
                    //      order
                    // d var
                    pos.1 += 1;
                    pos.0 += 1;
                    for (i, c) in order.to_string().chars().enumerate() {
                        position.push((c.to_string(), (pos.0, pos.1 + i)));
                    }
                    pos.1 -= 1;
                    pos.0 -= 1;
                }
                pos.1 -= span;
                //    order
                //   d
                // ----------  expr
                //      order
                // d var
                let height = new_height - expr.get_below_height(memoization);
                expr.calculate_aa(memoization, position, (height, pos.1 + length + 1));
            }
        }
    }

    fn get_below_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Expression::Addition(expressions) | Expression::Multiplication(expressions) => {
                expressions
                    .iter()
                    .map(|x| x.get_below_height(memoization))
                    .max()
                    .unwrap_or(0)
            }
            Expression::Division(_d, n) => n.get_height(memoization),
            Expression::Exponentiation(b, _p) => b.get_below_height(memoization),
            Expression::Subtraction(a, b)
            | Expression::Equality(a, b)
            | Expression::Complex(a, b) => a
                .get_below_height(memoization)
                .max(b.get_below_height(memoization)),
            Expression::Variable(_)
            | Expression::Constant(_)
            | Expression::Number(Numeral::Integer(_)) => 0,
            Expression::Number(Numeral::Rational(_num, _den)) => 1,
            Expression::Function(_, expressions) => expressions
                .iter()
                .map(|x| x.get_below_height(memoization))
                .max()
                .unwrap_or(0),
            Expression::Negation(expression) => expression.get_below_height(memoization),
            Expression::Derivative(expr, _var, order) => expr
                .get_below_height(memoization)
                .max(1 + if *order == 1 { 0 } else { 1 }),
        }
    }

    fn get_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        if let Some((height, _length)) = memoization.get(self) {
            if *height != 0 {
                return *height;
            }
        }

        let height = match self {
            Expression::Addition(expressions) | Expression::Multiplication(expressions) => {
                let mut max_top_height = 0;
                let mut max_below_height = 0;

                expressions.iter().for_each(|x| {
                    let x_height = x.get_height(memoization);
                    let x_below = x.get_below_height(memoization);

                    max_top_height = (x_height - x_below).max(max_top_height);
                    max_below_height = x_below.max(max_below_height);
                });

                max_top_height + max_below_height
            }
            Expression::Division(d, n) => d.get_height(memoization) + n.get_height(memoization) + 1,
            Expression::Exponentiation(b, p) => {
                b.get_height(memoization) + p.get_height(memoization)
            }
            Expression::Subtraction(a, b)
            | Expression::Equality(a, b)
            | Expression::Complex(a, b) => a.get_height(memoization).max(b.get_height(memoization)),
            Expression::Variable(_)
            | Expression::Constant(_)
            | Expression::Number(Numeral::Integer(_)) => 1,
            Expression::Number(Numeral::Rational(_num, _den)) => 3,
            Expression::Function(_, expressions) => expressions
                .iter()
                .map(|x| x.get_height(memoization))
                .max()
                .unwrap_or(1),
            Expression::Negation(expression) => expression.get_height(memoization),
            Expression::Derivative(expr, _var, order) => {
                let der_below = 1 + if *order == 1 { 0 } else { 1 };
                let expr_below = expr.get_below_height(memoization);

                let top = (3 + if *order == 1 { 0 } else { 2 } - der_below)
                    .max(expr.get_height(memoization) - expr_below);
                if der_below > expr_below {
                    der_below + top
                } else {
                    expr_below + top
                }
            }
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
            Expression::Addition(expressions) | Expression::Multiplication(expressions) => {
                let length = expressions
                    .iter()
                    .map(|x| x.get_length(memoization))
                    .sum::<usize>();

                expressions.len() * 3 - 3 + length
            }
            Expression::Division(d, n) => d.get_length(memoization).max(n.get_length(memoization)),
            Expression::Exponentiation(b, p) => {
                b.get_length(memoization)
                    + p.get_length(memoization)
                    + if matches!(
                        **b,
                        Expression::Addition(_)
                            | Expression::Multiplication(_)
                            | Expression::Exponentiation(_, _)
                            | Expression::Complex(_, _)
                            | Expression::Division(_, _)
                    ) {
                        2
                    } else {
                        0
                    }
            }
            Expression::Equality(a, b) | Expression::Subtraction(a, b) => {
                a.get_length(memoization) + 3 + b.get_length(memoization)
            }
            Expression::Complex(a, b) => a.get_length(memoization) + b.get_length(memoization) + 4,
            Expression::Variable(var) => var.len(),
            Expression::Constant(constant) => match constant {
                constant::Constant::Pi => 2,
                constant::Constant::E => 1,
                constant::Constant::Tau => 3,
            },
            Expression::Number(Numeral::Integer(num)) => num.to_string().len(),
            Expression::Number(Numeral::Rational(num, den)) => Expression::Division(
                Box::new(Expression::integer(*num)),
                Box::new(Expression::integer(*den)),
            )
            .get_length(memoization),
            Expression::Function(func, expressions) => {
                expressions
                    .iter()
                    .map(|x| x.get_length(memoization))
                    .sum::<usize>()
                    + func.get_length()
                    + 2
            }
            Expression::Negation(expression) => expression.get_length(memoization) + 2,
            Expression::Derivative(expr, var, order) => {
                expr.get_length(memoization)
                    + var.len()
                    + 3
                    + if *order == 1 {
                        0
                    } else {
                        order.to_string().len()
                    }
            }
        };

        if let Some((_h, l)) = memoization.get_mut(self) {
            *l = length;
        } else {
            memoization.insert(self.clone(), (0, length));
        }

        length
    }
}
