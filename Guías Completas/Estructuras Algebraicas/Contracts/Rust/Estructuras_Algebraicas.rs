// Suma
fn suma(a: i32, b: i32) -> i32 {
  return a + b;
}

// Resta
fn resta(a: i32, b: i32) -> i32 {
  return a - b;
}

// Multiplicación
fn multiplicacion(a: i32, b: i32) -> i32 {
  return a * b;
}

// División
fn division(a: i32, b: i32) -> f32 {
  return (a as f32) / (b as f32);
}

// Comprobación de la propiedad asociativa de la suma
fn asociativa_suma(a: i32, b: i32, c: i32) -> bool {
  return (a + b) + c == a + (b + c);
}

// Comprobación de la propiedad conmutativa de la suma
fn conmutativa_suma(a: i32, b: i32) -> bool {
  return a + b == b + a;
}

// Comprobación de la propiedad distributiva de la multiplicación sobre la suma
fn distributiva_multiplicacion_sobre_suma(a: i32, b: i32, c: i32) -> bool {
  return a * (b + c) == a * b + a * c;
}

fn main() {
  let resultado_suma = suma(2, 7);
  println!("{}", resultado_suma); // Imprime 9

  let resultado_resta = resta(4, 6);
  println!("{}", resultado_resta); // Imprime -2

  let resultado_multiplicacion = multiplicacion(7, 1);
  println!("{}", resultado_multiplicacion); // Imprime 7

  let resultado_division = division(7, 2);
  println!("{}", resultado_division); // Imprime 3.5

  let es_asociativa_suma = asociativa_suma(2, 7, 3);
  println!("{}", es_asociativa_suma); // Imprime true

  let es_conmutativa_suma = conmutativa_suma(2, 7);
  println!("{}", es_conmutativa_suma); // Imprime false

  let es_distributiva_multiplicacion_sobre_suma = distributiva_multiplicacion_sobre_suma(2, 3, 4);
  println!("{}", es_distributiva_multiplicacion_sobre_suma); // Imprime true
}
