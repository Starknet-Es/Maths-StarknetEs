## Guía de Zero Knowledge para Rust

Suponiendo que tiene conocimientos previos sobre Rust y que ha revisado el material de Zero Knowledge, nos centraremos en ejecutar varios contratos y ver los resultados.

- [zkProof_Base.rs](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Rust/zkProof_Base.rs)

Primero clone este repositorio para tener todos los códigos ejecutando
```bash
gh repo clone Starknet-Es/Maths-StarknetEs
```

## Zero Knowledge

Para describir el concepto de Zero Knowledge, revise en el documento Readme base de este contenido [(que puede consultar aquí)](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Readme.md)

Para ejecutar los comandos dentro de la carpeta donde se encuentra nuestros archivo Rust ejecute:

```rust
rustc zkProof_Base.rs
```

Se generará un archivo el cual puede luego ejecutar corriendo en su terminal:

```rust
./ zkProof_Base
```

![Graph]()


Este código es un ejemplo de cómo realizar una prueba de conocimiento cero de un valor secreto, utilizando un número aleatorio y una moneda para generar pruebas aleatorias. En este ejemplo, el valor secreto es el número `x` en la expresión 

```rust
let yn = mod_exp(b, x, p);
```

El código comienza definiendo los valores de `b`, `p` e `y` que corresponden a la expresión anteriormente mencionada. El valor de `x` es conocido previamente solo por el usuario en este caso se usó `x = 383` y se utiliza como valor de entrada en el programa.

A continuación, se define `N` como el valor `p-1`. Este valor se utiliza en el paso 1 de la prueba de conocimiento cero.

```rust
let n = p - 1;
```

En el paso 1, se elige un número aleatorio `e` entre `2 y N-1` (no se incluyen los extremos).

```rust
 let lmt = 2;
    let mut rng = rand::thread_rng();
```

Luego se calcula `bp = (b ** e) % p`- Este valor se envía al verificador, pero no se revela el valor de `e`.

```rust
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

```

En el paso 2, se simula un lanzamiento de moneda para determinar si se utilizará la opción 2a o 2b. Si e es par, se utiliza la opción 2a y se imprime el valor de e y bp. Si e es impar, se utiliza la opción 2b. En esta opción, se calcula `let _power = e;` y luego se calcula `let ybp = (y * bp) % p;` y ` let b_xe = mod_exp(b, power, p); let ybp = (y * bp) % p;`. Si ` if ybp == b_xe`, entonces la prueba ha sido exitosa.

```rust
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
```


## Link

- [Maths Starknet-Es Zero Knowledge](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main#sistemas-de-prueba-de-conocimiento-cero)
- [Guía Completa de Zero Knowledge](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Readme.md)
- [Guía de Zero Knowledge Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgePY.md)
- [Guía de Zero Knowledge Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeRS.md)
- [Guía de Zero Knowledge Cairo 1.0](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeCAIRO.md)










`
