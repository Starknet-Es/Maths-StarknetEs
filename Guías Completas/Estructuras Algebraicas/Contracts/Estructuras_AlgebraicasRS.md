## Guia de Estructuras Algebraicas para Rust
Suponiendo que tiene conocimientos previos sobre Rust y que ha revisado el material de Aritmetica modular, nos centraremos en ejecutar varios contratos y ver los resultados.

- [Estructuras_Algebraicas.rs](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Rust/Estructuras_Algebraicas.rs)


Primero clone este repositorio para tener todo los codigos ejecutadndo
```bash
gh repo clone Starknet-Es/Maths-StarknetEs
```

## Estructuras Algebraicas
Para describir el concepto de estructuras algebraicas, en el [Estructuras_Algebraicas.md](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Readme.md) se relacionó a algunos conjuntos numéricos con diversas operaciones matemáticas.

Para ejecutar los comandos dentro de la carpeta donde se encuentra nuestros archivo Rust ejecute
```bash
rustc Estructuras_Algebraicas.rs
```

Se generará un archivo el cual puede luego ejecutar corriendo en su terminal:
```rust
./ Estructuras_Algebraicas
```

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/algeru.png)

### Números naturales y operación suma:

Si a y b son números naturales, entonces a + b será un número natural:
```rust
fn suma(a: i32, b: i32) -> i32 {
  return a + b;
```  

### Números naturales y operación resta:
```rust
fn resta(a: i32, b: i32) -> i32 {
  return a - b;
```

### Elemento neutro:

En caso de que trabajemos con la suma, el «elemento neutro» es un elemento e de un conjunto, tal que para cualquier otro elemento a del mismo conjunto, se cumple que:

```bash
a + e = e + a = a
```
Y tendriamos para comprobarlo la suma asociativa y conmutativa

```rust
fn asociativa_suma(a: i32, b: i32, c: i32) -> bool {
  return (a + b) + c == a + (b + c);
}
```

```rust
fn conmutativa_suma(a: i32, b: i32) -> bool {
  return a + b == b + a;
}
```

Si fuese el caso en el que la operación a la que relacionamos con los números naturales es la multiplicación, entonces el elemento neutro sería el 1; puesto que para todo a perteneciente a los números naturales, tenemos que:

```bash
a * 1 = 1 * a = a
```

```rust
fn distributiva_multiplicacion_sobre_suma(a: i32, b: i32, c: i32) -> bool {
  return a * (b + c) == a * b + a * c;
```

## Elemento inverso:

Trabajando con números enteros, si la operación del grupo es la suma, entonces el inverso de `x será -x`; es el elemento con el que tengo que sumar a x para que el resultado sea 0.

```rust

```

El inverso multiplicativo de x se define como x ^ -1:

```rust

```

## Link

- [Maths Starknet-Es Figuras Algebraicas](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main#teor%C3%ADa-de-grupos-y-campos)
- [Guía Completa Figuras Algebraicas](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Readme.md)
- [Guía Figuras Algebraicas Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasPY.md)
- [Guía Figuras Algebraicas Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasRS.md)
- [Guía Figuras Algebraicas Cairo 1.0](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasCAIRO.md)
