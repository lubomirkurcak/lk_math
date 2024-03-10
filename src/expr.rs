use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub enum Expr<T> {
    Add(Box<Expr<T>>, Box<Expr<T>>),
    Sub(Box<Expr<T>>, Box<Expr<T>>),
    Mul(Box<Expr<T>>, Box<Expr<T>>),
    Div(Box<Expr<T>>, Box<Expr<T>>),
    Eq(Box<Expr<T>>, Box<Expr<T>>),
    Ident(String),
    Const(T),
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalError {
    CannotEvalFreeExpr,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::CannotEvalFreeExpr => writeln!(f, "EvalError::CannotEvalFreeExpr"),
        }
    }
}

impl std::error::Error for EvalError {}

impl<T> Expr<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: FromStr,
    T: Copy,
    T: PartialEq,
    T: From<bool>,
    T: Debug,
{
    pub fn eval(&self, vals: &HashMap<String, Expr<T>>) -> Result<T, EvalError> {
        match self {
            Expr::Add(a, b) => Ok(a.eval(vals)? + b.eval(vals)?),
            Expr::Sub(a, b) => Ok(a.eval(vals)? - b.eval(vals)?),
            Expr::Mul(a, b) => Ok(a.eval(vals)? * b.eval(vals)?),
            Expr::Div(a, b) => Ok(a.eval(vals)? / b.eval(vals)?),
            Expr::Eq(a, b) => Ok((a.eval(vals)? == b.eval(vals)?).into()),
            Expr::Ident(ident) => Ok(vals.get(ident).unwrap().eval(vals)?),
            Expr::Const(val) => Ok(*val),
            Expr::Free => Err(EvalError::CannotEvalFreeExpr),
        }
    }

    pub fn solve(&self, result: T, vals: &HashMap<String, Expr<T>>) -> HashMap<String, T> {
        let mut forced = HashMap::new();
        self.solve_internal(None, result, vals, &mut forced);
        forced
    }

    fn solve_internal(
        &self,
        my_ident: Option<&str>,
        result: T,
        vals: &HashMap<String, Expr<T>>,
        forced: &mut HashMap<String, T>,
    ) {
        match self {
            Expr::Add(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a + b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = result - a_val.unwrap();
                    assert_eq!(result, a_val.unwrap() + b_val);
                    b.solve_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result - b_val.unwrap();
                    assert_eq!(result, a_val + b_val.unwrap());
                    a.solve_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expr::Sub(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a - b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = a_val.unwrap() - result;
                    assert_eq!(result, a_val.unwrap() - b_val);
                    b.solve_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result + b_val.unwrap();
                    assert_eq!(result, a_val - b_val.unwrap());
                    a.solve_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expr::Mul(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a * b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = result / a_val.unwrap();
                    assert_eq!(result, a_val.unwrap() * b_val);
                    b.solve_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result / b_val.unwrap();
                    assert_eq!(result, a_val * b_val.unwrap());
                    a.solve_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expr::Div(a, b) => {
                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                // result = a / b
                if a_val.is_ok() && b_val.is_err() {
                    let b_val = a_val.unwrap() / result;
                    assert_eq!(result, a_val.unwrap() / b_val);
                    b.solve_internal(None, b_val, vals, forced);
                } else if a_val.is_err() && b_val.is_ok() {
                    let a_val = result * b_val.unwrap();
                    assert_eq!(result, a_val / b_val.unwrap());
                    a.solve_internal(None, a_val, vals, forced);
                } else {
                    panic!();
                }
            }
            Expr::Eq(a, b) => {
                // NOTE(lubo): Only enforcing equality is supported!
                assert_eq!(result, true.into());

                let a_val = a.eval(vals);
                let b_val = b.eval(vals);

                match (a_val, b_val) {
                    (Ok(a_val), Err(_)) => b.solve_internal(None, a_val, vals, forced),
                    (Err(_), Ok(b_val)) => a.solve_internal(None, b_val, vals, forced),
                    _ => panic!(),
                }
            }
            Expr::Ident(ident) => {
                vals.get(ident)
                    .unwrap()
                    .solve_internal(Some(ident), result, vals, forced);
            }
            Expr::Const(c) => {
                assert_eq!(c, &result);
            }
            Expr::Free => {
                assert!(my_ident.is_some());
                let my_ident = my_ident.unwrap().to_string();

                #[allow(clippy::map_entry)]
                // #[allow(
                //     clippy::map_entry,
                //     reason = "entry does not allow key by reference, see: https://github.com/rust-lang/rfcs/pull/1769"
                // )]
                if forced.contains_key(&my_ident) {
                    assert_eq!(forced.get(&my_ident).unwrap(), &result);
                } else {
                    forced.insert(my_ident, result);
                }
            }
        }
    }
}

impl<T> Expr<T>
where
    T: FromStr,
{
    fn from_str_custom(s: &str) -> Self {
        let s = s.trim();

        fn splitwrap<U: FromStr, F>(s: &str, i: usize, f: F) -> Expr<U>
        where
            F: Fn(Box<Expr<U>>, Box<Expr<U>>) -> Expr<U>,
        {
            let split = s.split_at(i);
            let a = Expr::from_str_custom(split.0);
            let b = Expr::from_str_custom(&split.1[1..]);
            f(Box::new(a), Box::new(b))
        }

        if let Some(i) = s.find('=') {
            return splitwrap(s, i, Expr::Eq);
        }

        if let Some(i) = s.find('+') {
            return splitwrap(s, i, Expr::Add);
        }

        if let Some(i) = s.find('-') {
            return splitwrap(s, i, Expr::Sub);
        }

        if let Some(i) = s.find('*') {
            return splitwrap(s, i, Expr::Mul);
        }

        if let Some(i) = s.find('/') {
            return splitwrap(s, i, Expr::Div);
        }

        match s.parse::<T>() {
            Ok(val) => Self::Const(val),
            Err(_) => Self::Ident(s.to_string()),
        }
    }
}

impl<T: Clone + FromStr> FromStr for Expr<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_custom(s))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::expr::EvalError;

    use super::Expr;

    #[test]
    fn eval_0() {
        let no_vals = HashMap::new();
        assert_eq!(Expr::Const(0).eval(&no_vals), Ok(0));
    }

    #[test]
    fn eval_free() {
        let no_vals = HashMap::new();
        assert_eq!(
            Expr::<i8>::Free.eval(&no_vals),
            Err(EvalError::CannotEvalFreeExpr)
        );
    }

    #[test]
    fn parseval_2_plus_2() {
        let no_vals = HashMap::new();
        assert_eq!("2+2".parse::<Expr<_>>().unwrap().eval(&no_vals), Ok(4));
    }

    #[test]
    fn parseval_2_minus_2() {
        let no_vals = HashMap::new();
        assert_eq!("2-2".parse::<Expr<_>>().unwrap().eval(&no_vals), Ok(0));
    }

    #[test]
    fn parseval_2_times_2() {
        let no_vals = HashMap::new();
        assert_eq!("2*2".parse::<Expr<_>>().unwrap().eval(&no_vals), Ok(4));
    }

    #[test]
    fn parseval_2_div_2() {
        let no_vals = HashMap::new();
        assert_eq!("2/2".parse::<Expr<_>>().unwrap().eval(&no_vals), Ok(1));
    }

    #[test]
    fn parse_ident_equals_const() {
        assert_eq!(
            "x=1".parse::<Expr<i8>>().unwrap(),
            Expr::Eq(Box::new(Expr::Ident("x".into())), Box::new(Expr::Const(1)))
        );
    }

    // TODO(lubo): This will need a refactor to unite Free and Ident
    // #[test]
    // fn solve() {
    //     let no_vals = HashMap::new();
    //     let expr = Expr::Eq(Box::new(Expr::Ident("x".into())), Box::new(Expr::Const(1)));
    //     let solution = expr.solve(true.into(), &no_vals);
    //     assert_eq!(solution, HashMap::from([("x".into(), 1)]));
    // }
}
