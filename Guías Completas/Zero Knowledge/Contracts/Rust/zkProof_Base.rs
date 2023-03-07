extern crate rand;

use rand::Rng;

fn main() {
    let b = 499;
    let p = 509;
    let y = 440;

    println!(r"Haremos una prueba de que conocemos el valor de x en la expresión (b ** x) % p = y,
    donde b = 499, p = 509 y y = 440; es decir, la expresión es esta: (499 ** x) % 509 = 440. x=383");

    // Pedimos el valor de x al usuario y lo convertimos a u64:
    let x = input("Coloca el valor de x: ").parse::<u64>().unwrap();
    let n = p - 1;

    // Calculamos yn = (b ** x) % p:
    let yn = mod_exp(b, x, p);

    if yn != y {
        println!("El valor introducido no es la solución");
    } else {
        println!("Efectivamente, (b ** x) % p = {}", y);

    // Paso número 1 (Elegir un número e < N y enviar (bp = b ^ e) al verificador):
    // Definimos el límite inferior para elegir e:
    let lmt = 2;
    let mut rng = rand::thread_rng();
    // Generamos un número aleatorio en el rango [lmt, n):
    let e = rng.gen_range(lmt..n);

if x == e {
    println!("Número e no es válido, por favor intente de nuevo");
    // Hay que evitar que e tenga el mismo valor que x
} else {
    let bp = mod_exp(b, e, p);
    println!("bp = {}", bp);

    if e % 2 == 0 {
        // Simulamos el lanzamiento de la moneda determinando si e es par o no.
        println!("La moneda ha salido cara, entonces:");
        println!("El valor de e es: {}", e);
        println!("Puedes comprobar que b ** e = bp = {}", bp);

        // Paso número 2 (Comprobar que conocemos x):
        let _power = e;
        let b_e = bp;
        let ybp = (y * bp) % p;

        if ybp == b_e {
        } else {
            println!("La salida ha sido exitosa");
        }
    } else {
        println!("La moneda ha salido cruz, entonces:");
        let power = (x + e) % n;
        println!("(x + e) % N = {}", power);
        let b_xe = mod_exp(b, power, p);
        let ybp = (y * bp) % p;

        if ybp == b_xe {
            println!("La salida ha sido exitosa");
        }
    }}
   }
}

// Función para leer el input del usuario:
fn input(prompt: &str) -> String {
    use std::io::Write;

    let mut s = String::new();
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut s).expect("Input error");
    s.trim().to_owned()
}

// Función para calcular la exponenciación modular
fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

