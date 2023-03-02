fn method_1(a: u64, b: u64) {
    for i in 1..509 {
        let result = a.pow(i) % b;
        println!("Result {}: {}", i, result);
    }
}

fn method_2(a: u64, b: u64) {
    for i in 1..509 {
        let result = a.pow(i) % b;
        if result == 466 {
            println!("Result is: {}", i);
        }
    }
}

fn main() {
    println!("Method 1:\n");
    method_1(499, 509);

    println!("\nMethod 2:\n");
    method_2(499, 509);
}
