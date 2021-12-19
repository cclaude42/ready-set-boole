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

fn print_truth_table(formula: &str) {
    let mut vars = String::from("");
    let mut row = String::from("| ");
    let mut possibilities : u32 = 1;
    
    for c in formula.chars() {
        if c.is_ascii_uppercase() && !vars.contains(c) {
            vars.push(c);
            row.push(c);
            row += " | ";
            possibilities = possibilities * 2;
        }
    }

    row += "= |";
    println!("{}", row);

    row = String::from("|");
    for _n in 0..=vars.len() {
        row += "---|";
    }
    println!("{}", row);

    if vars.len() == 0 {
        if eval_formula(formula) {
            println!("| 1 |");
        }
        else {
            println!("| 0 |");
        }
        return ;
    }

    for case in 0..possibilities {
        let mut bit : u32 = 1 << (vars.len() - 1);
        let mut copy = String::from(formula);
        row = String::from("| ");

        for var in vars.chars() {
            if case & bit != 0 {
                copy = copy.replace(var, "1");
                row += "1 | ";
            }
            else {
                copy = copy.replace(var, "0");
                row += "0 | ";
            }
            bit = bit >> 1;
        }

        if eval_formula(copy.as_str()) {
            row += "1 |";
        }
        else {
            row += "0 |";
        }

        println!("{}", row);
    }
}

fn test_truth_table(formula: &str) {
    println!("Testing {} :", formula);
    print_truth_table(formula);
    println!("");
}

fn main() {
    test_truth_table("AB&C|");
    test_truth_table("QT&");
    test_truth_table("AB>");
    test_truth_table("SS=");
    test_truth_table("EX>R=Y^");
    test_truth_table("A1=");
    test_truth_table("11^");
}