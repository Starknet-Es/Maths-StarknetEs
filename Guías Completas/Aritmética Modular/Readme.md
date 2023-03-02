## ¿Qué es la Aritmética Modular y por qué es tan usada en criptografía? 🤔
Quizás una de las maneras más sencillas e ilustrativas de entender qué es la aritmética modular es comprendiendo – o recordando –  cómo funciona un reloj 🕙

- ✅ Generalmente, un reloj tiene forma circular y posee 12 números (del 1 al 12).
- ✅ Una de sus agujas va girando poco a poco indicando en qué hora del día nos encontramos.
- ✅ Luego de pasar por las 12, cae de nuevo al número 1, iniciando el ciclo nuevamente

De modo que si, por ejemplo, un día te dormiste a las 10 pm y pasaste 8 horas durmiendo, entonces te despertaste a las 6 am, ¿cierto? 😉 👇

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1.jpeg" width="400">
</div>

Lo interesante en el funcionamiento del reloj es que es cíclico. Hace una especie de `«reinicio»` luego de pasar por las 12:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/2.jpeg" width="400">
</div>

Por ese motivo, como es de esperarse, en los relojes no hay ningún número más grande que el 12. A las 12 se da el «reinicio» del que hablábamos anteriormente y se sigue avanzando. Hasta aquí todo bien, pero ¿qué tiene esto que ver con la aritmética modular? 🤔

## Conceptos

Bien, es hora de aclarar algunos conceptos como: El conjunto finito y el módulo de trabajo.

En este ejemplo, el conjunto de números del 1 hasta el 12 es lo que denominamos conjunto finito. Son números que se van repitiendo cíclicamente en el sistema (las horas del día). Y el 12 es el módulo de trabajo. En este caso, es la cantidad de elementos que hay en el conjunto

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/3.jpeg" width="400">
</div>

Es muy ilustrativo este ejemplo del reloj porque se aprecia muy claramente en qué momento se «reinicia» el conteo, y es algo con lo que lidiamos todos los días.

Otro ejemplo podría ser el siguiente:

- El conjunto de los días de la semana: lunes, martes, miércoles, jueves, viernes, sábado y domingo, sería un conjunto finito. Y el módulo sería 7. No hay un octavo día de la semana en ese sistema.
- Lo mismo sucede con los meses. Para ellos se usa el módulo 12. Luego de pasar diciembre (que es el mes número 12 del año), el conteo se reinicia y volvemos a enero (el mes número 1)

En el día a día, frecuentamos hacer operaciones matemáticas con «aritmética ordinaria». Operaciones sencillas, tales como:

```bash
2 + 2 = 4
3 - 2 = 1
```

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/4.jpeg" width="400">
</div>

En aritmética «ordinaria», 10 + 8 = 18... Es una suma normal, como siempre la hemos hecho. Sin ningún misterio
Sin embargo, las operaciones hechas con aritmética modular son un poco diferentes y muy curiosas.

Como vimos en el ejemplo del reloj – recordemos: te dormiste a las 10 pm, pasaron 8 horas y te despertaste a las 6 am – podríamos decir que 10 + 8 (que es 18) es equivalente a 6 en este sistema. ¿Equivalente? ¿Cómo así que equivalente? 😅

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/5.jpeg" width="400">
</div>

En ese ejemplo, los números que sean más grandes que el 12 tienen su «equivalente», es decir, su representación en el reloj (dentro del conjunto finito):

- ✅ El 13 es equivalente al 1
- ✅ El 14 es equivalente al 2
- ✅ El 15 es equivalente al 3

... Y así sucesivamente

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/6.jpeg" width="400">
</div>

Por lo que la aritmética modular no hace más que permitir maneras peculiares de hacer operaciones matemáticas, en conjuntos finitos y respetando módulos de trabajo. ¡Maravilloso! 🥳, pero...

## ¿Por qué es útil esto en criptografía?

Bueno, por varias razones. Vamos por partes:

1. Recordemos que la criptografía se basa en proteger información
2. La aritmética modular permite establecer problemas computacionalmente muy difíciles de resolver

De modo que `"uniendo las dos premisas anteriores"` la aritmética modular es especialmente útil en criptografía porque permite resguardar información mediante el planteamiento de problemas computacionalmente muy difíciles de resolver

## ¿Cómo usar dicha aritmética para establecer esos complicados problemas y proteger información?
Para entender las razones por la cual este tipo de aritmética es tan usada en criptografía, primero es necesario comprender el concepto de «congruencia»:

Para recordarlo: Decíamos que un día te dormiste a las 10 pm y pasaste 8 horas durmiendo y, por lo tanto, te despertaste a las 6 am

De él comprendimos que 18 es equivalente a 6 en módulo 12. Esa equivalencia puede expresarse de la siguiente manera:

```bash
18 ≡ 6 (mod 12)
```

Y se lee: «18 es congruente a 6, módulo 12»

Hay varias maneras de ver esto. Una de ellas es decir que cuando haces la división (sin decimales) de 18 / 12, el resto que da es 6:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/7.jpeg" width="400">
</div>

¡Bien! 18 ≡ 6 (mod 12), es una relación de congruencia. 18 y 6 equivalen a lo mismo en módulo 12

¿Por qué es esto importante? 🤔

Evaluemos lo siguiente:

Si tomamos un número como el 2 y comenzamos a elevarlo a distintas potencias (usando números naturales), obtenemos algo como esto:

```bash
2 ^ 1 = 2
2 ^ 2 = 4
2 ^ 3 = 8
```

Y así sucesivamente, como puede verse en esta tabla:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/8.jpeg" width="400">
</div>

Si hacemos lo mismo, pero usando aritmética modular (en este caso, usamos módulo 19), los resultados son diferentes

Acá te presento una tabla de resultados. Ten en cuenta que abajo de ella (en el siguiente tweet) encontrarás algunas instrucciones para ir haciendo los cálculos

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/9.jpeg" width="400">
</div>

Las instrucciones son las siguientes:

- ✅ Primero resuelves la potencia
- ✅ Luego puedes evaluar si el resultado es menor, igual o mayor que 19

Si el resultado es menor a 19, entonces lo tomamos, de lo contrario, lo divides entre 19 y tomas el resto.

Por ejemplo:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/0.jpeg" width="400">
</div>

Graficando los resultados obtenidos en ambas tablas, puede apreciarse que usando aritmética «ordinaria» (la de la primera tabla), los resultados siempre son más «grandes» con respecto al anterior, siempre son «crecientes»:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/11.jpeg" width="400">
</div>

Por otra parte, los resultados generados usando aritmética modular, en ocasiones van creciendo, y en muchas otras, van decreciendo:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/12.jpeg" width="400">
</div>

Entender esa diferencia es clave. Repetimos:

- 📈 Con aritmética «ordinaria», los resultados siempre van creciendo.
- 📈 📉 Con aritmética modular, van variando entre ir creciendo y decreciendo.

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/13.jpeg" width="400">
</div>

## Problemas

Si te digo que tengo esta expresión matemática:

```
2 ^ X = 134,217,728
```

... Y que necesitas conseguir el valor de la X, ¿qué harías? 🤔

Pues, una forma sería ir probando exponentes número por número hasta que el resultado dé 134,217,728 😅

Es decir, probar si 2 ^ 2 es igual a 134,217,728; si no, entonces probar 2 ^ 3; si no, 2 ^ 4 y así hasta que dé 134,217,728 🤯

A esa forma de trabajar se le llama `«fuerza bruta»`, y es un modo muy ineficiente de operar, porque si los números de la expresión matemática dada son lo suficientemente grandes, hasta las mejores computadoras de la actualidad podrían tardar hasta años en calcular el resultado.

Sin embargo, se puede aplicar otras técnicas:

Una de ellas sería ir probando exponentes – por ejemplo – de 5 en 5, hasta que el resultado dé con el que deseamos, o se acerque:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/14.jpeg" width="400">
</div>

Como puede verse, aquí el asunto es que puedes ir probando valores y siempre sabrás si el valor que estás probando es más grande o más pequeño que el valor que estás buscando

Probando valores, de alguna manera, te estás ubicando con respecto al resultado al que quieres llegar

Con aritmética modular no es tan sencillo hacer lo mismo porque los resultados que van dando, en ocasiones van creciendo, y en muchas otras, van decreciendo. Aquí una gráfica probando valores para `2 ^ n (mod 37)`, donde `n` son los números comprendidos entre el `1 y el 36:`

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/15.jpeg" width="400">
</div>

El hecho de que los resultados tengan ese comportamiento «caótico» dificulta que puedas hallar el valor de la X.

## Hagamos este ejercicio

Tienes esta expresión matemática:

```bash
499 ^ X ≡ 466 (mod 509)
```

... Y necesitas hallar el valor de la X, ¿cómo lo resuelves? 🧐

Pareciera que la forma de resolver el problema es ir probando todos los valores (desde el 1 hasta el 508). Ese sería un intento de resolver el problema por «fuerza bruta» y, como se dijo antes, es una forma muy ineficiente de llegar al resultado.

Los complicados problemas computacionales de los que hablábamos anteriormente son problemas parecidos a este que se plantea acá, pero con números mucho más grandes. El hecho de que sea «difícil» hallar el valor de esa incógnita, es aprovechado para intentar proteger a muchos sistemas criptográficos. Esa es una de las razones más importantes por la que se usa la aritmética modular en criptografía.

Existen otros motivos por el cual se utiliza, pero en esta ocasión se ha querido hacer énfasis en ese en particular porque se le considera de los motivos principales.

¿hallaste el valor de la incógnita en: 499 ^ X = 466 (mod 509)?

No... Pues te enseñamos como hacerlo

- [Resolver Aritmética Modular Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular/Contracts/Aritm%C3%A9tica_ModularPY.md)
- [Resolver Aritmética Modular Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular/Contracts/Aritm%C3%A9tica_ModularRS.md)
- [Resolver Aritmética Modular Cairo](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular/Contracts/Aritm%C3%A9tica_ModularCAIRO.

¡Hasta la próxima! 😉
- [Link Indice Guía Completa Maths](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main/Gu%C3%ADas%20Completas)
- [Link Twitter explicación 0xHaser](https://twitter.com/0xhasher_/status/1559387647642157056)
