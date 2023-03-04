## Guía de Estructuras Algebraicas para Python

Suponiendo que tiene conocimientos previos sobre Python y que ha revisado el material de Aritmetica modular, nos centraremos en ejecutar varios contratos y ver los resultados.

- [Estructuras_Algebraicas.py](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Python/Estructuras_Algebraicas.py)


Primero clone este repositorio para tener todos los códigos ejecutando
```bash
gh repo clone Starknet-Es/Maths-StarknetEs
```

## Estructuras Algebraicas.

Para describir el concepto de estructuras algebraicas, en el documento Readme base de este contenido [(que puede consultar aquí)](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Readme.md) se relacionó a algunos conjuntos numéricos con diversas operaciones matemáticas.

Para ejecutar los comandos dentro de la carpeta donde se encuentra nuestros archivo Python ejecute

```python
python3.9 Estructuras_Algebraicas.py
```

![Graph](https://github.com/Nadai2010/Clases-Maths/blob/master/im%C3%A1genes/algepy.png)

### Números naturales y operación suma:

Si a y b son números naturales, entonces a + b será un número natural:

```python
def suma (a, b):

    return a + b
```  

### Números naturales y operación resta:

```python
def resta (a, b):

    return a - b
```

### Elemento neutro:

En caso de que trabajemos con la suma, el «elemento neutro» es un elemento e de un conjunto, tal que para cualquier otro elemento a del mismo conjunto, se cumple que:

```bash
a + e = e + a = a
```

```python
def neutro_suma (a, e):

    if a + e == e + a and a + e == a:
        print("e es el elemento neutro. e = 0.")
    else:
        print("e no es el elemento neutro. e es distinto de 0.")
```

Si fuese el caso en el que la operación a la que relacionamos con los números naturales es la multiplicación, entonces el elemento neutro sería el 1; puesto que para todo a perteneciente a los números naturales, tenemos que:

```bash
a * 1 = 1 * a = a
```

```python
def neutro_mult (a, e):

    if a * e == e * a and a * e == a:
        print("e es el elemento neutro. e = 1.")
    else:
        print("e no es el elemento neutro. e es distinto de 1.")
```

## Elemento inverso:

Trabajando con números enteros, si la operación del grupo es la suma, entonces el inverso de `x será -x`; es el elemento con el que tengo que sumar a x para que el resultado sea 0.

```python
def inv_aditivo (x):

    inverso_aditivo = -x
    return inverso_aditivo
```

El inverso multiplicativo de x se define como x ^ -1:

```python
def inv_mult (x):

    inverso_mult = x * -1
    return inverso_mult
```

## Link

- [Maths Starknet-Es Estructuras Algebraicas](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main#teor%C3%ADa-de-grupos-y-campos)
- [Guía Completa de Estructuras Algebraicas](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Readme.md)
- [Guía de Estructuras Algebraicas Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasPY.md)
- [Guía de Estructuras Algebraicas Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasRS.md)
- [Guía de Estructuras Algebraicas Cairo 1.0](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasCAIRO.md)
