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


fn test_map(x: u16, y: u16) {
    println!("Converting {} and {}", x, y);
    let mapped : f64 = map(x, y);
    println!("Converted to {} !", mapped);
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