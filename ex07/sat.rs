use std::collections::LinkedList;

fn eval_formula(formula: &str) -> bool {
    let mut stack : LinkedList<bool> = LinkedList::new();
    let mut a : bool;
    let mut b : bool;

    for c in formula.chars() { 
        if c == '0' {
            stack.push_back(false);
        }
        else if c == '1' {
            stack.push_back(true);
        }
        else if c == '!' {
            if stack.len() < 1 {
                panic!("invalid input");
            }
            a = stack.pop_back().unwrap();

            stack.push_back(!a);
        }
        else if "&|^>=".contains(c) {
            if stack.len() < 2 {
                panic!("invalid input");
            }
            b = stack.pop_back().unwrap();
            a = stack.pop_back().unwrap();

            if c == '&' {
                stack.push_back(a & b);
            }
            else if c == '|' {
                stack.push_back(a | b);
            }
            else if c == '^' {
                stack.push_back(a ^ b);
            }
            else if c == '>' {
                stack.push_back(!(a && !b));
            }
            else if c == '=' {
                stack.push_back(a == b);
            }
        }
        else {
            panic!("invalid character");
        }
    }

    if stack.len() != 1 {
        panic!("invalid input");
    }

    return stack.pop_back().unwrap();
}

fn sat(formula: &str) -> bool {
    let mut vars = String::from("");
    let mut possibilities : u32 = 1;
    
    for c in formula.chars() {
        if c.is_ascii_uppercase() && !vars.contains(c) {
            vars.push(c);
            possibilities = possibilities * 2;
        }
    }

    if vars.len() == 0 {
        if eval_formula(formula) {
            return true;
        }
        else {
            return false;
        }
    }

    for case in 0..possibilities {
        let mut bit : u32 = 1 << (vars.len() - 1);
        let mut copy = String::from(formula);

        for var in vars.chars() {
            if case & bit != 0 {
                copy = copy.replace(var, "1");
            }
            else {
                copy = copy.replace(var, "0");
            }
            bit = bit >> 1;
        }

        if eval_formula(copy.as_str()) {
            return true;
        }
    }

    return false;
}

fn main() {
    println!("{}", sat("AB|"));
    // true
    println!("{}", sat("AB&"));
    // true
    println!("{}", sat("AA!&"));
    // false
    println!("{}", sat("AA^"));
    // false
}