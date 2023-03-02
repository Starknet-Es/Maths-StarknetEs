fn mod_add(a: i64, b: i64, m: i64) -> i64 {
    // Retorna la suma (a + b) % m
    (a + b) % m
}

fn mod_subtract(a: i64, b: i64, m: i64) -> i64 {
    // Retorna la resta (a - b) % m
    (a - b) % m
}

fn mod_multiply(a: i64, b: i64, m: i64) -> i64 {
    // Retorna el producto (a * b) % m
    (a * b) % m
}

fn mod_divide(a: i64, b: i64, m: i64) -> i64 {
    // Retorna la división (a / b) % m
    // Nota: Esto solo es válido si m es primo y b es coprimo con m
    // En otras palabras, b no tiene factores en común con m
    let b_inv = b.pow((m - 2) as u32) % m;
    return (a * b_inv) % m;
}

fn main() {
    // Suma modular de 15 y 7 modulo 10
    let result = mod_add(15, 7, 10);
    println!("{}", result);  // Output: 2

    // Resta modular de 15 y 7 modulo 10
    let result = mod_subtract(15, 7, 10);
    println!("{}", result);  // Output: 8

    // Producto modular de 15 y 7 modulo 10
    let result = mod_multiply(15, 7, 10);
    println!("{}", result);  // Output: 5

    // División modular de 15 y 7 modulo 10
    let result = mod_divide(15, 7, 10);
    println!("{}", result);  // Output: 2
}
