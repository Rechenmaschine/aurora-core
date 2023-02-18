use aurora_hal;



#[test]
fn evaluate_val() {
    let x: u64 = 1;
    let val = aurora_hal::Expression::new_val(x);
    if let aurora_hal::Val::Uinteger(res) = val.evaluate().unwrap(){
        assert_eq!(res, 1);
    }}




//*******************************************************************
// Tests for testing unsigned integer operators
//*******************************************************************


#[test]
fn one_plus_one_u64() {
    let mut expr = aurora_hal::Expression::new();
    let one: u64 = 1;
    let one1 = aurora_hal::Expression::new_val(one);
    let one2 = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Addition);
    expr.x = Box::new(Some(one1));
    expr.y = Box::new(Some(one2));

    if let aurora_hal::Val::Uinteger(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 2);
    }
}

#[test]
fn two_minus_one_u64() {
    let mut expr = aurora_hal::Expression::new();
    let one: u64 = 1;
    let two: u64 = 2;
    let two = aurora_hal::Expression::new_val(two);
    let one = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Subtraction);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(one));

    if let aurora_hal::Val::Uinteger(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 1);
    }
}


#[test]
fn two_times_three_u64() {
    let mut expr = aurora_hal::Expression::new();
    let three: u64 = 3;
    let two: u64 = 2;
    let two = aurora_hal::Expression::new_val(two);
    let three = aurora_hal::Expression::new_val(three);

    expr.op = Some(aurora_hal::Operator::Multiplication);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(three));

    if let aurora_hal::Val::Uinteger(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 6);
    }
}


#[test]
fn six_divided_by_two_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let two: u64 = 2;
    let six = aurora_hal::Expression::new_val(six);
    let two = aurora_hal::Expression::new_val(two);

    expr.op = Some(aurora_hal::Operator::Division);
    expr.x = Box::new(Some(six));
    expr.y = Box::new(Some(two));

    if let aurora_hal::Val::Uinteger(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 3);
    }
}


#[test]
fn six_greater_than_3() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let three: u64 = 3;
    let six = aurora_hal::Expression::new_val(six);
    let three = aurora_hal::Expression::new_val(three);

    expr.op = Some(aurora_hal::Operator::GreaterThan);
    expr.x = Box::new(Some(six));
    expr.y = Box::new(Some(three));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}


#[test]
fn three_greater_than_six_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let three: u64 = 3;
    let three = aurora_hal::Expression::new_val(three);
    let six = aurora_hal::Expression::new_val(six);

    expr.op = Some(aurora_hal::Operator::GreaterThan);
    expr.x = Box::new(Some(three));
    expr.y = Box::new(Some(six));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, false);
    }
}


#[test]
fn six_greater_equal_three_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let three: u64 = 3;
    let three = aurora_hal::Expression::new_val(three);
    let six = aurora_hal::Expression::new_val(six);

    expr.op = Some(aurora_hal::Operator::GreaterOrEqual);
    expr.x = Box::new(Some(six));
    expr.y = Box::new(Some(three));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}


#[test]
fn three_less_than_six_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let three: u64 = 3;
    let three = aurora_hal::Expression::new_val(three);
    let six = aurora_hal::Expression::new_val(six);

    expr.op = Some(aurora_hal::Operator::LessThan);
    expr.x = Box::new(Some(three));
    expr.y = Box::new(Some(six));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}


#[test]
fn three_less_equal_six_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let three: u64 = 3;
    let three = aurora_hal::Expression::new_val(three);
    let six = aurora_hal::Expression::new_val(six);

    expr.op = Some(aurora_hal::Operator::LessOrEqual);
    expr.x = Box::new(Some(three));
    expr.y = Box::new(Some(six));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}


#[test]
fn six_equal_six_u64() {
    let mut expr = aurora_hal::Expression::new();
    let six: u64 = 6;
    let six1 = aurora_hal::Expression::new_val(six);
    let six2 = aurora_hal::Expression::new_val(six);

    expr.op = Some(aurora_hal::Operator::Equal);
    expr.x = Box::new(Some(six1));
    expr.y = Box::new(Some(six2));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}

//*******************************************************************
// Tests for signed integer operators
//*******************************************************************


#[test]
fn one_plus_one_i64() {
    let mut expr = aurora_hal::Expression::new();
    let one: i64 = 1;
    let one1 = aurora_hal::Expression::new_val(one);
    let one2 = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Addition);
    expr.x = Box::new(Some(one1));
    expr.y = Box::new(Some(one2));

    if let aurora_hal::Val::Integer(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 2);
    }
}

#[test]
fn two_minus_one_i64() {
    let mut expr = aurora_hal::Expression::new();
    let one: i64 = 1;
    let two: i64 = 2;
    let two = aurora_hal::Expression::new_val(two);
    let one = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Subtraction);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(one));

    if let aurora_hal::Val::Integer(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 1);
    }
}


#[test]
fn two_times_three_i64() {
    let mut expr = aurora_hal::Expression::new();
    let three: i64 = 3;
    let two: i64 = 2;
    let two = aurora_hal::Expression::new_val(two);
    let three = aurora_hal::Expression::new_val(three);

    expr.op = Some(aurora_hal::Operator::Multiplication);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(three));

    if let aurora_hal::Val::Integer(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 6);
    }
}


#[test]
fn six_divided_by_two_i64() {
    let mut expr = aurora_hal::Expression::new();
    let six: i64 = 6;
    let two: i64 = 2;
    let six = aurora_hal::Expression::new_val(six);
    let two = aurora_hal::Expression::new_val(two);

    expr.op = Some(aurora_hal::Operator::Division);
    expr.x = Box::new(Some(six));
    expr.y = Box::new(Some(two));

    if let aurora_hal::Val::Integer(res) = expr.evaluate().unwrap(){
        assert_eq!(res, 3);
    }
}

//*******************************************************************
// Tests for testing float operators
//*******************************************************************


#[test]
fn one_plus_one_f64() {
    let mut expr = aurora_hal::Expression::new();
    let one: f64 = 1.0;
    let one1 = aurora_hal::Expression::new_val(one);
    let one2 = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Addition);
    expr.x = Box::new(Some(one1));
    expr.y = Box::new(Some(one2));

    if let aurora_hal::Val::Float(res) = expr.evaluate().unwrap(){
        assert!((res < 2.0001) && (res > 1.9999))
    }
}

#[test]
fn two_minus_one_f64() {
    let mut expr = aurora_hal::Expression::new();
    let one: f64 = 1.0;
    let two: f64 = 2.0;
    let two = aurora_hal::Expression::new_val(two);
    let one = aurora_hal::Expression::new_val(one);

    expr.op = Some(aurora_hal::Operator::Subtraction);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(one));

    if let aurora_hal::Val::Float(res) = expr.evaluate().unwrap(){
        assert!((res < 1.0001) && (res > 0.9999));
    }
}


#[test]
fn two_times_three_f64() {
    let mut expr = aurora_hal::Expression::new();
    let three: f64 = 3.0;
    let two: f64 = 2.0;
    let two = aurora_hal::Expression::new_val(two);
    let three = aurora_hal::Expression::new_val(three);

    expr.op = Some(aurora_hal::Operator::Multiplication);
    expr.x = Box::new(Some(two));
    expr.y = Box::new(Some(three));

    if let aurora_hal::Val::Float(res) = expr.evaluate().unwrap(){
        assert!((res < 6.0001) && (res > 5.9999));
    }
}


#[test]
fn six_divided_by_two_f64() {
    let mut expr = aurora_hal::Expression::new();
    let six: f64 = 6.0;
    let two: f64 = 2.0;
    let six = aurora_hal::Expression::new_val(six);
    let two = aurora_hal::Expression::new_val(two);

    expr.op = Some(aurora_hal::Operator::Division);
    expr.x = Box::new(Some(six));
    expr.y = Box::new(Some(two));

    if let aurora_hal::Val::Float(res) = expr.evaluate().unwrap(){
        assert!((res < 3.0001) && (res > 2.9999));
    }
}

//*******************************************************************
// Tests for testing logical operators
//*******************************************************************

#[test]
fn true_and_true() {
    let mut expr = aurora_hal::Expression::new();
    let true_base: bool = true;
    let true1 = aurora_hal::Expression::new_val(true_base);
    let true2 = aurora_hal::Expression::new_val(true_base);

    expr.op = Some(aurora_hal::Operator::LogicalAnd);
    expr.x = Box::new(Some(true1));
    expr.y = Box::new(Some(true2));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    } else {
        panic!("Evaluation returned the wrong type")
    }
}

#[test]
fn true_or_false() {
    let mut expr = aurora_hal::Expression::new();
    let true_base: bool = true;
    let false_base: bool = false;
    let true1 = aurora_hal::Expression::new_val(true_base);
    let true2 = aurora_hal::Expression::new_val(false_base);

    expr.op = Some(aurora_hal::Operator::LogicalOr);
    expr.x = Box::new(Some(true1));
    expr.y = Box::new(Some(true2));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, true);
    } else {
        panic!("Evaluation returned the wrong type")
    }
}


#[test]
fn not_true() {
    let mut expr = aurora_hal::Expression::new();
    let true_base: bool = true;
    let true1 = aurora_hal::Expression::new_val(true_base);

    expr.op = Some(aurora_hal::Operator::LogicalNot);
    expr.x = Box::new(Some(true1));

    if let aurora_hal::Val::Bool(res) = expr.evaluate().unwrap(){
        assert_eq!(res, false);
    } else {
        panic!("Evaluation returned the wrong type")
    }
}



//*******************************************************************
// Tests for more complex expression trees
//*******************************************************************

#[test]
fn expression_1() {
    let mut expr1 = aurora_hal::Expression::new();
    let mut expr2 = aurora_hal::Expression::new();
    let mut expr3 = aurora_hal::Expression::new();

    let one: u64 = 1;
    let four: u64 = 4;
    let five: u64 = 5;
    let ten: u64 = 10;
    let one = aurora_hal::Expression::new_val(one);
    let five = aurora_hal::Expression::new_val(five);
    let four = aurora_hal::Expression::new_val(four);
    let ten = aurora_hal::Expression::new_val(ten);

    expr1.op = Some(aurora_hal::Operator::Addition);
    expr1.x = Box::new(Some(one));
    expr1.y = Box::new(Some(five));

    expr2.op = Some(aurora_hal::Operator::Multiplication);
    expr2.x = Box::new(Some(four));
    expr2.y = Box::new(Some(expr1));

    expr3.op = Some(aurora_hal::Operator::Subtraction);
    expr3.x = Box::new(Some(expr2));
    expr3.y = Box::new(Some(ten));

    if let aurora_hal::Val::Uinteger(res) = expr3.evaluate().unwrap(){
        assert_eq!(res, 14);
    }
}


#[test]
fn expression_2() {
    // ((5+1) == 4) || (11 > 10) = true

    let mut expr1 = aurora_hal::Expression::new();
    let mut expr2 = aurora_hal::Expression::new();
    let mut expr3 = aurora_hal::Expression::new();
    let mut expr4 = aurora_hal::Expression::new();

    let one: u64 = 1;
    let four: u64 = 4;
    let five: u64 = 5;
    let ten: u64 = 10;
    let eleven: u64 = 11;
    let one = aurora_hal::Expression::new_val(one);
    let five = aurora_hal::Expression::new_val(five);
    let four = aurora_hal::Expression::new_val(four);
    let ten = aurora_hal::Expression::new_val(ten);
    let eleven = aurora_hal::Expression::new_val(eleven);

    expr1.op = Some(aurora_hal::Operator::Addition);
    expr1.x = Box::new(Some(one));
    expr1.y = Box::new(Some(five));

    expr2.op = Some(aurora_hal::Operator::Equal);
    expr2.x = Box::new(Some(four));
    expr2.y = Box::new(Some(expr1));

    expr3.op = Some(aurora_hal::Operator::GreaterThan);
    expr3.x = Box::new(Some(eleven));
    expr3.y = Box::new(Some(ten));

    expr4.op = Some(aurora_hal::Operator::LogicalOr);
    expr4.x = Box::new(Some(expr2));
    expr4.y = Box::new(Some(expr3));

    if let aurora_hal::Val::Bool(res) = expr4.evaluate().unwrap(){
        assert_eq!(res, true);
    }
}