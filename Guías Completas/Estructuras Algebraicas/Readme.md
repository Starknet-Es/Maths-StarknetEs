## Definiendo conceptos criptográficos básicos:

#### Hoy le tocó el turno a las estructuras algebraicas:

¡Empecemos!

Leyendo documentación acerca de diversos sistemas criptográficos es habitual conseguirse con términos como `«grupos»` o `«cuerpos»`. Esos conceptos son estructuras algebraicas, y se estudian en una parte de la matemática llamada álgebra abstracta.

¿De qué se tratan? Comencemos desde la base:

## ¿Qué es un conjunto?
Un conjunto es una colección de elementos. Podríamos hablar de un conjunto de libros, un conjunto de edificios, conjuntos de números, etc.

<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/Estructuras.jpg" width="400">
</div>

Los conjuntos, como puede apreciarse, tienen elementos, que pueden ser finitos o infinitos. Piensa por un momento en el conjunto de los días de la semana (lunes, martes, miércoles y así hasta el domingo); ese sería un conjunto finito, ¿cierto?

En cambio, el conjunto de los números enteros (aquellos que no tienen decimales) constituye un conjunto infinito.

#### ¡Excelente! Entender lo que es un conjunto es clave.

## Ahora, ¿qué sucede si de alguna forma relacionamos un conjunto con una operación?

Por ejemplo: relacionemos a los números naturales con la suma.

Si sumamos dos números naturales (no importa cuáles), obtendremos como resultado otro número natural. Acá algunos ejemplos:

```bash
2 + 7 = 9
1 + 3 = 4
5 + 2 = 7
```

Como esto sucede, se dice entonces que la suma es una `«ley de composición interna»` en los números naturales.
No se puede decir lo mismo de la resta, dado que si tomamos dos números naturales y los restamos, puede suceder que el resultado sea un número negativo:

```bash
4 - 6 = -2
```

Y por esta razón, se dice que la resta no es una «ley de composición interna» en los números naturales. Por otra parte, en estructuras algebraicas hay un concepto muy particular denominado `«elemento neutro»`.

En el caso de que trabajemos con la suma, es un elemento e de un conjunto S, tal que para cualquier otro elemento a de S, se cumple que:

```bash
a + e = e + a = a
```

Es muy sencillo. En ese caso, podemos decir que el elemento neutro es el 0.

Para todo elemento a de los números naturales, tenemos que `a + 0 = 0 + a = a`

Por ejemplo:

```bash
7 + 0 = 0 + 7 = 7
```

Si fuese el caso en el que la operación a la que relacionamos con los números naturales es la multiplicación, entonces el elemento neutro sería el 1; puesto que para todo a perteneciente a los números naturales, tenemos que:

```bash
a * 1 = 1 * a = a
```

Por ejemplo:

```bash
7 * 1 = 1 * 7 = 7
```

#### ¡Bien! Para continuar es importante recordar lo que es, en matemática, la propiedad asociativa. 
Si seguimos con el ejemplo de la suma, se dice que esa operación es asociativa si se cumple que:

`(a + b) + c = a + (b + c)`, para todo `a, b y c` pertenecientes a los números naturales.

## Dicho todo esto, podemos definir lo que es un monoide:

Un monoide es una estructura algebraica que:

- Relaciona un conjunto con una operación que es asociativa,
- Donde esa operación es una ley de composición interna en ese conjunto,
- Y que tiene elemento neutro.

Hasta aquí todo genial, pero… estudiando criptografía es más frecuente toparse con el concepto de grupo que con el de monoide.
No hay problema: un grupo no es más que un monoide que tiene un «elemento inverso» para cada elemento del conjunto.

## ¿Elemento inverso? Te explico:

Trabajando con números enteros, si la operación del grupo es la suma, entonces el inverso de `x será -x`; es el elemento con el que tengo que sumar a x para que el resultado sea 0.

Como `x + (-x) = 0`, decimos que -x es el inverso aditivo de x:

Por ejemplo: `-7 sería el inverso de 7`

### Con la multiplicación sucede lo siguiente:

El inverso multiplicativo de x se define como x ^ -1

De modo que: `x * (x ^ -1) = 1`

Por ejemplo:

`7 * (7 ^ -1) = 1`

Llegados a este punto, vale la pena decir que (tomando como ejemplo la operación suma) si para dos elementos del grupo a y b se cumple que:

`a + b = b + a`, entonces se dice que la operación es conmutativa y el grupo en cuestión es abeliano. Muy relevantes en criptografía.

### Algunos ejemplos de grupos son:

- El conjunto de los números enteros con la suma.
- El conjunto de los números racionales con la suma.
- El conjunto de los números racionales, excluyendo al cero, con la multiplicación.

Cabe destacar que en documentos de criptografía es muy usual encontrarse con el concepto de cuerpo (o «field» en la literatura en inglés). ¿Qué es eso?

Bueno, un cuerpo es un conjunto A (no vacío) que se relaciona con dos operaciones «+» y «*» de la siguiente manera:

A con la suma es un grupo abeliano.

Y para el producto se tiene que:

- Dicha operación es asociativa y conmutativa.
- Existe elemento neutro y todo elemento distinto de 0 tiene inverso.

Y, por otra parte, si `a, b y c` pertenecen a A, entonces `a * (b + c) = (a * b) + (a * c)`

## Algunos ejemplos de cuerpos son:

- El conjunto de los números reales con la suma y el producto.
- El conjunto de los números racionales con la suma y el producto.
- Cuerpos finitos (muy empleados en sistemas criptográficos).

En definitiva, este es un tema súper interesante y básico para entender cómo funcionan muchísimos sistemas criptográficos. Espero que este material te haya gustado y, sobre todo, que sea de mucha utilidad.

* [Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasPY.md)
* [Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasRS.md)
* [Cairo](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Estructuras%20Algebraicas/Contracts/Estructuras_AlgebraicasCAIRO.md)

Además de eso, aquí abajo te dejamos algunos enlaces de interés:

* [Link del Índice de la Guía Completa Maths](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main/Gu%C3%ADas%20Completas)
* [Link de la explicación de 0xhasher_ en Twitter](https://twitter.com/0xhasher_/status/1586142877796552704)

¡Hasta la próxima!
