use std::str::Chars;
use std::collections::LinkedList;

struct OpNode {
    value: char,
    left: Option<Box<OpNode>>,
    right: Option<Box<OpNode>>,
}

impl OpNode {
    fn new() -> OpNode {
        OpNode {
            value: '*',
            left: None,
            right: None
        }
    }

    fn copy(&mut self) -> Option<Box<OpNode>> {
        let mut new_node = OpNode::new();
        new_node.value = self.value;

        match self.left.as_mut() {
            None => (),
            Some(left) => new_node.left = left.copy(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => new_node.right = right.copy(),
        }

        return Some(Box::new(new_node));
    }

    fn parse_op(&mut self, ops: &mut Chars) {
        let op = ops.next_back();
        if op.is_none() {
            return ;
        }

        self.value = op.unwrap();

        if "!&|^>=".contains(op.unwrap()) {
            let new_node = Some(Box::new(OpNode::new()));
            self.right = new_node;
            self.right.as_mut().unwrap().parse_op(ops);

            if "&|^>=".contains(op.unwrap()) {
                let new_node = Some(Box::new(OpNode::new()));
                self.left = new_node;
                self.left.as_mut().unwrap().parse_op(ops);
            }
        }
    }

    fn stringify(&mut self) -> String {
        let mut expr = String::from("");

        match self.left.as_mut() {
            None => (),
            Some(left) => expr += &left.stringify(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => expr += &right.stringify(),
        }

        expr.push(self.value);

        return expr;
    }

    fn negify(&mut self) {
        match self.left.as_mut() {
            None => (),
            Some(left) => left.negify(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => right.negify(),
        }

        if self.value == '!' {
            if self.right.as_ref().unwrap().value == '!' {
                self.value = self.right.as_ref().unwrap().right.as_ref().unwrap().value;
                self.left = self.right.as_mut().unwrap().right.as_mut().unwrap().left.take();
                self.right = self.right.as_mut().unwrap().right.as_mut().unwrap().right.take();
                self.negify();
            }
            else if "&|".contains(self.right.as_ref().unwrap().value) {
                if self.right.as_ref().unwrap().value == '&' {
                    self.value = '|';
                }
                else {
                    self.value = '&';
                }

                let mut left_node = OpNode::new();
                left_node.value = '!';
                left_node.right = self.right.as_mut().unwrap().left.take();

                let mut right_node = OpNode::new();
                right_node.value = '!';
                right_node.right = self.right.as_mut().unwrap().right.take();

                self.left = Some(Box::new(left_node));
                self.right = Some(Box::new(right_node));
                self.negify();
            }
        }
        else if self.value == '^' {
            self.value = '|';
            
            let mut left_negation_node = OpNode::new();
            left_negation_node.value = '!';
            left_negation_node.right = self.left.as_mut().unwrap().copy();

            let mut right_negation_node = OpNode::new();
            right_negation_node.value = '!';
            right_negation_node.right = self.right.as_mut().unwrap().copy();

            let mut left_node = OpNode::new();
            left_node.value = '&';
            left_node.left = self.left.as_mut().unwrap().copy();
            left_node.right = self.right.as_mut().unwrap().copy();

            let mut right_node = OpNode::new();
            right_node.value = '&';
            right_node.left = Some(Box::new(left_negation_node));
            right_node.right = Some(Box::new(right_negation_node));

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.negify();
        }
        else if self.value == '>' {
            self.value = '|';

            let mut left_node = OpNode::new();
            left_node.value = '!';
            left_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.negify();
        }
        else if self.value == '=' {
            self.value = '&';
            
            let mut left_node = OpNode::new();
            left_node.value = '>';
            left_node.left = self.left.as_mut().unwrap().copy();
            left_node.right = self.right.as_mut().unwrap().copy();

            let mut right_node = OpNode::new();
            right_node.value = '>';
            right_node.left = self.right.as_mut().unwrap().copy();
            right_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.negify();
        }
    }

    fn eval_sets(&mut self, sets: &Vec<Vec<i32>>) -> Vec<i32> {
        let formula = self.stringify();
        let mut stack : LinkedList<Vec<i32>> = LinkedList::new();
        let mut vars : String = String::from("");
        let mut master_set : Vec<i32> = Vec::new();

        // Get vars
        for c in 'A'..='Z' {
            if formula.contains(c) {
                vars.push(c);
            }
        }

        // Get master set
        for set in sets {
            for n in set {
                if !master_set.contains(n) {
                    master_set.push(*n);
                }
            }
        }

        // Evaluate
        for c in formula.chars() {
            // Case A-Z
            if c.is_ascii_uppercase() {
                stack.push_back(sets[vars.find(c).unwrap()].clone());
            }
            // Case !
            else if c == '!' {
                if stack.len() < 1 {
                    panic!("invalid input");
                }
                let right: Vec<i32> = stack.pop_back().unwrap();
                let mut inverse : Vec<i32> = Vec::new();

                for n in &master_set {
                    if !right.contains(&n) {
                        inverse.push(*n);
                    }
                }

                stack.push_back(inverse);
            }
            // Cases | &
            else if "&|".contains(c) {
                if stack.len() < 2 {
                    panic!("invalid input");
                }
                let right: Vec<i32> = stack.pop_back().unwrap();
                let left: Vec<i32> = stack.pop_back().unwrap();

                // Case |
                if c == '|' {
                    let mut combine : Vec<i32> = Vec::new();

                    for n in &right {
                        combine.push(*n);
                    }
                    for n in &left {
                        if !combine.contains(&n) {
                            combine.push(*n);
                        }
                    }

                    stack.push_back(combine);
                }
                // Case &
                else if c == '&' {
                    let mut inter : Vec<i32> = Vec::new();

                    for n in &right {
                        if left.contains(&n) {
                            inter.push(*n);
                        }
                    }

                    stack.push_back(inter);
                }
            }
        }

        if stack.len() != 1 {
            panic!("invalid input");
        }

        return stack.pop_back().unwrap();
    }
}

fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut chars = formula.chars();
    let mut root = OpNode::new();

    root.parse_op(&mut chars);
    root.negify();

    return root.eval_sets(sets);
}

fn main() {
    let sets = vec![
        vec![0, 1, 2],
        vec![0, 3, 4],
    ];
    let result = eval_set("BA&", &sets);
    println!("{:?}", result);
    // [0]
    let sets = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
    ];
    let result = eval_set("AB|", &sets);
    println!("{:?}", result);
    // [0, 1, 2, 3, 4, 5]
    let sets = vec![
        vec![0, 1, 2],
    ];
    let result = eval_set("A!", &sets);
    println!("{:?}", result);
    // []
}