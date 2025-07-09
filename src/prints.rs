use std::collections::HashMap;

use crate::ast::{Expression, constant, numeral::Numeral};

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
    /// expr.print_tree(0)
    /// ```
    /// Addition:
    ///   + Multiplication:
    ///     * x
    ///     * 2
    ///   + 5
    ///
    pub fn print_tree(&self, indent: usize) {
        println!("{}", self.calculate_tree(indent))
    }

    pub fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        match self {
            Expression::Addition(add) => {
                if add.terms.is_empty() {
                    "0".to_string()
                } else if add.terms.len() == 1 {
                    add.terms[0].calculate_tree(indent)
                } else {
                    let mut result = String::from("Addition:\n");
                    for (i, term) in add.terms.iter().enumerate() {
                        result.push_str(&format!(
                            "{}{}{}",
                            next_indent_str,
                            "+ ",
                            term.calculate_tree(next_indent)
                        ));
                        if i < add.terms.len() - 1 {
                            result.push('\n');
                        }
                    }
                    result
                }
            }
            Expression::Multiplication(mul) => {
                if mul.terms.is_empty() {
                    "1".to_string()
                } else if mul.terms.len() == 1 {
                    mul.terms[0].calculate_tree(indent)
                } else {
                    let mut result = String::from("Multiplication:\n");
                    for (i, term) in mul.terms.iter().enumerate() {
                        result.push_str(&format!(
                            "{}{}{}",
                            next_indent_str,
                            "* ",
                            term.calculate_tree(next_indent)
                        ));
                        if i < mul.terms.len() - 1 {
                            result.push('\n');
                        }
                    }
                    result
                }
            }
            Expression::Subtraction(sub) => {
                format!(
                    "Subtraction:\n{}{}\n{}- {}",
                    next_indent_str,
                    sub.left.calculate_tree(next_indent),
                    next_indent_str,
                    sub.right.calculate_tree(next_indent)
                )
            }
            Expression::Division(div) => {
                format!(
                    "Division:\n{}{}\n{}/ {}",
                    next_indent_str,
                    div.num.calculate_tree(next_indent),
                    next_indent_str,
                    div.den.calculate_tree(next_indent)
                )
            }
            Expression::Exponentiation(exp) => {
                format!(
                    "Exponentiation:\n{}{}\n{}^ {}",
                    next_indent_str,
                    exp.base.calculate_tree(next_indent),
                    next_indent_str,
                    exp.expo.calculate_tree(next_indent)
                )
            }
            Expression::Equality(equ) => {
                format!(
                    "Equality:\n{}{}\n{}= {}",
                    next_indent_str,
                    equ.left.calculate_tree(next_indent),
                    next_indent_str,
                    equ.right.calculate_tree(next_indent)
                )
            }
            Expression::Complex(com) => {
                format!(
                    "Complex:\n{}{}\n{}i {}",
                    next_indent_str,
                    com.real.calculate_tree(next_indent),
                    next_indent_str,
                    com.imag.calculate_tree(next_indent)
                )
            }
            Expression::Variable(name) => name.to_string(),
            Expression::Constant(constant) => constant.to_string(),
            Expression::Negation(neg) => {
                format!(
                    "Negation:\n{}- {}",
                    next_indent_str,
                    neg.term.calculate_tree(indent)
                )
            }
            Expression::Function(fun) => {
                let mut result = format!("{}(", fun.name);
                for (i, arg) in fun.args.iter().enumerate() {
                    result.push_str(&arg.calculate_tree(0));
                    if i < fun.args.len() - 1 {
                        result.push_str(", ");
                    }
                }
                result.push(')');
                result
            }
            Expression::Number(num) => {
                format!("{}", num)
            }
            Expression::Derivative(der) => {
                format!(
                    "Derivative{}:\n{}{}\n{}' {}",
                    if der.order > 1 {
                        format!("({})", der.order)
                    } else {
                        "".to_owned()
                    },
                    next_indent_str,
                    der.term.calculate_tree(next_indent),
                    next_indent_str,
                    der.variable,
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
        println!("{}\n", self.get_processed())
    }

    pub fn get_processed(&self) -> String {
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

    fn calculate_positions(
        &self,
        memoization: &mut HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        match self {
            Expression::Addition(add) => {
                let mut pos = prev_pos;
                let below_height = self.get_below_height(memoization);

                add.terms.iter().enumerate().for_each(|(i, x)| {
                    let new_height = pos.0 + below_height - x.get_below_height(memoization);
                    x.calculate_positions(memoization, position, (new_height, pos.1));
                    pos.1 += x.get_length(memoization);
                    if i < add.terms.len() - 1 {
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push(("+".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                    }
                });
            }
            Expression::Multiplication(mul) => {
                let mut pos = prev_pos;
                let below_height = self.get_below_height(memoization);

                mul.terms.iter().enumerate().for_each(|(i, x)| {
                    let new_height = pos.0 + below_height - x.get_below_height(memoization);
                    x.calculate_positions(memoization, position, (new_height, pos.1));
                    pos.1 += x.get_length(memoization);
                    if i < mul.terms.len() - 1 {
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push(("*".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                    }
                });
            }
            Expression::Subtraction(sub) => {
                let mut pos = prev_pos;
                sub.left.calculate_positions(memoization, position, pos);
                pos.1 += sub.left.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("-".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                sub.right.calculate_positions(memoization, position, pos);
            }
            Expression::Division(div) => {
                let length = self.get_length(memoization);
                let bottom_height = div.den.get_height(memoization);

                let bottom_length = div.den.get_length(memoization);
                let top_length = div.num.get_length(memoization);

                let (span, top) = if top_length > bottom_length {
                    ((top_length - bottom_length) / 2, false)
                } else {
                    ((bottom_length - top_length) / 2, true)
                };

                let mut pos = prev_pos;

                if !top {
                    pos.1 += span;
                    div.den.calculate_positions(memoization, position, pos);
                    pos.1 -= span;
                } else {
                    div.den.calculate_positions(memoization, position, pos);
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
                    div.num.calculate_positions(memoization, position, pos);
                    pos.1 -= span;
                } else {
                    div.num.calculate_positions(memoization, position, pos);
                }
            }
            Expression::Exponentiation(exp) => {
                let mut pos = prev_pos;
                if matches!(
                    exp.base,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_)
                        | Expression::Complex(_)
                        | Expression::Division(_)
                ) {
                    Self::calculate_parenthesis(
                        position,
                        pos,
                        true,
                        exp.base.get_height(memoization),
                    );
                    pos.1 += 1;
                }
                exp.base.calculate_positions(memoization, position, pos);
                pos.1 += exp.base.get_length(memoization);
                if matches!(
                    exp.base,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_)
                        | Expression::Complex(_)
                        | Expression::Division(_)
                ) {
                    Self::calculate_parenthesis(
                        position,
                        pos,
                        false,
                        exp.base.get_height(memoization),
                    );
                    pos.1 += 1;
                }
                pos.0 += exp.base.get_height(memoization);
                exp.expo.calculate_positions(memoization, position, pos);
            }
            Expression::Equality(equ) => {
                let mut pos = prev_pos;
                equ.left.calculate_positions(memoization, position, pos);
                pos.1 += equ.left.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("=".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                equ.right.calculate_positions(memoization, position, pos);
            }
            Expression::Complex(com) => {
                let mut pos = prev_pos;
                com.real.calculate_positions(memoization, position, pos);
                pos.1 += com.real.get_length(memoization);
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                position.push(("+".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                com.imag.calculate_positions(memoization, position, pos);
                pos.1 += com.imag.get_length(memoization);
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
            Expression::Negation(neg) => {
                let mut pos = prev_pos;
                position.push(("-".to_string(), pos));
                pos.1 += 1;
                position.push((" ".to_string(), pos));
                pos.1 += 1;
                neg.term.calculate_positions(memoization, position, pos);
            }
            Expression::Function(fun) => {
                let mut pos = prev_pos;
                let height = self.get_height(memoization);

                fun.name.to_string().chars().for_each(|c| {
                    position.push((c.to_string(), pos));
                    pos.1 += 1;
                });
                Self::calculate_parenthesis(position, pos, true, height);
                pos.1 += 1;
                fun.args.iter().for_each(|x| {
                    x.calculate_positions(memoization, position, pos);
                    pos.1 += x.get_length(memoization);
                });
                Self::calculate_parenthesis(position, pos, false, height);
            }
            Expression::Number(Numeral::Integer(num)) => {
                for (i, c) in num.to_string().chars().enumerate() {
                    position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
                }
            }
            Expression::Number(Numeral::Rational(num, den)) => {
                Expression::division(Expression::integer(*num), Expression::integer(*den))
                    .calculate_positions(memoization, position, prev_pos)
            }
            Expression::Derivative(der) => {
                let length = der.variable.len()
                    + 2
                    + if der.order == 1 {
                        0
                    } else {
                        der.order.to_string().len()
                    };

                let below_height = self.get_below_height(memoization);

                let span = der.variable.len() / 2;

                let mut pos = prev_pos;

                let new_height = pos.0 + below_height;
                pos.0 = new_height - (1 + if der.order == 1 { 0 } else { 1 });

                // d
                position.push(("d".to_string(), pos));
                pos.1 += 2;
                // d var
                for (i, c) in der.variable.chars().enumerate() {
                    position.push((c.to_string(), (pos.0, pos.1 + i)));
                }
                if der.order != 1 {
                    //     order
                    // d var
                    pos.1 += der.variable.len();
                    pos.0 += 1;
                    for (i, c) in der.order.to_string().chars().enumerate() {
                        position.push((c.to_string(), (pos.0, pos.1 + i)));
                    }
                    pos.1 -= der.variable.len();
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
                if der.order != 1 {
                    //    order
                    //   d
                    // ----------
                    //      order
                    // d var
                    pos.1 += 1;
                    pos.0 += 1;
                    for (i, c) in der.order.to_string().chars().enumerate() {
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
                let height = new_height - der.term.get_below_height(memoization);
                der.term
                    .calculate_positions(memoization, position, (height, pos.1 + length + 1));
            }
        }
    }

    fn get_below_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Expression::Addition(add) => add
                .terms
                .iter()
                .map(|x| x.get_below_height(memoization))
                .max()
                .unwrap_or(0),
            Expression::Multiplication(mul) => mul
                .terms
                .iter()
                .map(|x| x.get_below_height(memoization))
                .max()
                .unwrap_or(0),
            Expression::Division(div) => div.den.get_height(memoization),
            Expression::Exponentiation(exp) => exp.base.get_below_height(memoization),
            Expression::Subtraction(sub) => sub
                .left
                .get_below_height(memoization)
                .max(sub.right.get_below_height(memoization)),
            Expression::Equality(equ) => equ
                .left
                .get_below_height(memoization)
                .max(equ.right.get_below_height(memoization)),
            Expression::Complex(comp) => comp
                .real
                .get_below_height(memoization)
                .max(comp.imag.get_below_height(memoization)),
            Expression::Variable(_)
            | Expression::Constant(_)
            | Expression::Number(Numeral::Integer(_)) => 0,
            Expression::Number(Numeral::Rational(_num, _den)) => 1,
            Expression::Function(fun) => fun
                .args
                .iter()
                .map(|x| x.get_below_height(memoization))
                .max()
                .unwrap_or(0),
            Expression::Negation(neg) => neg.term.get_below_height(memoization),
            Expression::Derivative(der) => der
                .term
                .get_below_height(memoization)
                .max(1 + if der.order == 1 { 0 } else { 1 }),
        }
    }

    fn get_height(&self, memoization: &mut HashMap<Expression, (usize, usize)>) -> usize {
        if let Some((height, _length)) = memoization.get(self) {
            if *height != 0 {
                return *height;
            }
        }

        let height = match self {
            Expression::Addition(add) => {
                let mut max_top_height = 0;
                let mut max_below_height = 0;

                add.terms.iter().for_each(|x| {
                    let x_height = x.get_height(memoization);
                    let x_below = x.get_below_height(memoization);

                    max_top_height = (x_height - x_below).max(max_top_height);
                    max_below_height = x_below.max(max_below_height);
                });

                max_top_height + max_below_height
            }
            Expression::Multiplication(mul) => {
                let mut max_top_height = 0;
                let mut max_below_height = 0;

                mul.terms.iter().for_each(|x| {
                    let x_height = x.get_height(memoization);
                    let x_below = x.get_below_height(memoization);

                    max_top_height = (x_height - x_below).max(max_top_height);
                    max_below_height = x_below.max(max_below_height);
                });

                max_top_height + max_below_height
            }
            Expression::Division(div) => {
                div.num.get_height(memoization) + div.den.get_height(memoization) + 1
            }
            Expression::Exponentiation(exp) => {
                exp.base.get_height(memoization) + exp.expo.get_height(memoization)
            }
            Expression::Subtraction(sub) => sub
                .left
                .get_height(memoization)
                .max(sub.right.get_height(memoization)),
            Expression::Equality(equ) => equ
                .left
                .get_height(memoization)
                .max(equ.right.get_height(memoization)),
            Expression::Complex(com) => com
                .real
                .get_height(memoization)
                .max(com.imag.get_height(memoization)),
            Expression::Variable(_)
            | Expression::Constant(_)
            | Expression::Number(Numeral::Integer(_)) => 1,
            Expression::Number(Numeral::Rational(_num, _den)) => 3,
            Expression::Function(fun) => fun
                .args
                .iter()
                .map(|x| x.get_height(memoization))
                .max()
                .unwrap_or(1),
            Expression::Negation(neg) => neg.term.get_height(memoization),
            Expression::Derivative(der) => {
                let der_below = 1 + if der.order == 1 { 0 } else { 1 };
                let expr_below = der.term.get_below_height(memoization);

                let top = (3 + if der.order == 1 { 0 } else { 2 } - der_below)
                    .max(der.term.get_height(memoization) - expr_below);
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
            Expression::Addition(add) => {
                let length = add
                    .terms
                    .iter()
                    .map(|x| x.get_length(memoization))
                    .sum::<usize>();

                add.terms.len() * 3 - 3 + length
            }
            Expression::Multiplication(mul) => {
                let length = mul
                    .terms
                    .iter()
                    .map(|x| x.get_length(memoization))
                    .sum::<usize>();

                mul.terms.len() * 3 - 3 + length
            }
            Expression::Division(div) => div
                .den
                .get_length(memoization)
                .max(div.num.get_length(memoization)),
            Expression::Exponentiation(exp) => {
                exp.base.get_length(memoization)
                    + exp.expo.get_length(memoization)
                    + if matches!(
                        exp.base,
                        Expression::Addition(_)
                            | Expression::Multiplication(_)
                            | Expression::Exponentiation(_)
                            | Expression::Complex(_)
                            | Expression::Division(_)
                    ) {
                        2
                    } else {
                        0
                    }
            }
            Expression::Equality(equ) => {
                equ.left.get_length(memoization) + 3 + equ.right.get_length(memoization)
            }
            Expression::Subtraction(sub) => {
                sub.left.get_length(memoization) + 3 + &sub.right.get_length(memoization)
            }
            Expression::Complex(com) => {
                com.real.get_length(memoization) + com.imag.get_length(memoization) + 4
            }
            Expression::Variable(var) => var.len(),
            Expression::Constant(constant) => match constant {
                constant::Constant::Pi => 2,
                constant::Constant::E => 1,
                constant::Constant::Tau => 3,
            },
            Expression::Number(Numeral::Integer(num)) => num.to_string().len(),
            Expression::Number(Numeral::Rational(num, den)) => {
                Expression::division(Expression::integer(*num), Expression::integer(*den))
                    .get_length(memoization)
            }
            Expression::Function(fun) => {
                fun.args
                    .iter()
                    .map(|x| x.get_length(memoization))
                    .sum::<usize>()
                    + fun.name.get_length()
                    + 2
            }
            Expression::Negation(neg) => neg.term.get_length(memoization) + 2,
            Expression::Derivative(der) => {
                der.term.get_length(memoization)
                    + der.variable.len()
                    + 3
                    + if der.order == 1 {
                        0
                    } else {
                        der.order.to_string().len()
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
