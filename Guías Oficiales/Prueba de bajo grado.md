<div align="center">
  <h1 style="font-size: larger;">
    <strong>Pruebas de bajo grado</strong> 
    </h1>
    <img src="" width="600">
</div>

# Pruebas de bajo grado

## Temas

- [Pruebas de bajo grado](#Pruebas-de-bajo-grado)
- [La prueba directa](#La-prueba-directa)
  - [El caso de una función constante (d=1)](#El-caso-de-una-función-constante-(d=1))
  - [El caso de una función lineal (d=2)](#El-caso-de-una-función-lineal-(d=2))
- [La prueba directa no nos basta](#La-prueba-directa-no-nos-basta) 
- [Un Prover viene al rescate](#Un-Prover-viene-al-rescate)
- [Reducción a la mitad del número de consultas para el caso de dos polinomios](#Reducción-a-la-mitad-del-número-de-consultas-para-el-caso-de-dos-polinomios)
- [División de un polinomio en dos polinomios de grado menor](#División-de-un-polinomio-en-dos-polinomios-de-grado-menor) 
- [Protocolo FRI](#Protocolo-FRI)
  - [Fase de compromiso](#Fase-de-compromiso)
  - [Fase de consulta](#Fase-de-consulta)
- [Conclusión](#Conclusión)

## Parte 4
Esta es la cuarta publicación de nuestra serie STARK Math. Si aún no lo ha hecho, le recomendamos que lea los posts 1 ([Comienza el viaje]()), 2 ([Aritmetización I]()) y 3 ([Aritmetización II]()) antes de leer este. Hasta ahora hemos explicado cómo, en STARK, el proceso de Aritmetización nos permite reducir el problema de la Integridad Computacional a un problema de prueba de bajo grado (low degree testing, en inglés).

### La salsa secreta de la concisión
Las pruebas de bajo grado se refieren al problema de decidir si una función determinada es un polinomio de algún grado acotado, realizando solo un pequeño número de consultas a la función. Las pruebas de bajo grado se han estudiado durante más de dos décadas y son una herramienta central en la teoría de las pruebas probabilísticas. El objetivo de esta publicación es explicar las pruebas de bajo grado con más detalle y describir FRI, el protocolo que usamos para las pruebas de bajo grado en STARK. Esta publicación asume familiaridad con polinomios sobre campos finitos.

Para hacer que esta publicación tan matemática sea más fácil de digerir, marcamos ciertos párrafos así:

> los párrafos que contengan pruebas o explicaciones que no sean necesarias para comprender el panorama general se marcarán así 

## Pruebas de bajo grado

Antes de discutir las pruebas de bajo grado, primero presentamos un problema un poco más simple como calentamiento: Se nos da una función y se nos pide que decidamos si esta función es igual a algún polinomio de grado menor que alguna constante d, consultando la función en un número "pequeño" de ubicaciones. Formalmente, dado un subconjunto L de un campo F y un límite de grado d, deseamos determinar si f:L➝F es igual a un polinomio de grado menor que d, es decir, si existe un polinomio

<div align="center">
<img src="" width="700">
</div>

Sobre F para la cual _p_(_a_) = _f_(_a_) por cada a en L. para valores concretos, puede pensar en un campo de tamaño que es muy grande, digamos 2¹²⁸, y L, que es de tamaño de aproximadamente 10,000,000.

Resolver este problema requiere una consulta f en todo el dominio L, ya que f podría estar de acuerdo con un polinomio en todas partes en L, excepto en una ubicación única. Incluso si permitimos la probabilidad constante de error, el número de consultas seguirá siendo lineal en el tamaño de L.

Por esta razón, el problema de las pruebas de bajo grado en realidad se refiere a una relajación aproximada del problema anterior, que es suficiente para construir pruebas probabilísticas y también se puede resolver con una serie de consultas que es logarítmica en | L | (Tenga en cuenta que si L≈10,000,000, entonces log₂ (l) ≈23).

- **La función f es igual a un polinomio de bajo grado**. A saber, existe un polinomio _p_(_x_)_ sobre F, de grado menor que D, que está de acuerdo con f en todas partes en L.
- **La función f está lejos de TODOS los polinomios de bajo grado**. Por ejemplo, necesitamos modificar al menos el 10% de los valores de f antes de obtener una función que concuerde con un polinomio de grado menor que d.

Tenga en cuenta que existe otra posibilidad: la función f puede estar levemente cerca de un polinomio de bajo grado, pero no igual a uno. Por ejemplo, una función en la que el 5% de los valores difieren de un polinomio de bajo grado no cae en ninguno de los dos casos descritos anteriormente. Sin embargo, el paso previo de aritmetización (discutido en nuestras publicaciones anteriores) asegura que nunca surja el tercer caso. Más detalladamente, la aritmetización muestra que un prover honesto que trate con una afirmación verdadera caerá en el primer caso, mientras que un prover (posiblemente malicioso) que intente "probar" una afirmación falsa caerá, con alta probabilidad, en el segundo caso.

Para distinguir los dos casos, utilizaremos una prueba probabilística de tiempo polinomial que consulta f en un pequeño número de ubicaciones (discutiremos qué significa "pequeño" más adelante).

> Si f es de grado bajo, entonces la prueba debería aceptar con probabilidad 1. Si, en cambio, f está lejos de ser de bajo grado, entonces la prueba debería rechazar con probabilidad alta. En términos más generales, buscamos la garantía de que si f es δ-lejos de cualquier función de grado inferior a d (es decir, uno debe modificar al menos δ|L| ubicaciones para obtener un polinomio de grado menor que d), entonces la prueba rechaza con probabilidad al menos Ω(δ) (o alguna otra función "agradable" de δ). Intuitivamente, cuanto más cerca de cero está δ, más difícil es distinguir entre los dos casos. 

En las siguientes secciones describimos una prueba sencilla, luego explicamos por qué no es suficiente en nuestro entorno y, por último, describimos una prueba más compleja que es exponencialmente más eficiente. Esta última prueba es la que utilizamos en STARK.

## La prueba directa 

La primera prueba que consideramos es sencilla: comprueba si una función es (cercana a) un polinomio de grado inferior a d, utilizando d+1 consultas. La prueba se basa en un hecho básico sobre los polinomios: _cualquier polinomio de grado menor que d está completamente determinado por sus valores en cualquier d ubicaciones distintas de F_. Este hecho es una consecuencia directa de que un polinomio de grado k puede tener a lo sumo k raíces en F. Es importante destacar que el número de consultas, que es d+1, puede ser significativamente menor que el tamaño del dominio de f, que es |L|.

Primero discutiremos dos casos especiales sencillos, para intuir cómo funcionará la prueba en el caso general.

- **El caso de una función constante (d=1)**. Esto corresponde al problema de distinguir entre el caso en que f es una función constante (f(x)=c para algún c en F), y el caso donde f está lejos de cualquier función constante. El caso de una función constante (d=1). Esto corresponde al problema de distinguir entre el caso donde f es una función constante (f(x)=c para algún c en F), y el caso donde f está lejos de cualquier función constante. En este caso especial, existe una prueba natural de 2 consultas que podría funcionar: consulta f en una ubicación fija z1 y también en una ubicación aleatoria w, y luego verifica que f(z1)=f(w). Intuitivamente, f(z1) determina el (supuesto) valor constante de f, y f(w) prueba si todo f está cerca de este valor constante o no.
- **El caso de una función lineal (d=2)**. Esto corresponde al problema de distinguir entre el caso donde f es una función lineal (f(x)=ax+b para algún a,b en F), y el caso en que f se aleja de cualquier función lineal. En este caso especial hay una prueba natural de 3 consultas que podría funcionar: consultar f en dos puntos fijos z1,z2 y también en un punto aleatorio w, y luego comprobar que (z1,f(z1)), (z2,f(z2)), (w,f(w)) son colineales, es decir, que podemos trazar una recta a través de estos puntos. Intuitivamente, los valores de f(z1) y f(z2) determinan la (supuesta) recta, y f(w) comprueba si todo f está cerca de esta recta o no.

**Los casos especiales anteriores sugieren una prueba para el caso general de un límite de grado d**. Consulte f en d posiciones fijas z1,z2,...,zd y también en una posición aleatoria w. Los valores de f en z0,z1,...,zd definen un polinomio único h(x) de grado menor que d sobre F que coincide con f en estos puntos. La prueba comprueba entonces que h(w)=f(w). A esto lo llamamos prueba directa.

Por definición, si f(x) es igual a un polinomio p(x) de grado menor que d, entonces h(x) será idéntico a p(x) y, por tanto, la prueba directa pasa con probabilidad 1. Esta propiedad se denomina "completitud perfecta", y significa que esta prueba sólo tiene un error unilateral.

Nos queda argumentar qué ocurre si f es δ-lejos de cualquier función de grado menor que d. (Por ejemplo, piense en δ=10%.) Ahora argumentamos que, en este caso, la prueba directa rechaza con probabilidad al menos δ. En efecto, sea 𝞵 la probabilidad, sobre una elección aleatoria de w, de que h(w)≠f(w). Observe que 𝞵 debe ser al menos δ.

> Esto se debe a que si suponemos hacia la contradicción que 𝞵 es menor que δ, entonces deducimos que f es δ-cerca de h, lo que contradice nuestra suposición de que f es δ-lejos de cualquier función de grado menor que d. 

## La prueba directa no nos basta

En nuestro caso, estamos interesados en probar funciones f:L➝F que codifican trazas de cálculo y, por tanto, cuyo grado d (y dominio L) son bastante grandes. Ejecutar simplemente la prueba directa, que realiza d+1 consultas, sería demasiado costoso. Para obtener el ahorro exponencial de STARK (en tiempo de verificación comparado con el tamaño de la traza de cálculo), necesitamos resolver este problema con sólo O(log d) consultas, que es exponencialmente menor que el límite de grado d.

Lamentablemente, esto es imposible porque si consultamos f en menos de d+1 lugares no podemos concluir nada.

> Una forma de ver esto es considerar dos distribuciones diferentes de funciones f:L➝F. En una distribución elegimos uniformemente un polinomio de grado exactamente d y lo evaluamos en L. En la otra distribución elegimos uniformemente un polinomio de grado menor que d y lo evaluamos en L. En ambos casos, para cualquier localización d z1,z2,...,zd, los valores f(z1),f(z2),...,f(zd) se distribuyen uniforme e independientemente. (Dejamos este hecho como ejercicio para el lector.) Esto implica que, desde el punto de vista de la información, no podemos distinguir estos dos casos, a pesar de que una prueba debería hacerlo (ya que los polinomios de la primera distribución deberían ser aceptados por la prueba, mientras que los de grado exactamente d están muy lejos de todos los polinomios de grado inferior a d y, por tanto, deberían ser rechazados). 

Parece que tenemos un desafío difícil de superar.

## Un Prover viene al rescate

Hemos visto que necesitamos d+1 consultas para probar que una función f:L➝F es cercana a un polinomio de grado menor que d, pero no podemos permitirnos tantas consultas. Evitamos esta limitación considerando un escenario ligeramente distinto, que nos resulta suficiente. En concreto, consideramos el problema de la comprobación de bajo grado cuando se dispone de un prover (probador, en español) para proporcionar información auxiliar útil sobre la función f. Veremos que en este escenario "asistido por prover" de comprobación de bajo grado podemos conseguir una mejora exponencial en el número de consultas, hasta O(log d).

Más en detalle, consideramos un protocolo realizado entre un prover y un verifier (verificador, en español), en el que el prover (no fiable) intenta convencer al verifier de que la función es de grado bajo. Por un lado, el prover conoce toda la función f que se está comprobando. Por otro lado, el verifier puede consultar la función f en un pequeño número de lugares, y está dispuesto a recibir ayuda del prover, pero NO confía en que el prover sea honesto. Esto significa que el prover puede hacer trampas y no seguir el protocolo. Sin embargo, si el prover hace trampa, el verifier tiene la libertad de "rechazar", independientemente de si la función f es de grado bajo o no. Lo importante aquí es que el verificador no estará convencido de que f es de grado bajo a menos que esto sea cierto.

Tenga en cuenta que la prueba directa descrita anteriormente es simplemente el caso especial de un protocolo en el que el prover no hace nada, y el verifier prueba la función sin ayuda. Para hacer algo mejor que la prueba directa tendremos que aprovechar la ayuda del prover de alguna manera significativa.

A lo largo del protocolo, el prover querrá permitir al verificador consultar funciones auxiliares en las ubicaciones que el verifier elija. Esto puede lograrse mediante compromisos, un mecanismo que discutiremos en una futura entrada del blog. Por ahora basta con decir que el prover puede comprometerse con una función de su elección a través de un árbol de Merkle, y posteriormente el verificador puede solicitar al prover que revele cualquier conjunto de ubicaciones de la función comprometida. La principal propiedad de este mecanismo de compromiso es que una vez que el prover se compromete con una función, debe revelar los valores correctos y no puede hacer trampas (por ejemplo, no puede decidir cuáles son los valores de la función después de ver las peticiones del verifier).

## Reducción a la mitad del número de consultas para el caso de dos polinomios

Empecemos con un ejemplo sencillo que ilustra cómo un prover puede ayudar a reducir el número de consultas en un factor de 2. Más adelante ampliaremos este ejemplo. Supongamos que tenemos dos polinomios f y g y queremos comprobar que ambos son de grado inferior a d. Si simplemente realizáramos la prueba directa individualmente sobre f y g, tendríamos que hacer 2 * (d + 1) consultas. A continuación describimos cómo, con la ayuda de un prover, podemos reducir el número de consultas a (d + 1) más un término de orden menor.

En primer lugar, el verifier muestrea un valor aleatorio 𝛼 del campo y lo envía al prover. A continuación, el verifier responde comprometiéndose a la evaluación en el dominio L (recordemos que L es el dominio de la función f) del polinomio h(x) = f(x) + 𝛼 g(x) (en otras palabras, el verifier calculará y enviará la raíz de un árbol de Merkle cuyas hojas son los valores de h en L). El verifier comprueba ahora que h tiene un grado menor que d, mediante la prueba directa, que requiere d+1 consultas.

Intuitivamente, si f o g tienen grado al menos d, entonces con alta probabilidad también lo tiene h. Por ejemplo, considere el caso donde el coeficiente de xⁿ en f no es cero para algún n≥d. Entonces, hay como máximo una opción de 𝛼 (enviada por el verifier) para la cual el coeficiente de xⁿ en h es cero, lo que significa que la probabilidad de que h tenga un grado menor que d es aproximadamente 1/|F|. Si el campo es lo suficientemente grande (por ejemplo, |F|>2¹²⁸), la probabilidad de error es insignificante.

Sin embargo, la situación no es tan sencilla. La razón es que, como hemos explicado, no podemos comprobar literalmente que h es un polinomio de grado menor que d. En su lugar, sólo podemos comprobar que h está cerca de tal polinomio. Esto significa que el análisis anterior no es exacto. ¿Es posible que f esté lejos de un polinomio de bajo grado y la combinación lineal h esté cerca de uno con una probabilidad no despreciable sobre 𝛼? Bajo condiciones suaves la respuesta es no (que es lo que queremos), pero está fuera del alcance de este post; remitimos al lector interesado a [este artículo](https://acmccs.github.io/papers/p2087-amesA.pdf) y a [este otro](https://eccc.weizmann.ac.il/report/2017/134/).

Además, ¿cómo sabe el verifier que el polinomio h enviado por el prover tiene la forma f(x)+𝛼 g(x)? Un prover malintencionado puede hacer trampas enviando un polinomio que sea efectivamente de grado bajo, pero que sea diferente de la combinación lineal que el verifier pidió. Si ya sabemos que h está cerca de un polinomio de bajo grado, entonces probar que este polinomio de bajo grado tiene la forma correcta es sencillo: el verifier muestrea una ubicación z en L al azar, consulta f, g, h en z, y comprueba que se cumple la ecuación h(z)=f(z)+𝛼 g(z). Esta prueba debería repetirse varias veces para aumentar la precisión de la prueba, pero el error se reduce exponencialmente con el número de muestras que hacemos. Por lo tanto, este paso aumenta el número de consultas (que hasta ahora era d+1) sólo en un término de orden menor.

## División de un polinomio en dos polinomios de grado menor

Hemos visto que, con la ayuda del prover, podemos probar que dos polinomios son de grado menor que d con menos de 2*(d+1) consultas. Ahora describimos cómo podemos convertir un polinomio de grado menor que d en dos polinomios de grado menor que d/2. 

Sea f(x) un polinomio de grado inferior a d y supongamos que d es par (en nuestro entorno esto se cumple sin pérdida de generalidad). Podemos escribir f(x)=g(x²)+xh(x²) para dos polinomios g(x) y h(x) de grado inferior a d/2. En efecto, podemos dejar que g(x) sea el polinomio obtenido a partir de los coeficientes pares de f(x), y que h(x) sea el polinomio obtenido a partir de los coeficientes impares de f(x). Por ejemplo, si d=6 podemos escribir

<div align="center">
<img src="" width="700">
</div>

lo que significa que

<div align="center">
<img src="" width="700">
</div>

y

<div align="center">
<img src="" width="700">
</div>

que es un algoritmo n*log(n) para la evaluación de polinomios (que mejora el algoritmo ingenuo n2). 

## Protocolo FRI

Ahora combinamos las dos ideas anteriores (probar dos polinomios con la mitad de consultas, y dividir un polinomio en dos más pequeños) en un protocolo que sólo utiliza O(log d) consultas para probar que una función f tiene (más precisamente, está cerca de una función de) grado menor que d. Este protocolo se conoce como FRI (que significa Fast Reed - Solomon Interactive Oracle Proof of Proximity), y el lector interesado puede leer más sobre él [aquí](https://eccc.weizmann.ac.il/report/2017/134/). Para simplificar, a continuación supondremos que d es una potencia de 2. El protocolo consta de dos fases: una _fase de compromiso_ y una _fase de consulta_.

## Fase de compromiso

El prover divide el polinomio original f₀(x)=f(x) en dos polinomios de grado menor que d/2, g₀(x) y h₀(x), satisfaciendo f₀(x)=g₀(x²)+xh₀(x²). El verifier muestrea un valor aleatorio 𝛼₀, lo envía al probador y le pide que se comprometa con el polinomio f₁(x)=g₀(x) + 𝛼₀h₀(x). Tenga en cuenta que f₁(x) es de grado menor que d/2.

Podemos continuar recursivamente dividiendo f₁(x) en g₁(x) y h₁(x), eligiendo un valor 𝛼₁, construyendo f₂(x) y así sucesivamente. Cada vez, el grado del polinomio se reduce a la mitad. Por lo tanto, después de log(d) pasos nos quedamos con un polinomio constante, y el prover puede simplemente enviar el valor constante al verifier.

Una nota sobre los dominios: para que el protocolo anterior funcione, necesitamos la propiedad de que para cada z en el dominio L, se cumple que -z también está en L. Además, el compromiso sobre f₁(x) no será sobre L sino sobre L²={x²: x ∊ L}. Dado que aplicamos iterativamente el paso FRI, L² también tendrá que satisfacer la propiedad {z, -z}, y así sucesivamente. Estos requisitos algebraicos naturales se satisfacen fácilmente a través de opciones naturales de dominios L (por ejemplo, un subgrupo multiplicativo cuyo tamaño es una potencia de 2), y de hecho coinciden con los que de todos modos necesitamos para beneficiarnos de algoritmos FFT eficientes (que se utilizan en otras partes de STARK, por ejemplo, para codificar las trazas de ejecución).

## Fase de consulta

Ahora tenemos que comprobar que el prover no ha hecho trampas. El verifier toma una muestra aleatoria z en L y consulta f₀(z) y f₀(-z). Estos dos valores son suficientes para determinar los valores de g₀(z²) y h₀(z²), como se puede ver por las siguientes dos ecuaciones lineales en las dos "variables" g₀(z²) y h₀(z²):

<div align="center">
<img src="" width="700">
</div>

El verifier puede resolver este sistema de ecuaciones y deducir los valores de g₀(z²) y h₀(z²). De ello se deduce que puede calcular el valor de f₁(z²), que es una combinación lineal de los dos. Ahora el verificador consulta f₁(z²) y se asegura de que es igual al valor calculado anteriormente. Esto sirve como indicación de que el compromiso con f₁(x), que fue enviado por el prover en la fase de compromiso, es realmente el correcto. El verifier puede continuar, consultando f₁(-z²) (recordemos que (-z²)∊ L² y que el compromiso sobre f₁(x) fue dado en L²) y deducir de él f₂(z⁴).

El verifier continúa de esta manera hasta que utiliza todas estas consultas para deducir finalmente el valor de f_{log d}(z) (denotando f con un subíndice log d, que no podemos escribir debido a la falta de soporte de Medium para notación matemática completa). Pero, recordemos que f_{log d}(z) es un polinomio constante cuyo valor constante fue enviado por el prover en la fase de compromiso, antes de elegir z. El verifier debe comprobar que el valor enviado por el prover es efectivamente igual al valor que el verifier calculó a partir de las consultas a las funciones anteriores.

En general, el número de consultas sólo es **logarítmico** en el límite de grado d. 

> Para hacernos una idea de por qué el prover no puede hacer trampas, consideremos el problema de juguete en el que f₀ es cero en el 90% de los pares de la forma {z,-z}, es decir, f₀(z) = f₀(-z) = 0 (llamémosles los pares "buenos"), y distinto de cero en el 10% restante (los pares "malos"). Con una probabilidad del 10%, la z seleccionada al azar cae en un par malo. Tenga en cuenta que sólo un 𝛼 dará lugar a f₁(z²)=0, y el resto dará lugar a f₁(z²)≠0. Si el prover engaña en el valor de f₁(z²), será pillado, por lo que suponemos lo contrario. Así, con una alta probabilidad (f₁(z²), f₁(-z²)) también será un mal par en la siguiente capa (el valor de f₁(-z²) no es importante ya que f₁(z²)≠0). Esto continúa hasta la última capa, donde el valor será distinto de cero con alta probabilidad. 

Por otro lado, dado que empezamos con una función con un 90% de ceros, es poco probable que el prover sea capaz de acercarse a un polinomio de bajo grado que no sea el polinomio cero (no probaremos este hecho aquí). En particular, esto implica que el prover debe enviar 0 como valor de la última capa. Pero entonces, el verificador tiene una probabilidad de aproximadamente el 10% de atrapar al prover. Esto fue sólo un argumento informal, el lector interesado puede encontrar una prueba rigurosa [aquí](https://eccc.weizmann.ac.il/report/2017/134/).

En la prueba descrita hasta ahora (y en el análisis anterior), la probabilidad de que el verificador descubra a un prover malicioso es sólo del 10%. En otras palabras, la probabilidad de error es del 90%. Esto puede mejorarse exponencialmente repitiendo la fase de consulta anterior para unas cuantas z muestreadas independientemente. Por ejemplo, eligiendo 850 z, obtenemos una probabilidad de error de 2^{-128}, que es prácticamente cero.

## Conclusión

En esta publicación hemos descrito el problema de las pruebas de bajo grado. La solución directa (test) requiere demasiadas consultas para lograr la brevedad (succinctness) requerida por STARK. Para lograr la complejidad de la consulta logarítmica, utilizamos un protocolo interactivo llamado FRI, en el que el prover agrega más información para convencer al verifier de que la función es de bajo grado. Fundamentalmente, FRI permite que el verifier resuelva el problema de las pruebas de bajo grado con una cantidad de consultas (y rondas de interacción) que es logarítmica en el grado prescrito. La próxima publicación resumirá las últimas tres publicaciones y explicará cómo podemos eliminar el aspecto interactivo utilizado en FRI y en partes anteriores del protocolo, para obtener pruebas sucintas no interactivas.

---

**Traduciones realizadas por el equipo oficial de Starknet-Es, gracias a [Dimeyad](https://github.com/dimeyad) y [Nadai](https://github.com/Nadai2010) por estos aportes**

Alessandro Chiesa & Lior Goldberg 
StarkWare

---


