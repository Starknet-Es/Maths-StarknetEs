<div align="center">
  <h1 style="font-size: larger;">
    <strong>Aritmetización I</strong> 
    </h1>
    <img src="" width="600">
</div>

## Temas

- [STARK a grandes rasgos](#stark-a-grandes-rasgos)
  - [Recapitulación: Declaraciones de integridad computacional](#recapitulación-declaraciones-de-integridad-computacional)
- [Aritmetización](#aritmetización)
  - [Traza de ejecución](#traza-de-ejecución) 
  - [Restricciones polinómicas](#restricciones-polinómicas)
- [Conjetura de Collatz](#conjetura-de-collatz)
  - [Trazado de la ejecución de la secuencia de Collatz](#trazado-de-la-ejecución-de-la-secuencia-de-collatz)
  - [Las restricciones polinómicas de la secuencia de Collatz](#las-restricciones-polinómicas-de-la-secuencia-de-collatz) 
- [Conclusión](#conclusión)


## Parte 2
Esta es la segunda parte de nuestra serie sobre las matemáticas detrás de STARKs (primera parte aquí), y es el primero de dos posts sobre aritmetización.

En el momento en que complete este post, usted debe tener una buena idea de lo que son la traza de ejecución y las restricciones polinómicas, y cómo una declaración de Integridad Computacional se transforma en estas. Empezaremos con un ejemplo sencillo de un recibo de supermercado, y pasaremos a uno un poco más complejo, el de las secuencias de Collatz, que está relacionado con un conocido problema abierto en teoría de números. Asumiremos una familiaridad básica con polinomios sobre campos finitos y representaciones binarias de números enteros.

## STARK a grandes rasgos
El objetivo del protocolo STARK es verificar los cálculos de forma sucinta y transparente. El primer paso en STARK se denomina aritmetización, y consiste en la traducción (a menudo denominada "reducción") del problema de verificar un cálculo al problema de comprobar que un polinomio determinado, que puede evaluarse de forma eficiente por parte del verificador (esta es la parte "sucinta"), es de grado bajo. La aritmetización es útil porque permite utilizar herramientas del ámbito de los Códigos de Corrección de Errores que comprueban eficientemente el bajo grado. Sin embargo, la aritmetización en sí misma sólo traduce una declaración de integridad computacional en un polinomio, preparando el escenario para la siguiente fase de STARK, que es otro protocolo interactivo que implica a un prover que intenta convencer a un verificador de que el polinomio es realmente de bajo grado. El verificador está convencido de que el polinomio es de grado bajo si y sólo si el cálculo original es correcto (excepto para una probabilidad infinitesimal). En el último paso de STARK, el protocolo interactivo se transforma en una única prueba no interactiva, que puede publicarse en una cadena de bloques y ser verificada públicamente por cualquiera.

La propia aritmetización consta de dos pasos: el primero consiste en generar una traza de ejecución y restricciones polinómicas; el segundo consiste en transformar estos dos objetos en un único polinomio de bajo grado. En términos de interacción prover-verificador, lo que realmente ocurre es que el prover y el verificador acuerdan de antemano cuáles son las restricciones polinómicas. A continuación, el prover genera una traza de ejecución, y en la interacción posterior, el prover intenta convencer al verificador de que las restricciones polinómicas se satisfacen sobre esta traza de ejecución, sin que el verificador lo vea.

## Recapitulación: Declaraciones de integridad computacional
En la [entrada anterior]() hablamos de la noción de declaración de integridad computacional ("CI", por sus siglas en inglés), la afirmación de que el resultado de un determinado cálculo es correcto, desde un punto de vista abstracto. Veamos un ejemplo concreto de declaración de integridad computacional: la suma total que debemos pagar en el supermercado se ha calculado correctamente. La prueba convencional de este enunciado concreto es el recibo. Normalmente, los artículos del recibo se enumeran con sus precios, y la suma total se indica en la parte inferior, así:

FOTO

Para simplificar, sólo consideramos que se trata de una afirmación de que la suma es correcta. Para comprobar si esta afirmación CI es válida, se puede repasar la lista -sin saltarse ningún elemento- para calcular la suma total y compararla con el número que aparece al final del recibo. Se trata de un ejemplo muy ingenuo, pero lo utilizaremos más adelante para demostrar la idea de comprobabilidad sucinta.

## Aritmetización
El primer paso de la aritmetización consiste en tomar una declaración de CI (como "la quinta transacción del bloque 7218290 es correcta") y traducirla a un lenguaje algebraico formal. Esto tiene dos propósitos: 

1. Define la afirmación de forma sucinta y sin ambigüedades
2. Incrusta la afirmación en un dominio algebraico.

Esta incrustación es lo que permite el segundo paso de la aritmetización, que reduce la afirmación CI a una afirmación sobre el grado de un polinomio específico.

La representación algebraica que utilizamos tiene dos componentes principales: 

1. Una traza de ejecución
2. Un conjunto de restricciones polinómicas. 

La traza de ejecución es una tabla que representa los pasos del cálculo subyacente, donde cada fila representa un único paso. El conjunto de restricciones polinómicas se construye de forma que todas ellas se cumplan si y sólo si el trazado representa un cálculo válido. Aunque la traza de ejecución puede ser muy larga, trabajaremos con un conjunto sucinto de restricciones polinómicas.

### Traza de ejecución
El tipo de traza de ejecución que queremos generar debe tener la particularidad de ser sucintamente comprobable: cada fila puede verificarse basándose sólo en las filas que están cerca de ella en la traza, y se aplica el mismo procedimiento de verificación a cada par de filas. Esta característica afecta directamente al tamaño de la prueba. Para ejemplificar lo que entendemos por concisamente comprobable, volvamos al recibo del supermercado y añadamos otra columna para el total:

FOTO

Esta simple adición nos permite verificar cada fila individualmente, dada su fila anterior.

Podemos, por ejemplo, examinar estas dos filas:

FOTO

y estar convencido de que este paso concreto del cálculo (es decir, el número 16,41) es correcto, ya que `12,96+3,45=16,41`. Observe que se aplica la misma restricción a cada par de filas. Esto es lo que entendemos por restricciones sucintas.

### Restricciones polinómicas
Reescribamos el recibo del supermercado (con el total acumulado) en forma de tabla:

FOTO

Denotemos el valor de la celda en la i-ésima fila y j-ésima columna por Ai,j (denotaremos el subíndice de esta manera ya que [Medium no soporta LaTeX](https://austinstartups.com/medium-needs-to-support-latex-for-math-and-science-67559a93f731)). Ahora podemos reformular las condiciones de corrección como este conjunto de restricciones polinómicas:

FOTO

Se trata de restricciones polinómicas lineales en Ai,j. Si el conjunto de restricciones polinómicas que utilizamos son de alto grado, esto tiene un efecto adverso sobre la longitud de la prueba y el tiempo que se tarda en generarla. En consecuencia, lo mejor que podemos esperar son restricciones lineales. Nótese que (2) es en realidad una única restricción aplicada varias veces, y el tamaño total del conjunto es independiente de la longitud del comprobante.

En resumen, hemos tomado un problema de CI para verificar un recibo de supermercado y lo hemos transformado en una traza de ejecución sucintamente comprobable y en el correspondiente conjunto de restricciones polinómicas que se cumplen si y sólo si la suma total del recibo original es correcta.

Veamos un ejemplo más complejo.

## Conjetura de Collatz
En 1937, un matemático alemán llamado Lothar Collatz presentó una conjetura en el campo de la teoría de números. A primera vista, esta conjetura podría parecer simplemente un bonito enigma matemático, pero en realidad se trata de un difícil problema abierto en la teoría de números. A lo largo de los años llamó la atención de muchos matemáticos y adquirió muchos sinónimos (por ejemplo, la conjetura 3n + 1, la conjetura de Ulam, el problema de Kakutani y muchos más). Paul Erdős dijo una vez sobre esta conjetura: "Puede que las matemáticas no estén preparadas para este tipo de problemas".

Una secuencia de Collatz comienza con cualquier número entero positivo, y cada elemento posterior de la secuencia se obtiene a partir del anterior de la siguiente manera:

* Si el elemento anterior es par: se divide por 2.
* Si el elemento anterior es impar y mayor que 1: se multiplica por 3 y se añade 1.
* Si el elemento anterior es 1, basta.

Consideremos un ejemplo sencillo en el que el término inicial es 52:

* 52 -> 26 -> 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1.

**Conjetura de Collatz:** para cualquier número entero positivo con el que empecemos, la secuencia siempre llega a 1.

Desgraciadamente, resolver la conjetura de Collatz está fuera del alcance de esta entrada del blog. En su lugar, consideraremos el problema de verificar un cálculo que compruebe la conjetura para un número entero inicial concreto.

### Trazado de la ejecución de la secuencia de Collatz
La sentencia CI es: "Una secuencia de Collatz que comienza con 52, termina con 1 después de 11 iteraciones".

Sea A la traza de ejecución del cálculo de la secuencia. La fila i-ésima, denotada por Ai, representa el número i-ésimo de la secuencia. Todos los números se representan como cadenas binarias, para facilitar la expresión de la condición par/impar con polinomios. Ai,j es igual al j-ésimo bit menos significativo del i-ésimo número de la secuencia. Por ejemplo, A0=001011: el primer término es 52, su representación binaria es 110100 y luego invertimos el orden de los bits (la inversión del orden de los bits simplifica la indexación en la notación de las restricciones polinómicas).

He aquí la traza de ejecución de la secuencia de Collatz anterior que comienza con 52:

FOTO

Observe que aquí la traza tiene 6 columnas porque 6 bits son suficientes para representar incluso el mayor número de la secuencia. Si hubiéramos empezado la secuencia con 51, el siguiente número habría sido 154, por lo que la traza de dicha secuencia habría necesitado al menos 8 columnas.

### Las restricciones polinómicas de la secuencia de Collatz
Recordemos que las restricciones polinómicas que buscamos son tales que todas ellas se cumplen si y sólo si la traza A describe la secuencia de Collatz dada (empezando por 52, terminando por 1, y la transición entre dos filas consecutivas cualesquiera se hace correctamente). En nuestro ejemplo, la traza A es de tamaño 6x12, es decir, representa una secuencia de Collatz de 12 números de 6 bits. El conjunto de restricciones polinómicas es el siguiente (n=12, m=6):

FOTO

Repasemos cada una de las restricciones. Las tres primeras son sencillas:

1. Se cumple si y sólo si la primera fila es una representación binaria de 52.
2. Se cumple si y sólo si la última fila es una representación binaria de 1.
3. Se cumple si y sólo si la traza sólo contiene bits (un número es igual a su cuadrado si y sólo si es 0 ó 1).

El cuarto conjunto de restricciones define el corazón del cómputo sucinto de la secuencia, es decir, la conexión entre cada dos filas consecutivas. La capacidad de expresar las restricciones computacionales como un patrón recurrente de restricciones locales (es decir, sucintas), es fundamental para que el verificador sea exponencialmente más rápido que una repetición ingenua del cálculo.

Examinemos detenidamente las propias restricciones.

Para cualquier i<n-1, denotemos:

FOTO

Por lo tanto, para cada i<n-1, obtenemos la siguiente restricción:

FOTO

Ai,0 es el bit menos significativo del número i-ésimo, que determina su paridad como número entero, por lo que esta restricción describe la regla de la secuencia de Collatz.

En resumen, todas las restricciones se cumplen si y sólo si la traza representa un cálculo válido de una secuencia de Collatz.

Obsérvese que cualquier secuencia de Collatz de longitud n puede representarse mediante una traza de tamaño n*m, donde m es el número máximo de bits en la representación de un número de la secuencia, y las restricciones polinómicas correspondientes se modifican en consecuencia. Obsérvese que las restricciones polinómicas no crecen con n y m, sino que siguen siendo simples y concisas.

Dado un primer término específico para una secuencia de Collatz, un sencillo programa informático puede generar la traza de ejecución y las restricciones polinómicas.

## Conclusión
En este post hemos cubierto el primer paso en la aritmetización de las sentencias CI.

Hemos visto cómo una sentencia CI sobre una secuencia de Collatz puede transformarse en una traza de ejecución y en un conjunto de restricciones polinómicas sucintamente descritas. Se pueden utilizar métodos similares para transformar cualquier cálculo y, en general, cualquier sentencia CI se puede traducir a esta forma. Sin embargo, los detalles son muy importantes. Mientras que hay muchas maneras en las que una traza de ejecución (y un conjunto de restricciones polinómicas) puede describir un cálculo específico, sólo un puñado de ellas dan lugar a una pequeña prueba STARK que se puede construir de manera eficiente. Gran parte del esfuerzo en StarkWare se dedica a diseñar reducciones que conduzcan a buenas restricciones polinómicas, lo que llamamos AIR (Algebraic Intermediate Representation), ya que gran parte del rendimiento de nuestros sistemas depende de ello.

En la próxima entrada trataremos el segundo paso de la aritmetización: la transformación de la traza de ejecución y las restricciones polinómicas en un único polinomio, que se garantiza que es de bajo grado, si y sólo si el cálculo original es correcto.

---

**Traduciones realizadas por el equipo oficial de Starknet-Es, gracias a [Dimeyad](https://github.com/dimeyad) y [Nadai](https://github.com/Nadai2010) por estos aportes**

Kineret Segal & Shir Peled
StarkWare

---

