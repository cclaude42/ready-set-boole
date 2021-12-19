fn gray_code(n: u32) -> u32 {
    let mut result : u32 = 0;
    let mut current : u32 = 1;
    let mut pow : u32 = 1;

    for _i in 0..16 {
        if (n + (4 * pow) - pow) % (4 * pow) < (2 * pow) {
            result = result | current;
        }

        current = current << 1;
        pow = 2 * pow;
    }

    return result;
}

fn main() {
    println!("{}", gray_code(0));
    // 0
    println!("{}", gray_code(1));
    // 1
    println!("{}", gray_code(2));
    // 3
    println!("{}", gray_code(3));
    // 2
    println!("{}", gray_code(4));
    // 6
    println!("{}", gray_code(5));
    // 7
    println!("{}", gray_code(6));
    // 5
    println!("{}", gray_code(7));
    // 4
    println!("{}", gray_code(8));
    // 12

    for i in 0..16 {
        println!("{:04b}", gray_code(i));
    }
}