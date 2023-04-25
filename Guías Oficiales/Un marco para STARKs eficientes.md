<div align="center">
  <h1 style="font-size: larger;">
    <strong>Un marco para STARKs eficientes</strong> 
    </h1>  
 <h2 style="font-size: medium;">
    <severe>Combinación de pruebas probabilísticas y funciones hash</severe> 
    </h2>  
    <img src="" width="600">
</div>

## Temas
- [Nuestro objetivo: argumentos transparentes escalables](#Nuestro-objetivo-argumentos-transparentes-escalables)
- [Hoja de ruta para esta publicación](#Hoja-de-ruta-para-esta-publicación)
- [Pruebas criptográficas de PCPs, a través de la construcción Micali](#Pruebas-criptográficas-de-PCPs-a-través-de-la-construcción-Micali)
- [La transparencia proviene de la criptografía](#La-transparencia-proviene-de-la-criptografía)
- [La escalabilidad proviene de la prueba probabilística](#La-escalabilidad-proviene-de-la-prueba-probabilística)
- [IOPs: una nueva noción de prueba probabilística](#IOPs-una-nueva-noción-de-prueba-probabilística)
- [Publicaciones anteriores describen una IOP eficiente](#Publicaciones-anteriores-describen-una-IOP-eficiente)
- [Pruebas criptográficas a través de la construcción BCS](#Pruebas-criptográficas-a-través-de-la-construcción-BCS)
- [Conclusión](#Conclusión)


 Este es la quinta y última publicación de nuestra serie STARK Math.
 
En nuestra [primera publicación](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Comienza%20el%20viaje.md), presentamos la noción de responsabilidad inclusiva y su importancia para las cadenas de bloques sin permiso. También explicamos por qué las propiedades de escalabilidad de STARK permiten lograr esta noción. Recomendamos al lector leer este primer post antes de continuar.

En la segunda, tercera y cuarta publicación, nos sumergimos en la teoría matemática detrás de las STARKs. Explicamos cómo [transformar sentencias computacionales en restricciones polinómicas](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Aritmetizaci%C3%B3n%20I.md). Explicamos cómo [reducir la verificación de restricciones polinómicas a la verificación de bajos grados](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Aritmetizaci%C3%B3n%20II.md). Y describimos una [prueba eficiente para la baja gradación](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Prueba%20de%20bajo%20grado.md). A continuación, no asumimos familiaridad con estas publicaciones, excepto por una sola sección que explica cómo los protocolos descritos en estas publicaciones pueden verse como un cierto tipo de prueba probabilística (cuyos detalles no importan en esta publicación).

En esta publicación, concluimos la serie revisando la noción de STARK y luego explicando cómo construir un STARK a partir de **pruebas probabilísticas** y **funciones hash criptográficas**.

## Nuestro objetivo: argumentos transparentes escalables

Deseamos construir un STARK, que es una prueba criptográfica (también conocida como un "argumento") que es tanto escalable como transparente. 
Nuestra [primera publicación](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Comienza%20el%20viaje.md) describe este objetivo en detalle, que ahora resumimos. 

Consideramos pruebas criptográficas para las declaraciones de integridad computacional (CI por sus siglas en inglés). Esto significa que, para un programa dado 𝐀, entrada x, salida "y", y tiempo limitado T, deseamos producir una cadena de prueba π que certifique la declaración.

“𝐀(x) da salida a "y" dentro de T pasos de cálculo”.

Ningún procedimiento (eficiente) debería ser capaz de producir pruebas que parezcan válidas para afirmaciones falsas (por ejemplo, reclamar 𝐀(x)=1 cuando en cambio 𝐀(x)=0). Son posibles declaraciones de CI más generales, donde 𝐀 además toma una entrada auxiliar privada, pero ignoramos esto en esta publicación.

(IMAGEN)

Las STARKs son pruebas criptográficas que cumplen las siguientes propiedades deseables:

* **Escalabilidad**: el tiempo necesario para producir 𝛑 es T·polylog(T), mientras que el tiempo necesario para validar 𝛑 es solo polylog(T); en particular, la longitud de 𝛑 es polylog(T). En otras palabras, producir pruebas no es mucho más costoso que ejecutar el cálculo original, y validar las pruebas es **exponencialmente más rápido** que ejecutar el cálculo original. Las pruebas son **exponencialmente más cortas** que el tamaño del cálculo original.
* **Transparencia**: los parámetros globales del sistema de prueba criptográfico, utilizados para producir y validar pruebas, no tienen **trampillas**. Esto contrasta con los sistemas de prueba que se basan en una parte confiable para muestrear parámetros públicos basados en información secreta, que puede usarse como una "trampilla" para comprometer la seguridad del sistema de prueba.

Al final de esta publicación, tendrá una comprensión informal de dónde provienen las propiedades de escalabilidad y transparencia.

Notamos brevemente aquí que las STARKs solo utilizan criptografía simétrica ligera (funciones hash criptográficas), lo que los hace **rápidos** y (plausiblemente) **seguros post-cuánticos**. Esto contrasta con muchas otras pruebas criptográficas, cuya seguridad se basa en la criptografía de clave pública que es costosa e insegura contra los adversarios cuánticos. 

## Hoja de ruta para esta publicación

Las construcciones STARK conocidas combinan dos ingredientes:

1. pruebas largas que pueden verificarse mediante controles locales aleatorios y económicos; y
2. una [función hash criptográfica](https://en.wikipedia.org/wiki/Cryptographic_hash_function) como SHA-256 o Keccak.

Informalmente, el primer componente le da a STARK su escalabilidad, mientras que el segundo componente le da a STARK su transparencia (sin impedir la escalabilidad). En las próximas secciones elaboramos el plan anterior.

* Primero, describimos la construcción de Micali, un método para construir pruebas criptográficas que sigue el modelo anterior. Los sistemas de prueba que subyacen a esta construcción se conocen como pruebas verificables probabilísticamente (PCPs por sus siglas en inglés). Describimos esta construcción por razones pedagógicas: es un elegante caso especial de la construcción que usamos.
* En segundo lugar, explicamos por qué la construcción de Micali proporciona transparencia y escalabilidad.
* En tercer lugar, describimos la noción de Interactive Oracle Proofs (IOPs), que generaliza las PCPs y permite diseñar protocolos mucho más eficientes. También explicamos cómo la aritmetización y los protocolos de prueba de bajo grado en publicaciones anteriores son IOPs.
* Finalmente, mencionamos la construcción BCS, una extensión de la construcción Micali que utiliza la noción más general de IOP en lugar de PCP. **Las STARKs eficientes, incluidos los que usamos, se obtienen a través de la construcción BCS**.

## Pruebas criptográficas de PCPs, a través de la construcción Micali

En esta sección, presentamos las [pruebas verificables probabilísticamente](https://en.wikipedia.org/wiki/Probabilistically_checkable_proof) (PCPs), un tipo de sistema de prueba que implica una **verificación local aleatoria de una prueba larga**. Luego explicamos cómo se puede combinar un PCP con criptografía ligera para obtener un STARK. Esta construcción proviene de un artículo de [Silvio Micali](https://dl.acm.org/citation.cfm?id=586984) (basado en un trabajo anterior de [Joe Kilian](https://dl.acm.org/citation.cfm?id=129782)).

Una prueba verificable probabilísticamente (PCP) es un **protocolo** entre un probador PCP y un verificador PCP que permite establecer la exactitud de las declaraciones de integridad computacional (CI) a través de una **verificación local aleatoria en una prueba larga**. Dada una declaración de CI (𝐀,x,y,T), el demostrador PCP produce una cadena de prueba 𝚿 que "codifica" el seguimiento de cálculo de la declaración de CI. Si bien la prueba 𝚿 es más larga que la traza de cálculo de pasos T (la longitud de 𝚿 es cuasilineal en T), 𝚿 tiene la característica notable de que puede validarse a través de una prueba probabilística que lee solo una pequeña parte de 𝚿. Es decir, dada la misma declaración de CI (A,x,y,T), el verificador PCP puede validar 𝚿 leyendo aleatoriamente una pequeña cantidad de ubicaciones de 𝚿 y luego ejecutando una "verificación local" económica en los valores leídos. (¡El número de ubicaciones de lectura puede ser una pequeña constante, como 3, independiente de T!) Si la declaración de CI es verdadera, el verificador siempre acepta. Si, en cambio, la declaración de CI es falsa, entonces el verificador la rechaza con alta probabilidad, independientemente de cómo se haya elegido la cadena de prueba 𝚿. Consulte la Figura 2 para ver un diagrama.

(IMAGEN)

Recuerda que nuestro objetivo es producir demostraciones 𝛑 que sean breves y rápidas de validar. Esto es bastante diferente de los PCP, que en cambio implican controles locales económicos para pruebas largas 𝚿. Entonces, ¿cómo pasamos de 𝚿 a 𝛑?

Una idea natural que puede venir a la mente es hacer que el probador haga una muestra previa de una vista local de 𝚿, en nombre del verificador, y luego envíe esta vista local como prueba 𝛑. Más detalladamente, el probador simula una verificación local aleatoria del verificador PCP en la prueba larga 𝚿 y luego incluye en una prueba 𝛑 solo ubicaciones de 𝚿 leídas a través de esta verificación local; el probador también incluye en 𝛑 la aleatoriedad 𝛒 utilizada para el verificador PCP. (Tenga en cuenta que 𝛑 es corto porque el número de ubicaciones de lectura de 𝚿 es pequeño). La intuición del muestreo previo es que, para validar 𝛑, se podría ejecutar el verificador PCP con la misma aleatoriedad 𝛒, lo que provocaría la lectura las mismas posiciones de 𝚿, que estaban incluidas en 𝛑. Esta atractiva idea, sin embargo, es defectuosa. Primero, un probador que hace trampa puede incluir en 𝛑 una opción de "aleatoriedad" 𝛒 que, de hecho, no es aleatoria. En segundo lugar, un probador que hace trampa puede elegir respuestas a las consultas del verificador PCP que dependen de 𝛒 para pasar la verificación local del verificador PCP. Esto es posible porque la seguridad de un PCP se basa en que 𝚿 sea inmutable, es decir, fijo antes de que se elija la verificación local aleatoria del verificador.

Los problemas anteriores se pueden abordar mediante el uso de cualquier función hash criptográfica H, como SHA-256 (modelada como un oráculo aleatorio), para realizar un premuestreo seguro. Aquí "seguro" significa que el probador podrá convencer al verificador de que la información incluida en la prueba corta 𝛑 es una verificación local aleatoria "honesta" que se ejecutó en alguna prueba larga 𝚿.

De manera informal, como se muestra en la Figura 3, se espera que el probador use la función hash H para comprometerse con la prueba larga 𝚿 a través de un árbol de Merkle ([Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) en inglés) y luego obtenga la aleatoriedad 𝛒 usando H para hacer hash de la raíz del árbol de Merkle. Esto asegura que la aleatoriedad 𝛒 sea "como aleatoria" (ya que 𝛒 es una salida de la función hash H) y también asegura que 𝛒 se elija después de que el probador se haya comprometido con 𝚿. Ahora el probador puede realizar un premuestreo como se describe arriba, es decir, simula una verificación local aleatoria del verificador PCP en la aleatoriedad 𝛒, para determinar qué ubicaciones de 𝚿 deben incluirse en 𝛑. Finalmente, el probador "se libera" de las ubicaciones elegidas, al incluir en 𝛑 una ruta de autenticación para cada ubicación elegida (la ruta de autenticación para una ubicación consta de los hermanos de los nodos en la ruta desde la ubicación hasta la raíz). Las rutas de autenticación demuestran que los valores reclamados de 𝚿 son consistentes con la raíz de Merkle y, en particular, no fueron elegidos por el probador después de que la aleatoriedad 𝛒 se derivara de la raíz. En general, la prueba corta 𝛑 solo incluirá la raíz reclamada del árbol de Merkle, los valores reclamados de 𝚿 para las ubicaciones seleccionadas y las rutas de autenticación para cada uno de estos valores (en relación con la raíz). A continuación, se puede validar 𝛑 comparando todas las rutas de autenticación de los valores declarados con la raíz, volviendo a derivar la aleatoriedad 𝛒 de la raíz y comprobando que el verificador PCP acepta los valores declarados cuando se ejecuta con la aleatoriedad 𝛒.¹

Para resumir, hemos utilizado la función hash H para realizar un “pre-muestreo seguro” que permite incluir en la prueba corta 𝛑 una única verificación local aleatoria de la prueba larga 𝚿.

[1] : Esta "justificación" de por qué funciona el muestreo previo seguro es solo intuición, y obtener una prueba formal de seguridad requiere algo de trabajo. Por ejemplo, un probador malicioso podría intentar comprometerse con muchas pruebas diferentes 𝚿 en busca de una elección "favorable" de aleatoriedad 𝛒, y luego incluir esta elección favorable en 𝛑. Una prueba de seguridad tendría que establecer que dichos probadores, y de hecho cualquier probador eficiente, fallarán con alta probabilidad.

(IMAGEN)

## La transparencia proviene de la criptografía

Una característica importante de la construcción de Micali es que la única criptografía necesaria para producir o validar una prueba corta 𝛑 es una [función hash criptográfica](https://en.wikipedia.org/wiki/Cryptographic_hash_function) H (por ejemplo, SHA-256 o Keccak). La elección de H es, por lo tanto, el único “parámetro global” que todos los usuarios del sistema de prueba deben conocer, y esta elección puede hacerse a través de información pública. Esto significa que las pruebas criptográficas obtenidas a través de la construcción Micali son **transparentes**.

Lo anterior contrasta con otras pruebas criptográficas en las que producir o validar pruebas requiere el uso de parámetros globales que se producen en base a información secreta. Para aquellos familiarizados con las pruebas criptográficas basadas en emparejamientos, un ejemplo típico de parámetros globales es

(G, 𝛂·G, 𝛂²·G, 𝛂³·G, …)

donde G es un elemento de grupo y 𝛂 es un escalar secreto. Dichos parámetros globales deben ser muestreados por una parte confiable o mediante una [ceremonia de múltiples partes](https://z.cash/technology/paramgen/) porque los usuarios no deben conocer la "trampilla" 𝛂. De hecho, saber 𝛂 permitiría producir pruebas aparentemente válidas para afirmaciones falsas.

## La escalabilidad proviene de la prueba probabilística

Otra característica importante de la construcción de Micali es que el tiempo para producir/validar la prueba corta 𝛑 está cerca del tiempo para producir/validar la prueba larga 𝚿. Esto se debe simplemente a que las operaciones criptográficas necesarias son económicas en comparación con las operaciones PCP. Así aprendemos que la eficiencia en la construcción de Micali está determinada esencialmente por la eficiencia del PCP subyacente. En particular, si el PCP es escalable (producir 𝚿 toma un tiempo cuasilineal en T mientras que validar 𝚿 es exponencialmente más rápido), entonces la construcción de Micali produce una prueba criptográfica escalable. Se conocen construcciones de PCPs escalables (ver [este artículo](https://eccc.weizmann.ac.il//report/2012/045/)).

Desafortunadamente, los costos de los PCP siguen siendo muy altos, lo que los hace inadecuados para el uso práctico. Debido a esto, las implementaciones STARKs no se basan en PCP a través de la construcción de Micali. En cambio, se basan en otro tipo de prueba probabilística, por lo que se puede lograr escalabilidad, con buenos costos concretos e incluso con conocimiento cero. Discutimos esto a continuación.

## IOPs: una nueva noción de prueba probabilística

Las STARKs eficientes se basan en un tipo de sistema de prueba probabilístico conocido como Pruebas Oraculares Interactivas (IOP, por sus siglas en inglés), que se introdujo en 2015. De manera informal, un probador y un verificador participan en un protocolo interactivo en el que, en cada ronda, el verificador envía algo de aleatoriedad 𝛔ᵢ al probador, y el probador responde con una prueba larga 𝚿ᵢ. Al final de la interacción, el verificador realiza una verificación local aleatoria de todas las pruebas largas (𝚿₁,𝚿₂,…) enviadas por el probador a lo largo de la interacción. Consulte la Figura 4 para ver un diagrama. Tenga en cuenta que un PCP es simplemente un "IOP no interactivo" y, por lo tanto, es un caso restringido.

(IMAGEN)

En los últimos años, los investigadores han desarrollado numerosos principios de diseño para construir IOP altamente eficientes: [BCGV16](https://eprint.iacr.org/2016/021)], [BCGRS16](https://eprint.iacr.org/2016/324), [BB+16](https://eprint.iacr.org/2016/646), [BBGR16](https://eccc.weizmann.ac.il/report/2016/149/), [BCFGRS16](https://eprint.iacr.org/2016/988), [BBHR17](https://eccc.weizmann.ac.il/report/2017/134/), [BBHR18](https://eprint.iacr.org/2018/046), [BCRSVW18](https://eprint.iacr.org/2018/828), [BKS19](https://eccc.weizmann.ac.il/report/2018/090/), [BGKS19](https://eccc.weizmann.ac.il/report/2019/044/). El protocolo IOP que usamos en nuestras construcciones STARKs está más estrechamente relacionado con [BBHR18](https://eprint.iacr.org/2018/046).

## Publicaciones anteriores describen una IOP eficiente

Ahora explicamos cómo nuestras publicaciones anteriores sobre aritmetización y pruebas de bajo grado en realidad describieron una IOP eficiente. En la Figura 5 proporcionamos un diagrama de esta IOP. La primera fase del protocolo es la aritmetización, que transforma el enunciado CI dado (𝐀,x,y,T) en un problema que implica establecer límites de grado en ciertos polinomios. La segunda fase es la prueba de bajo grado, que resuelve este último problema. Resumimos el flujo de trabajo del protocolo.

Aritmetización (área azul en la Figura 5). El probador y el verificador transforman el programa 𝐀 en una colección de restricciones polinómicas, como se describe en nuestra publicación Aritmetización I. Además, el probador ejecuta el cálculo descrito por (𝐀,x,y,T), obteniendo una traza de cálculo de paso T.

Luego, como se describe en nuestra publicación [Aritmetización II](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Aritmetizaci%C3%B3n%20II.md), el probador codifica esta traza para obtener una traza codificada 𝚽, que se envía al verificador. (Aquí, el probador no necesita recibir ninguna aleatoriedad del verificador antes de enviar 𝚽). Después de eso, el verificador envía aleatoriedad 𝛔₀, lo que permite que tanto el probador como el verificador "agrupan" todas las restricciones polinómicas en una sola restricción polinomial, tomando su combinación lineal aleatoria. El probador combina este último con la traza codificada 𝚽 para obtener un polinomio compuesto 𝚵, que se envía al verificador. El verificador asegura a través de una verificación de consistencia local que 𝚽 y 𝚵 están adecuadamente relacionados. En este punto, si la verificación de consistencia local pasa con alta probabilidad, la declaración de CI es verdadera si y solo si 𝚽 y 𝚵 tienen un grado bajo.

Pruebas de bajo grado (área gris en la Figura 5). El probador usa el protocolo FRI (descrito en nuestra [publicación de pruebas de bajo grado](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Oficiales/Prueba%20de%20bajo%20grado.md)) para convencer al verificador de que 𝚽 y 𝚵 son evaluaciones de polinomios de bajo grado. Esto implica participar en un protocolo en el que, en cada ronda, el verificador envía un 𝛔ᵢ aleatorio y el probador responde con una prueba auxiliar 𝚿ᵢ, y al final del protocolo el verificador realiza una verificación local aleatoria de 𝚽, 𝚵 y los 𝚿ᵢ. Si el verificador del protocolo FRI acepta con alta probabilidad entonces 𝚽 y 𝚵 tienen los grados deseados. Si es así, el verificador concluye que la afirmación CI (𝐀,x,y,T) es una afirmación verdadera.

(IMAGEN)

## Pruebas criptográficas a través de la construcción BCS

Los STARK eficientes se obtienen mediante la construcción de BCS, un método que combina un IOP con una función hash criptográfica H para obtener una prueba criptográfica. Este método es una extensión de la construcción de Micali y conserva sus características (de transparencia y escalabilidad) que discutimos anteriormente. No describimos la construcción BCS en esta publicación, y solo comentamos que se puede considerar como la aplicación de la construcción Micali en cada ronda de un IOP y colocar los compromisos de cada ronda en una cadena hash (que mantiene las rondas en orden).

## Conclusión

En este post hemos explicado que las construcciones STARK eficientes se obtienen combinando IOP eficientes (un tipo de prueba probabilística) y funciones hash criptográficas. El IOP le confiere al STARK su escalabilidad, mientras que la función hash le confiere al STARK su transparencia. Las diferentes construcciones de STARK difieren en las IOP subyacentes, y hemos explicado cómo nuestras publicaciones anteriores describieron los componentes del protocolo IOP que se usa en nuestra construcción de STARK.

Esto concluye nuestra serie de Matemáticas STARK. ¡Esperamos que haya sido valioso para usted!

Alessandro Chiesa y Gideon Kaempfer.

