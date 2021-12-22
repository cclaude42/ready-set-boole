fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut powerset : Vec<Vec<i32>> = Vec::new();
    let mut possibilities : u32 = 1;

    for i in 1..set.len() {
        possibilities = possibilities << 1;
        if set[i..].contains(&set[i - 1]) {
            panic!("set has duplicates");
        }
    }

    for case in 0..possibilities {
        let mut subset : Vec<i32> = Vec::new();
        let mut bit : u32 = 1;

        for s in set {
            if case & bit != 0 {
                subset.push(*s);
            }
            bit = bit << 1;
        }

        powerset.push(subset);
    }

    return powerset;
}

fn test_powerset(set: &[i32]) {
    println!("Powersets of {:?} :", set);
    println!("{:?}", powerset(set));
    println!("Total : {} elements (should be 2^n)", powerset(set).len());
    println!("");
}

fn main() {
    test_powerset(&[]);
    test_powerset(&[42]);
    test_powerset(&[1, 2, 3]);
    test_powerset(&[6, -5, 12, 4]);
    test_powerset(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // test_powerset(&[1, 1]); // <-- invalid
}