#![allow(dead_code)] //Only here for development. Remove once feature is actually used?
use aurora_hal_macros::add_fields;

pub trait WrapWithVal {
    fn to_val(&self) -> Val;
}

impl WrapWithVal for u64 {
    fn to_val(&self) -> Val {
        Val::Uinteger(self.clone())
    }
}

impl WrapWithVal for i64 {
    fn to_val(&self) -> Val {
        Val::Integer(self.clone())
    }
}

impl WrapWithVal for f64 {
    fn to_val(&self) -> Val {
        Val::Float(self.clone())
    }
}

impl WrapWithVal for bool {
    fn to_val(&self) -> Val {
        Val::Bool(self.clone())
    }
}

#[derive(Clone, Copy)]
pub enum Val {
    Integer(i64),
    Uinteger(u64),
    Float(f64),
    Bool(bool),
}

pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Equal,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Value(Val),
}

pub struct Expression {
    pub op: Option<Operator>,
    pub x: Box<Option<Expression>>,
    pub y: Box<Option<Expression>>,
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            op: None,
            x: Box::new(None),
            y: Box::new(None),
        }
    }

    pub fn new_val<T: WrapWithVal>(x: T) -> Expression {
        Expression {
            op: Some(Operator::Value(x.to_val())),
            x: Box::new(None),
            y: Box::new(None),
        }
    }

    pub fn evaluate(&self) -> Result<Val, &'static str> {

        if let Some(Operator::Value(val)) = self.op {
            return Ok(val.clone());
        } else if let Some(Operator::LogicalNot) = self.op {
            return if let Some(expr1) = self.x.as_ref() {
                let val = expr1.evaluate().unwrap();
                if let Val::Bool(v) = val {
                    Ok(Val::Bool(!v))
                } else {
                    Err("Can't invert non-boolean value with logical NOT")
                }
            } else {
                Err("Can't invert non-boolean value with logical NOT")
            }
        }

        let val1 = if let Some(expr) = self.x.as_ref() {
            expr.evaluate().unwrap()
        } else {
            panic!("Tried to evaluate empty expression");
        };


        let val2 = if let Some(expr) = self.y.as_ref() {
            expr.evaluate().unwrap()
        } else {
            panic!("Tried to evaluate empty expression");
        };

        let _val2_clone = val2.clone();

        let result = match self.op {
            Some(Operator::Addition) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Float(v + v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Integer(v + v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Uinteger(v + v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't add two boolean values");
                    }
                }
            }
            Some(Operator::Subtraction) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Float(v - v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Integer(v - v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Uinteger(v - v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't subtract two boolean values");
                    }
                }
            }
            Some(Operator::Multiplication) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Float(v * v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Integer(v * v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Uinteger(v * v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't multiply two boolean values");
                    }
                }
            }
            Some(Operator::Division) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Float(v / v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Integer(v / v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Uinteger(v / v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't divide two boolean values");
                    }
                }
            }
            Some(Operator::GreaterThan) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Bool(v > v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Bool(v > v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Bool(v > v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't use greater than operator on boolean values");
                    }
                }
            }
            Some(Operator::GreaterOrEqual) => {
                match val1 {
                    Val::Float(v) => {
                        return Err("Can't use greater or equal operator on float values");
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Bool(v >= v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Bool(v >= v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't use \"greater or equal\" operator on boolean values");
                    }
                }
            }
            Some(Operator::LessThan) => {
                match val1 {
                    Val::Float(v) => {
                        if let Val::Float(v2) = val2 {
                            Val::Bool(v < v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Bool(v < v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Bool(v < v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't use less than operator on boolean values");
                    }
                }
            }
            Some(Operator::LessOrEqual) => {
                match val1 {
                    Val::Float(_v) => {
                            return Err("Can't use \"less or equal\" operator on float values");
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Bool(v <= v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Bool(v <= v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't use \"less or equal\" operator on boolean values");
                    }
                }
                }
            Some(Operator::Equal) => {
                match val1 {
                    Val::Float(v) => {
                        return Err("Can't use equal operator on float values");
                    }
                    Val::Integer(v) => {
                        if let Val::Integer(v2) = val2 {
                            Val::Bool(v == v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Uinteger(v) => {
                        if let Val::Uinteger(v2) = val2 {
                            Val::Bool(v == v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    Val::Bool(_v) => {
                        return Err("Can't use equal to operator on boolean values");
                    }
                }
            }
            Some(Operator::LogicalAnd) => {
                match val1 {
                    Val::Bool(v) => {
                        if let Val::Bool(v2) = val2 {
                            Val::Bool(v && v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    _ => return Err("Can't compare two non-boolean values with logical AND"),
                }
            }
            Some(Operator::LogicalOr) => {
                match val1 {
                    Val::Bool(v) => {
                        if let Val::Bool(v2) = val2 {
                            Val::Bool(v || v2)
                        } else {
                            return Err("Trying to evaluate expression with two non-matching values");
                        }
                    }
                    _ => return Err("Can't compare two non-boolean values with logical OR"),
                }
            }
            Some(Operator::LogicalNot) => {
                return Err("If condition didn't catch Logical Not");
            }
            Some(Operator::Value(_val)) => {
                return Err("If condition didn't catch Value");
            }
            None => panic!("Tried to evaluate Expression with no Operator"),
        };
        Ok(result)
    }
}

struct Condition {
    s: String,
    e: Expression,
}

impl Condition {
    pub fn new() -> Condition {
        Condition {
            s: String::new(),
            e: Expression::new(),
        }
    }

    pub fn parse_str(string: String) -> Condition {
        let mut c = Condition::new();
        //TODO implement String parsing so a complete Condition is created
        c.s = string;
        c
    }

    pub fn evaluate<T: WrapWithVal>(&mut self, val: T) -> bool {
        self.e.x = Box::new(Some(Expression::new_val(val)));
        if let Val::Bool(val) = self.e.evaluate().unwrap().clone() {
            return val;
        } else {
            panic!("Expression doesn't evaluate to bool");
        }
    }
}

struct Value<T> {
    val: T,
    callbacks: Vec<(Condition, fn())>,
}

impl<T: Copy + WrapWithVal> Value<T> {
    pub fn add_callback(&mut self, callback: fn(), condition: Condition) {
        self.callbacks.push((condition, callback));
    }

    pub fn set(&mut self, val: T) {
        self.val = val;

        for (ref mut condition, callback) in &mut self.callbacks {
            if condition.evaluate(self.val) == true {
                callback();
            }
        }
    }

    pub fn get(&self) -> T {
        self.val.clone()
    }
}


#[add_fields]
struct Tree {}
