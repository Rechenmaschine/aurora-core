use aurora_hal_macros::add_fields;

struct Condition<T> {
    s: String,
    evaluate: fn(&Vec<T>) -> bool,
}

impl<T: std::cmp::PartialOrd> Condition<T> {
    pub fn new() -> Condition<T> {
        let e = |x: &Vec<T>|{false};
        Condition {
            s: String::from(""),
            evaluate: e,
        }
    }

    pub fn parse_str<>(string: String) -> Condition {
        let mut c = Condition::new();
        c.s = string;
        match c.s.as_str() {
            "<" => c.evaluate = |x: &Vec<T>| {x[0] < x[1]}, //new is smaller than old
            ">" => c.evaluate = |x: &Vec<T>| {x[0] > x[1]}, //new is bigger than old
            _ => panic!("Condition not recognized")
        }
        c
    }
}

struct Value<T> {
    val: T,
    callbacks: Vec<(Condition<T>, fn())>,
}

impl <T: Copy> Value<T> {
    pub fn add_callback(&mut self, callback: fn(), condition: Condition) {
        self.callbacks.push((condition, callback));
    }

    pub fn set(&mut self, val: T) {
        let arg = vec![val, self.val.clone()];
        self.val = val;

        for (condition, callback) in &self.callbacks {
            if condition.evaluate(&arg) == true {
                callback();
            }
        }
    }

    pub fn get(&self) -> T {
        self.val
    }
}

#[add_fields]
struct Tree {}
