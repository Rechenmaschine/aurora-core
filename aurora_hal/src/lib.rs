use std::any::Any;
use std::borrow::Borrow;
use aurora_hal_macros::add_fields;


trait WrapWithVal{
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

#[derive(Clone, Copy)]
enum Val {
    Integer(i64),
    Uinteger(u64),
    Float(f64),
    Bool(bool),
}

enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Value(Val),
}

struct Expression {
    op: Option<Operator>,
    x: Box<Option<Expression>>,
    y: Box<Option<Expression>>,
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

        if matches!(val1.clone(), val2) {
            let result = match self.op {
                Some(Operator::Addition) => {
                    match val1 {
                        Val::Float(v) => {
                            if let Val::Float(v2) = val2{
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
                        Val::Bool(v) => {
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
                        Val::Bool(v) => {
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
                        Val::Bool(v) => {
                            return Err("Can't multiply two boolean values");
                        }
                    }
                }
                Some(Operator::Division) => {
                    match val1 {
                        Val::Float(v) => {
                            if let Val::Float(v2) = val2 {
                                Val::Float(v / v2)
                            } else{
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
                        Val::Bool(v) => {
                            return Err("Can't divide two boolean values");
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
                    match val1 {
                        Val::Bool(v) => {
                            Val::Bool(!v)
                        }
                        _ => return Err("Can't invert non-boolean value with logical NOT"),
                    }
                }
                Some(Operator::Value(val)) => {
                    let v = val.clone();
                    v
                }
                None => panic!("Tried to evaluate Expression with no Operator"),
            };
            Ok(result)
        } else {
            Err("Tried to compare two different types")
        }
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

impl <T: Copy + WrapWithVal> Value<T> {
    pub fn add_callback(&mut self, callback: fn(), condition: Condition) {
        self.callbacks.push((condition, callback));
    }

    pub fn set(&mut self, val: T) {
        let arg = vec![val, self.val.clone()];
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
