use std::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(a, b) => Ok(eval(*a)? + eval(*b)?),
        Subtract(a, b) => Ok(eval(*a)? - eval(*b)?),
        Multiply(a, b) => Ok(eval(*a)? * eval(*b)?),
        Divide(a, b) => {
            let divisor = eval(*b)?;
            if divisor == 0.0 {
                return Err("Division by zero".into());
            }
            Ok(eval(*a)? / divisor)
        }
        Caret(a, b) => Ok(eval(*a)?.powf(eval(*b)?)),
        Negative(a) => Ok(-eval(*a)?),
        And(a, b) => Ok((eval(*a)? as i64 & eval(*b)? as i64) as f64),
        Or(a, b) => Ok((eval(*a)? as i64 | eval(*b)? as i64) as f64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtraction() {
        let expr = Node::Subtract(Box::new(Node::Number(10.0)), Box::new(Node::Number(4.0)));
        assert_eq!(eval(expr).unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        let expr = Node::Multiply(Box::new(Node::Number(3.0)), Box::new(Node::Number(7.0)));
        assert_eq!(eval(expr).unwrap(), 21.0);
    }

    #[test]
    fn test_negative_number() {
        let expr = Node::Negative(Box::new(Node::Number(5.0)));
        assert_eq!(eval(expr).unwrap(), -5.0);
    }

    #[test]
    fn test_nested_operations() {
        let expr = Node::Subtract(
            Box::new(Node::Add(Box::new(Node::Number(8.0)), Box::new(Node::Number(2.0)))),
            Box::new(Node::Number(3.0)),
        );
        assert_eq!(eval(expr).unwrap(), 7.0);
    }

    #[test]
    fn test_large_exponentiation() {
        let expr = Node::Caret(Box::new(Node::Number(2.0)), Box::new(Node::Number(10.0)));
        assert_eq!(eval(expr).unwrap(), 1024.0);
    }
}
