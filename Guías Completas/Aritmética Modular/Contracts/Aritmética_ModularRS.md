## Guia de Aritmetica Modular para Rust

Suponiendo que tiene conocimietnos previos sobre Rust y que ha revisado los conceptos de Aritmetica modular, nos centraremos en ejecutar varios contratos y ver los resultados. 

- [Aritmetica_Modular_Base.rs](https://github.com/Nadai2010/Clases-Maths/blob/master/Contracts/Rust/Aritmetica_Modular_Base.rs)- Código Base para las diferentes ecuaciones suma, resta, multiplicación y división en aritmética modular.
- [Aritmetica_Modular.rs](https://github.com/Nadai2010/Clases-Maths/blob/master/Contracts/Rust/Aritmetica_Modular.rs)- Código en para resolver el [ejercio de una expresión matemática](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular#hagamos-este-ejercicio)

Primero clone este repositorio para tener todo los codigos ejecutando:
```bash
gh repo clone Starknet-Es/Maths-StarknetEs
```

## Aritmetica Modular Base
Para ejecutar los comandos dentro de la carpeta donde se encuentra nuestros archivo Rust y ejecute

```rust
rustc Aritmetica_Modular_Base.rs
```

Se generará un archivo el cual puede luego ejecutar corriendo en su terminal:

```rust
./Aritmetica_Modular_Base
```

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/modbasers.png)

### Suma modular de 15 y 7 modulo 10
```rust
let result = mod_add(15, 7, 10);
    println!("{}", result); 
```
En este ejemplo, 15 + 7 = 22. El residuo de la división de 22 entre 10 es 2, por lo que el resultado de la suma modular de 15 y 7 módulo 10 es 2.

### Resta modular de 15 y 7 modulo 10
```rust
let result = mod_subtract(15, 7, 10);
    println!("{}", result);
```

La resta de 15 y 7 es 8. Sin embargo, como estamos trabajando en el módulo 10, necesitamos asegurarnos de que el resultado esté dentro del rango de valores permitidos (en este caso, de 0 a 9). El residuo de la división de 8 entre 10 es 8, lo que significa que el resultado de la resta modular de 15 y 7 módulo 10 es 8.

### Producto modular de 15 y 7 modulo 10
```rust
let result = mod_multiply(15, 7, 10);
    println!("{}", result); 
```

En este caso, el producto de 15 y 7 es 105. Sin embargo, como estamos trabajando en el módulo 10, necesitamos asegurarnos de que el resultado esté dentro del rango de valores permitidos (en este caso, de 0 a 9). El residuo de la división de 105 entre 10 es 5, lo que significa que el resultado del producto modular de 15 y 7 módulo 10 es 5.

### División modular de 15 y 7 modulo 10
```rust
let result = mod_divide(15, 7, 10);
    println!("{}", result);
```

La operación de división no es una operación modular en sí misma, por lo que en este caso estamos usando una técnica conocida como inverso multiplicativo para encontrar el resultado de la división modular.

El inverso multiplicativo de un número `a` en módulo `m` es un número `b` tal que `ab` es `congruente con 1 módulo m` (es decir, el residuo de la división de ab entre m es 1). En otras palabras, `b` es el número que, al multiplicarse por `a`, nos da un `resultado congruente con 1 módulo m`.

En este ejemplo, estamos buscando el inverso multiplicativo de 7 en módulo 10. El inverso multiplicativo de 7 en módulo 10 es 3, porque 7*3 es congruente con 1 módulo 10 (21 es divisible entre 10, por lo que el residuo de la división es 1).

Por lo tanto, la división de 15 entre 7 en módulo 10 es equivalente a la multiplicación de 15 por el inverso multiplicativo de 7 en módulo 10, es decir, 15 * 3. El resultado de 15 * 3 módulo 10 es 5, por lo que el resultado de la división modular de 15 y 7 módulo 10 es 5

## Aritmética Modular
Este contrato resuelve el problema "Encuentra el valor de "X" en la ecuación 499 ^ X ≡ 466 (mod 509)." Para ello ejecute en su terminal dentro de la carpeta donde está el codigo `Aritmetica_Modular.py`

```rust
rustc Aritmetica_Modular.rs 
```

Se generará un archivo el cual puede luego ejecutar corriendo en su terminal:

```rust
./Aritmetica_Modular
```

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/modrust.png)
![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/modrust.png)

## Link

- [Maths Starknet-Es Aritmética Modular](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main#aritm%C3%A9tica-modular)
- [Guía Completa Aritmética Modular](https://github.com/Nadai2010/Clases-Maths/blob/master/Readme%20Aritm%C3%A9tica.md)
- [Guía Aritmética Modular Python](https://github.com/Nadai2010/Clases-Maths/blob/master/Contracts/Aritm%C3%A9tica_ModularPY.md)
- [Guía Aritmética Modular Rust](https://github.com/Nadai2010/Clases-Maths/blob/master/Contracts/Aritm%C3%A9tica_ModularRS.md)
- [Guía Aritmética Modular Cairo 1.0](https://github.com/Nadai2010/Clases-Maths/blob/master/Contracts/Aritm%C3%A9tica_ModularCAIRO.md)



