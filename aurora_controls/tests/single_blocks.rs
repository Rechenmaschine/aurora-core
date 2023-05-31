use aurora_controls::block::Block;
use aurora_controls::blocks::add::{Adder};
use aurora_controls::blocks::multiply::{self, Multiplier};
use aurora_controls::blocks::negate::{self, Negator};
use std::collections::LinkedList;

#[test]
fn int_array_adder() {
    let mut adder = Adder::new();
    let input = [2, -3, 0, 5, 4];
    
    let result: i32 = adder.step(input);
    let expect = 8;

    assert_eq!(result, expect, "Adder on [i32]; expected {} but got {}", expect, result);
}

#[test]
fn float_slice_adder() {
    let mut adder = Adder::new();
    let input = [2.1, -3.4, 0.0, 5.8, 4.9];
    
    let result: f64 = adder.step(&input[1..4]);
    let expect = 2.4;

    assert_eq!(result, expect, "Adder on &[f64]; expected {} but got {}", expect, result);
}

#[test]
fn uint_vec_multiplier() {
    let mut multiplier = Multiplier::new();
    let mut input: Vec<u64> = Vec::new();
    input.extend([1,2,3,4,5]);
    
    let result: u64 = multiplier.step(input);
    let expect: u64 = 120;

    assert_eq!(result, expect, "Multiplier on Vec<u64>; expected {} but got {}", expect, result);
}

#[test]
fn int_ref_linked_list_negator() {
    let mut negator = Negator::new();
    let input = LinkedList::from([&4, &2, &-3, &0]);

    let result: Vec<i32> = negator.step(input);
    let expect = Vec::from([-4, -2, 3, 0]);

    assert_eq!(result, expect, "Multiplier on LinkedList<i32>; expected {:?} but got {:?}", expect, result);
}