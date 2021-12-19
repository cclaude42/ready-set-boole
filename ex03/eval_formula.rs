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

fn main() {
    println!("{}", eval_formula("10&"));
    // false
    println!("{}", eval_formula("10|"));
    // true
    println!("{}", eval_formula("11>"));
    // true
    println!("{}", eval_formula("10="));
    // false
    println!("{}", eval_formula("1011||="));
    // true
    println!("");

    println!("01& : {}", eval_formula("01&"));
    println!("11& : {}", eval_formula("11&"));
    println!("01| : {}", eval_formula("01|"));
    println!("00| : {}", eval_formula("00|"));
    println!("10^ : {}", eval_formula("10^"));
    println!("11^ : {}", eval_formula("11^"));
    println!("0!  : {}", eval_formula("0!"));
    println!("1!  : {}", eval_formula("1!"));
    println!("01> : {}", eval_formula("01>"));
    println!("10> : {}", eval_formula("10>"));
    println!("00= : {}", eval_formula("00="));
    println!("01= : {}", eval_formula("01="));
    println!("");

    println!("( (!0) || (1 ^ 1) ) && ( (1 -> 0) == (0) ) : {}", eval_formula("0!11^|10>0=&"));
    println!("( (!0) || (1 ^ 1) ) && ( (1 -> 0) == (1) ) : {}", eval_formula("0!11^|10>1=&"));
    println!("( (!0) || (1 ^ 1) ) && ( (1 -> 1) == (1) ) : {}", eval_formula("0!11^|11>1=&"));
    println!("( (!1) || (1 ^ 1) ) && ( (1 -> 1) == (1) ) : {}", eval_formula("1!11^|11>1=&"));
    println!("");
}