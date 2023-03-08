<div align="center">
  <h1 style="font-size: larger;">
    <strong>Aritmetización II</strong> 
    </h1>
    <img src="" width="600">
</div>


## Temas

- [Recapitulación](#recapitulación)
- [Consultas y códigos de corrección de errores](#consultas-y-códigos-de-corrección-de-errores)
- [Ejemplo de juguete: Traza de ejecución booleana](#ejemplo-de-juguete-traza-de-ejecución-booleana)
- [Raíces de polinomios](#raíces-de-polinomios)
  - [Montaje](#montaje)
  - [Fibonacci](#fibonacci)
  - [Traslación a polinomios](#traslación-a-polinomios)
- [Sucinto](#sucinto)
- [Más restricciones, más columnas](#más-restricciones-más-columnas)
- [Conclusión](#conclusión)

## Parte 3
"Tenemos que profundizar"
Este es el tercer post de nuestra serie STARK Math, si no has leído el primero y el segundo, te recomendamos que lo hagas antes de seguir leyendo. Aviso: éste es un poco más matemático que los anteriores.

## Recapitulación
En el post anterior introdujimos la [aritmetización](https://medium.com/starkware/arithmetization-i-15c046390862) - el proceso de transformar una declaración de Integridad Computacional (IC) en la comprobación de si un polinomio es de bajo grado. Esta transformación nos permite lograr una verificación sucinta, en la que el verificador de la declaración de IC requiere exponencialmente menos recursos que los necesarios para una repetición ingenua. En el post anterior nos centramos en el primer paso de esta transformación mediante el ejemplo de transformar una sentencia CI sobre una [secuencia de Collatz](https://en.wikipedia.org/wiki/Collatz_conjecture) en una traza de ejecución y un conjunto de restricciones polinómicas. En esta entrada daremos el siguiente paso y mostraremos, utilizando esta vez una [secuencia de Fibonacci](https://en.wikipedia.org/wiki/Fibonacci_number), cómo el prover puede combinar la traza de ejecución y las restricciones polinómicas para obtener un polinomio que se garantiza que es de grado bajo si y sólo si la traza de ejecución satisface las restricciones polinómicas con las que empezamos. Además, mostraremos cómo el dominio sobre el que se considera el polinomio permite al verificador evaluarlo sucintamente. También discutiremos brevemente cómo los códigos de corrección de errores juegan un papel en STARKs.
Asumiremos que estamos familiarizados con los grupos finitos, los polinomios sobre campos finitos y las entradas anteriores de esta serie.

## Consultas y códigos de corrección de errores
Recordemos que nuestro objetivo es hacer posible que un verificador plantee al probador un número muy reducido de preguntas y decida si acepta o rechaza la prueba con un alto nivel de precisión garantizado. Lo ideal sería que el verificador pidiera al prover que proporcionara los valores en unos pocos lugares (aleatorios) de la traza de ejecución y comprobara que las restricciones polinómicas se cumplen en esos lugares. Una traza de ejecución correcta pasará naturalmente esta prueba. Sin embargo, no es difícil construir una traza de ejecución completamente errónea, que viole las restricciones sólo en un lugar, y, al hacerlo, llegar a un resultado completamente distinto y diferente. Identificar este fallo mediante un pequeño número de consultas aleatorias es altamente improbable.

Las técnicas habituales que abordan problemas similares son los [Códigos de Corrección de Errores](https://en.wikipedia.org/wiki/Error_detection_and_correction).

Los Códigos de Corrección de Errores transforman un conjunto de cadenas, algunas de las cuales pueden ser muy similares entre sí, en un conjunto de cadenas que son, por pares, muy diferentes, sustituyendo las cadenas originales por cadenas más largas.

Curiosamente, los polinomios pueden utilizarse para construir buenos códigos de corrección de errores, ya que dos polinomios de grado d, evaluados en un dominio considerablemente mayor que d, son diferentes en casi todas partes¹. Tales códigos se denominan códigos [Reed-Solomon](https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction).

Observando esto, podemos extender la traza de ejecución pensando en ella como una evaluación de un polinomio en algún dominio, y evaluando este mismo polinomio en un dominio mucho mayor. Extendiendo de manera similar una traza de ejecución incorrecta, se obtiene una cadena muy diferente, lo que a su vez hace posible que el verificador distinga entre estos casos utilizando un pequeño número de consultas.

Nuestro plan es, por tanto:

1. Reformular el rastro de ejecución como un polinomio.
2. Extenderlo a un dominio grande
3. Transformarlo, utilizando las restricciones del polinomio, en otro polinomio que se garantiza que es de grado bajo si y sólo si el rastro de ejecución es válido.

## Ejemplo de juguete: Traza de ejecución booleana
Supongamos que la sentencia CI en cuestión es "El prover tiene una secuencia de 512 números, todos los cuales son 0 o 1", que nos gustaría verificar leyendo sustancialmente menos de 512 números. Veamos qué tipo de traza de ejecución y restricciones polinómicas expresan este ejemplo de juguete:

La traza de ejecución tiene 512 filas, cada una de las cuales contiene una única celda con un cero o un uno.
La restricción polinómica que usamos aquí es simplemente Aᵢ⋅Aᵢ-Aᵢ=0, donde Aᵢ denota la celda i-ésima de esta traza de ejecución de una sola columna (un número es igual a su cuadrado si y sólo si es 0 ó 1).

Para reformular esta traza de ejecución en términos de polinomios, especificamos el campo en el que vamos a trabajar: elegimos Z₉₆₇₆₉, obtenido a partir del conjunto de enteros 0,1,...,96768 con suma y multiplicación módulo 96769. A continuación elegimos un subgrupo G de Z₉₆₇₆₉* (usamos F* para denotar el grupo multiplicativo² de F) tal que |G|=512, y algún generador³ g de G. La existencia de tal subgrupo está garantizada ya que 512 divide el tamaño de este grupo (que es 96768).

Ahora pensamos en los elementos de la traza de ejecución como evaluaciones de algún polinomio f(x) de grado inferior a 512 de la siguiente manera: la celda i-ésima contiene la evaluación de f en la potencia i-ésima del generador.

Formalmente:

FOTO

Este polinomio de grado 512 como máximo puede calcularse por interpolación, y a continuación procedemos a evaluarlo en un dominio mucho mayor⁴, formando un caso especial de codificación Reed-Solomon.

Por último, utilizamos este polinomio para crear otro, cuyo bajo grado depende de la restricción que se satisface sobre la traza de ejecución.

Para ello, debemos salirnos por la tangente y hablar de raíces de polinomios.

## Raíces de polinomios
Un hecho básico sobre los polinomios y sus raíces es que si p(x) es un polinomio, entonces p(a)=0 para algún valor específico a, si y sólo si existe un polinomio q(x) tal que (x-a)q(x)=p(x), y deg(p)=deg(q)+1.

Además, para toda x≠a, podemos evaluar q(x) calculando:

FOTO

Por inducción, un hecho similar es cierto para k raíces. A saber, si aᵢ es una raíz de p para todo i=0..k-1, entonces existe un polinomio q de grado deg(p)-k, y en todos los valores menos en estos k, es exactamente igual a:

FOTO

### Montaje
Reformulando la restricción polinómica en términos de f se obtiene el siguiente polinomio:

FOTO

Hemos definido f tal que las raíces de esta expresión son 1, g, g², ..., g⁵¹¹ si y sólo si las casillas de la traza de ejecución son 0 ó 1. Podemos definir:

FOTO

Y sabemos por el párrafo anterior que existe un polinomio de grado como máximo 2-deg(f)-512 que concuerda con p en todo x ∉{1, g, g², ..., g⁵¹¹} si y sólo si la traza de ejecución es de hecho una lista de 512 bits (es decir, 0 o 1). Nótese que anteriormente, el prover ha extendido la traza de ejecución a un dominio mayor, por lo que la consulta de los valores polinómicos en ese dominio está bien definida.

Si existe un protocolo por el cual el prover puede convencer⁵ al verificador de que este polinomio es de grado bajo, de tal forma que en él el verificador sólo pregunta por valores fuera de la traza de ejecución, entonces efectivamente el verificador estará convencido sobre la veracidad de la declaración CI sólo cuando sea verdadera. De hecho, en el próximo post, mostraremos un protocolo que hace exactamente eso, con una probabilidad de error muy pequeña. Por el momento, echemos un vistazo a otro ejemplo, que sigue siendo simple, pero no completamente trivial, y veamos cómo funciona la reducción en ese caso.

## Fibonacci
El ejemplo que utilizamos a continuación es el de calcular correctamente una sucesión de Fibonacci en Z₉₆₇₆₉ hasta el lugar 512-ésimo. La secuencia se define formalmente por:

FOTO

Y nuestra afirmación (es decir, la sentencia CI) es que a₅₁₁=62215.

Podemos crear una traza de ejecución para esta sentencia CI simplemente anotando los 512 números:

Foto

Las **restricciones polinómicas** que utilizamos son:

FOTO

### Traslación a polinomios
También aquí definimos un polinomio f(x) de grado a lo sumo 512, tal que los elementos de la traza de ejecución son evaluaciones de f en potencias de algún generador g.

Formalmente:

FOTO

Expresando las restricciones polinómicas en términos de f en lugar de A, obtenemos:

FOTO

Dado que una composición de polinomios sigue siendo un polinomio - la sustitución de la Aᵢ en las restricciones con f(gⁱ) todavía significa que estos son restricciones polinómicas.

Tenga en cuenta que 1, 2, y 4 son las restricciones que se refieren a un único valor de f, nos referimos a ellos como restricciones de límite.

La relación de recurrencia de Fibonacci, por el contrario, encarna un conjunto de restricciones sobre toda la traza de ejecución, y se puede reformular alternativamente como:

El uso de un generador para indexar las filas de la traza de ejecución nos permite codificar la noción de "fila siguiente" como una simple relación algebraica. Si x es una fila determinada de la traza de ejecución, gx es la fila siguiente, g²x es la fila posterior, g-¹x es la fila anterior, etcétera.

El polinomio de la relación de recurrencia: f(g²x)-f(gx)-f(x) es cero para cada x que indexa una fila en la traza de ejecución, excepto para las dos últimas. Esto significa que 1, g, g², ..., g⁵⁰⁹ son todas raíces de este polinomio de relación de recurrencia (y es de grado como máximo 510), por lo que podemos construir q(x) de la siguiente manera:

FOTO

En la jerga de STARK, a menudo se denomina polinomio de composición. En efecto, cuando la traza de ejecución original obedece a la relación de recurrencia de Fibonacci, esta expresión concuerda con algún polinomio cuyo grado es como máximo 2 (recuérdese que el grado de f es como máximo 512) en todos los valores excepto estos 510 1, g, g², ..., g⁵⁰⁹ . Sin embargo, el término polinomio de composición es algo engañoso, ya que cuando la traza de ejecución no satisface la restricción polinómica - las evaluaciones de esta expresión difieren de cualquier polinomio de bajo grado en muchos lugares. En otras palabras, se aproxima a un polinomio de bajo grado si y sólo si la CI original es correcta, que era nuestro objetivo.

Esto concluye la reducción prometida, que traduce el problema de comprobar si ciertas restricciones polinómicas se satisfacen sobre algún rastro de ejecución, al problema de comprobar si algún polinomio (conocido por el prover) es de bajo grado.

## Sucinto
Tener una técnica de verificación muy eficiente es clave para STARKs, y se puede ver como compuesto de dos partes - el uso de un pequeño número de consultas, y que el verificador realice un pequeño cálculo en cada consulta. Lo primero se consigue mediante códigos de corrección de errores, que permiten realizar consultas en muy pocos lugares, y lo segundo lo hemos escondido bajo la alfombra a lo largo de este artículo, hasta ahora. El trabajo del verificador puede resumirse en 1) consultar el polinomio de composición en lugares aleatorios, y 2) comprobar el bajo grado basándose en estas consultas. La comprobación sucinta de bajo grado se tratará en la próxima entrada, pero ¿qué queremos decir exactamente con "consultar el polinomio de composición"? El lector ávido puede haber sospechado de esta expresión, y con razón. El prover, después de todo, puede ser malicioso. Cuando el verificador pide la evaluación del polinomio de composición en alguna x, el prover puede responder con la evaluación de algún polinomio de grado realmente bajo, que pasará cualquier prueba de grado bajo, pero que no es el polinomio de composición.

Para evitar esto, el verificador consulta explícitamente la traza de ejecución Fibonacci en alguna fila w pidiendo los valores de f en tres lugares: f(w), f(gw), f(g²w).

El verificador puede ahora calcular el valor del polinomio de composición en w mediante:

FOTO

Donde el numerador puede ser calculado usando los valores obtenidos del prover, y el denominador... bueno, ahí está el problema (que fue barrido bajo la alfombra).

Por un lado, el denominador es completamente independiente de la traza de ejecución, por lo que el verificador puede calcularlo antes de comunicarse con el prover.

Por otro lado, en la práctica, la traza puede estar compuesta por cientos de miles de filas, y calcular el denominador costaría mucho tiempo de ejecución al verificador.

Aquí es donde la aritmetización es crucial para la concisión, ya que el cálculo de esta expresión para el caso especial en el que las potencias de g forman un subgrupo se puede hacer de manera muy eficiente si uno se da cuenta de ello:

FOTO

Esta igualdad es cierta porque ambos lados son polinomios de grado |G| cuyas raíces son exactamente los elementos de G.

El cálculo del lado derecho de esta ecuación parece requerir un número de operaciones lineal en |G|. Sin embargo, si recurrimos a la exponenciación por elevación al cuadrado, la parte izquierda de esta ecuación puede calcularse en un tiempo de ejecución que es logarítmico en |G|.

Y el denominador real del polinomio de composición de Fibonacci en cuestión puede obtenerse reescribiéndolo como:

Foto

Este aparente tecnicismo es la clave para que el verificador pueda ejecutarse en tiempo polilogarítmico, y sólo es posible porque vemos la traza de ejecución como evaluaciones de un polinomio sobre algún subgrupo del campo, y que las restricciones polinómicas en cuestión se mantienen sobre un subgrupo.

Se pueden aplicar trucos similares para trazas de ejecución más sofisticadas, pero es crucial que el patrón de repetición de la restricción coincida con algún subgrupo del campo.

## Más restricciones, más columnas
Los ejemplos de esta entrada eran deliberadamente sencillos, para resaltar aspectos clave de la aritmetización. Una pregunta natural que surge es: ¿cómo se maneja el caso de múltiples columnas y múltiples restricciones? La respuesta es sencilla: múltiples columnas simplemente significa que hay más de un polinomio para trabajar, y múltiples polinomios de composición - resultantes de las múltiples restricciones - se combinan en un solo polinomio, una combinación lineal aleatoria de todos ellos, por el bien de la última fase en STARK, que es una prueba de bajo grado. Con alta probabilidad, la combinación lineal es de grado bajo si y sólo si lo son todos sus componentes.

## Conclusión
Hemos demostrado cómo, dada una traza de ejecución y polinomios de restricción, el prover puede construir un polinomio que es de bajo grado si y sólo si la declaración CI original se mantiene. Además, hemos demostrado cómo el verificador puede consultar los valores de este polinomio de manera eficiente, asegurándose de que el prover no sustituye el polinomio verdadero por uno falso de bajo grado.

En el próximo post, que entrará en los detalles de la comprobación de bajo grado, se mostrará cómo se hace esta magia, de consultar un pequeño número de valores y determinar si algún polinomio es de bajo grado.

---

**Traduciones realizadas por el equipo oficial de Starknet-Es, gracias a [Dimeyad](https://github.com/dimeyad) y [Nadai](https://github.com/Nadai2010) por estos aportes**

Shir Peled
StarkWare

---

¹ Para ver esto, observe que la diferencia entre polinomios distintos de grado d es un polinomio distinto de cero de grado d, por lo tanto tiene como máximo d ceros.

² Recordatorio: el grupo multiplicativo se obtiene omitiendo el elemento cero del campo.

² Un generador es un elemento del subgrupo cuyas potencias abarcan todo el subgrupo.

⁴ La elección del tamaño de este dominio se traduce directamente en el error de solidez, cuanto mayor sea - menor será el error de solidez.

⁵ Tal que el verificador está convencido si y sólo si el prover no está engañando.



