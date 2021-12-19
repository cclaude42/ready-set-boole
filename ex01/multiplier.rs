fn adder(a: u32, b: u32) -> u32 {
    let mut sum : u32 = a ^ b;
    let mut current : u32 = 1;
    let mut carry : bool = false;

    for _i in 0..32 {
        if carry {
            sum = sum ^ current;
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

    return sum;
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut sum : u32 = 0;
    let mut current : u32 = 1;

    for i in 0..32 {
        if (current & b) != 0 {
            sum = adder(sum, a << i);
        }

        current = current << 1;
    }

    return sum;
}

fn test_multiplier(a: u32, b: u32) {
    let n = multiplier(a, b);
    println!("Real       : {} * {} = {}", a, b, a * b);
    println!("Multiplier : {} * {} = {}", a, b, n);
    println!("");
}

fn main() {
    test_multiplier(1, 0);
    test_multiplier(2, 4);
    test_multiplier(3, 3);
    test_multiplier(23, 4);
    test_multiplier(8, 47);
    test_multiplier(12, 0);
    test_multiplier(37, 56);
    test_multiplier(236, 477);
    test_multiplier(1000, 10000);
    // test_adder(-1, -10); // <== invalid for u32
    // test_multiplier(578902, 3672890); // <== invalid for + operator, mimicking behavior with panic!
}