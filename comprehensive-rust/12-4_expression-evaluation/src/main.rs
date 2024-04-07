/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;

            match op {
                Operation::Add => left.checked_add(right).ok_or_else(|| String::from("integer overflow")),
                Operation::Sub => left.checked_sub(right).ok_or_else(|| String::from("integer overflow")),
                Operation::Mul => left.checked_mul(right).ok_or_else(|| String::from("integer overflow")),
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("division by zero"));
                    }

                    left.checked_div(right).ok_or_else(|| String::from("integer overflow"))
                }
            }
        }
        Expression::Value(val) => Ok(val),
    }
}


fn main() {
    unimplemented!();
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

#[test]
fn test_integer_overflow() {
    // ADD
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(i64::MAX)),
            right: Box::new(Expression::Value(4)),
        }),
        Err(String::from("integer overflow"))
    );
    // SUB
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(i64::MIN)),
            right: Box::new(Expression::Value(1)),
        }),
        Err(String::from("integer overflow"))
    );
    // MUL
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(i64::MAX)),
            right: Box::new(Expression::Value(2)),
        }),
        Err(String::from("integer overflow"))
    );
    //DIV
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(i64::MIN)),
            right: Box::new(Expression::Value(-1)),
        }),
        Err(String::from("integer overflow"))
    );
}