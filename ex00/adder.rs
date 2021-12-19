fn adder(a: u32, b: u32) -> u32 {
    let mut added : u32 = a ^ b;
    let mut current : u32 = 1;
    let mut carry : bool = false;

    for _i in 1..33 {
        if carry {
            added = added ^ current;
        }

        if (current & a & b) != 0 {
            carry = true;
        }
        else if carry && (current & (a | b)) != 0 {
            carry = true;
        }
        else {
            carry = false;
        }

        current = current << 1;
    }

    if carry {
        panic!("attempt to add with overflow");
    }

    return added;
}

fn test_adder(a: u32, b: u32) {
    let n = adder(a, b);
    println!("Real  : {} + {} = {}", a, b, a + b);
    println!("Adder : {} + {} = {}", a, b, n);
    println!("");
}

fn main() {
    test_adder(0, 0);
    test_adder(1, 0);
    test_adder(2, 4);
    test_adder(3, 3);
    test_adder(12, 0);
    test_adder(37, 56);
    test_adder(236, 477);
    test_adder(1000, 10000);
    test_adder(578902, 3672890);
    // test_adder(-1, -10); // <== invalid for u32
    // test_adder(2000000000, 3000000000); // <== invalid for + operator, mimicking behavior with panic!
}