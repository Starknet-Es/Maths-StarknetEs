## Guía de Zero Knowledge para Python

Suponiendo que tiene conocimientos previos sobre Python y que ha revisado el material de Zero Knowledge, nos centraremos en ejecutar varios contratos y ver los resultados.

- [zkProof_Base.py](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Python/zkProof_Base.py)

Primero clone este repositorio para tener todos los códigos ejecutando
```bash
gh repo clone Starknet-Es/Maths-StarknetEs
```

## Zero Knowledge

Para describir el concepto de Zero Knowledge, revise en el documento Readme base de este contenido [(que puede consultar aquí)]()

Para ejecutar los comandos dentro de la carpeta donde se encuentra nuestros archivo Python ejecute.

```python
python3.9 zkProof_Base.py
```

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/im%C3%A1genes/zkproofpy2.png)

Este código es un ejemplo de cómo realizar una prueba de conocimiento cero de un valor secreto, utilizando un número aleatorio y una moneda para generar pruebas aleatorias. En este ejemplo, el valor secreto es el número `x` en la expresión 

```python
yn = (b ** x) % p
```

El código comienza definiendo los valores de `b`, `p` e `y` que corresponden a la expresión anteriormente mencionada. El valor de `x` es conocido previamente solo por el usuario en este caso se usó `x = 383` y se utiliza como valor de entrada en el programa.

A continuación, se define `N` como el valor `p-1`. Este valor se utiliza en el paso 1 de la prueba de conocimiento cero.

```python
N = p - 1
```

En el paso 1, se elige un número aleatorio `e` entre `2 y N-1` (no se incluyen los extremos).

```python
  lmt = 2
    e = random.randrange(lmt, N)
```

Luego se calcula `bp = (b ** e) % p`- Este valor se envía al verificador, pero no se revela el valor de `e`.

```python
   bp = (b ** e) % p
        print("bp =", bp)
```

En el paso 2, se simula un lanzamiento de moneda para determinar si se utilizará la opción 2a o 2b. Si e es par, se utiliza la opción 2a y se imprime el valor de e y bp. Si e es impar, se utiliza la opción 2b. En esta opción, se calcula `power = (x + e) % N` y luego se calcula `b_xe = (b ** power) % p` y `ybp = (y * bp) % p`. Si `ybp = b_xe`, entonces la prueba ha sido exitosa.

```python
  if e % 2 == 0: # Simulamos el lanzamiento de la moneda determinando si e es par o no.

            print("La moneda ha salido cara, entonces:")
            print("El valor de e es:", e)
            print("Puedes comprobar que b ** e = bp =", bp)

        else:
```
Podrá revisar si ha salido cara, entonces ha sido exitoso pero...

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/im%C3%A1genes/zkproofpy4.png)

```rust
            print("La moneda ha salido cruz, entonces:")
            power = (x + e) % N
            print("(x + e) % N =", power)
            b_xe = (b ** power) % p
            ybp = (y * bp) % p

            if ybp == b_xe:
                print("La prueba ha sido exitosa")
```
Si ha salido Cruz, también habrá sido exitoso como hemos explicado en tutorial, hemos demostrado el valor de X sin revelar el secreto.

![Graph](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/im%C3%A1genes/zkproofpy3.png)


## Link

- [Maths Starknet-Es Zero Knowledge](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main#sistemas-de-prueba-de-conocimiento-cero)
- [Guía Completa de Zero Knowledge](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Readme.md)
- [Guía de Zero Knowledge Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgePY.md)
- [Guía de Zero Knowledge Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeRS.md)
- [Guía de Zero Knowledge Cairo 1.0](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeCAIRO.md)






`
