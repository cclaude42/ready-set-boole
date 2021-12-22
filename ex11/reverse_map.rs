fn map(x: u16, y: u16) -> f64 {
    let mut result : u32 = 0;
    let mut tbit : u32 = 1;
    let mut sbit : u16 = 1;

    for _i in 0..16 {
        if sbit & x != 0 {
            result = result | tbit;
        }
        tbit = tbit << 1;
        if sbit & y != 0 {
            result = result | tbit;
        }
        tbit = tbit << 1;
        sbit = sbit << 1;
    }

    let f : f64 = result.into();
    let d : f64 = u32::MAX.into();

    return f / d;
}

fn reverse_map(n: f64) -> (u16, u16) {
    let m : f64 = u32::MAX.into();
    let f : f64 = n * m;
    let r : u32 = f as u32;

    let mut x : u16 = 0;
    let mut y : u16 = 0;
    let mut tbit : u32 = 1;
    let mut sbit : u16 = 1;

    for _i in 0..16 {
        if tbit & r != 0 {
            x = x | sbit;
        }
        tbit = tbit << 1;
        if tbit & r != 0 {
            y = y | sbit;
        }
        tbit = tbit << 1;
        sbit = sbit << 1;
    }

    return (x, y);
}

fn test_map(x: u16, y: u16) {
    println!("Converting {} and {}", x, y);
    let mapped : f64 = map(x, y);
    println!("Converted to {} !", mapped);
    println!("Reversing conversion...");
    let unmapped : (u16, u16) = reverse_map(mapped);
    println!("Number evaluates to x {} and y {}", unmapped.0, unmapped.1);
    println!("");
}

fn main() {
    test_map(0, 0);
    test_map(1, 0);
    test_map(0, 1);
    test_map(100, 100);
    test_map(30000, 1);
    test_map(1, 30000);
    test_map(65534, 65535);
    test_map(65535, 65534);
    test_map(65535, 65535);
}