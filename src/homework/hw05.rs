fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1 = 48;
    let num2 = 18;
    let result = gcd(num1, num2);
    
    println!("GCD({}, {}) = {}", num1, num2, result);
}