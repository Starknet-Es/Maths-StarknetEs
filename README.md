<div align="center">
<img alt="starknet logo" src="https://github.com/Nadai2010/Nadai-Maths-Starks/blob/master/im%C3%A1genes/Starknet.jpg" width="600" >
  <h1 style="font-size: larger;">
    <img src="https://github.com/Nadai2010/Nadai-SHARP-Starknet/blob/master/im%C3%A1genes/Starknet.png" width="40">
    <strong>StarknetEs Basecamp - Maths Starks</strong> 
    <img src="https://github.com/Nadai2010/Nadai-SHARP-Starknet/blob/master/im%C3%A1genes/Starknet.png" width="40">
  </h1>
</div>

## Traducción Starknet Basecamp - Stark Maths

- Puede encontrar las notas originales [aquí](https://bit.ly/starkmaths2023)
- Puede encontrar Guías completas desde Starknet-Es [aquí](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular/Readme.md)

## Temas

- [Matemáticas de base](#matemáticas-de-base)
    - [Terminología](#terminología)
    - [Aritmética modular](#aritmética-modular)
    - [Teoría de grupos y campos](#teoría-de-grupos-y-campos)
    - [Subgrupos](#subgrupos)
    - [Grupos cíclicos y generadores](#grupos-cíclicos-y-generadores)
    - [Encontrar una inversa](#encontrar-una-inversa)
    - [Campos](#campos)
    - [Campos finitos y generadores](#campos-finitos-y-generadores)
    - [Polinomios](#polinomios)
    - [Lemma de Schwartz-Zippel](#lemma-de-schwartz-zippel)
    - [Interpolación de Lagrange](#interpolación-de-lagrange)
- [Sistemas de prueba de conocimiento cero](#sistemas-de-prueba-de-conocimiento-cero)
    - [¿Qué es una prueba de conocimiento cero?](#qué-es-una-prueba-de-conocimiento-cero)
    - [Actores en un sistema a prueba de conocimiento cero](#actores-en-un-sistema-a-prueba-de-conocimiento-cero)
    - [Tipos de sistema ZK](#tipos-de-sistema-zk)
    - [¿Qué exigimos de una prueba?](#qué-exigimos-de-una-prueba)
    - [Sucintos o Succinctness](#sucintos-o-succinctness)
    - [Sistema de comprobación idealizado para la integridad computacional](#sistema-de-comprobación-idealizado-para-la-integridad-computacional)
    - [Uso de polinomios y restricciones](#uso-de-polinomios-y-restricciones)
    - [Códigos Reed Solomon](#códigos-reed-solomon)
- [Integridad computacional](#integridad-computacional)
- [Starks](#starks)
    - [Visión general del proceso stark](#visión-general-del-proceso-stark)
    - [Uso de la aletoriedad](#uso-de-la-aletoriedad)
    - [Concisión y rendimiento](#concisón-y-rendimiento)
    - [Aritmetización](#aritmetización)
    - [Crear un polinomio para nuestra traza](#crear-un-polinomio-para-nuestra-traza)
    - [Polinomio de composición](#polinomio-de-composición)
    - [Ampliando nuestro polinomio](#ampliando-nuestro-polinomio)
    - [De las restricciones polinómicas al problema de las pruebas de bajo grado](#de-las-restricciones-polinómicas-al-problema-de-las-pruebas-de-bajo-grado)
    - [Pruebas de bajo grado](#pruebas-de-bajo-grado)
    - [FRI](#fri)
    - [Heurística Fiat-Shamir](#heurística-fiat-shamir)
    - [Cairo y el no determinismo](#cairo-y-el-no-determinismo)
- [Referencias y lecturas complementarias](#referencias-y-lecturas-complementarias)

## Matemáticas de base
### Terminología
* El conjunto de los números enteros se designa `ℤ`, por ejemplo, con {⋯,-4,-3,-2,-1,0,1,2,3,4,⋯}.
* El conjunto de los números racionales se designa `ℚ`, por ejemplo, con {...1,3/2,2,22/7...}.
* El conjunto de los números reales se designa `ℝ`, por ejemplo con {2, -4, 613, π, √ 2, ...}.

Los fields se denotan por `𝔽`, si son un campo finito o `𝕂` para un campo de números reales o complejos. 
También usamos `ℤ*ₚ` para representar un campo finito de enteros mod prime p con inversos multiplicativos.

Utilizamos campos finitos para la criptografía, porque los elementos tienen representaciones "cortas", exactas y propiedades útiles.

---

### Aritmética modular
[Véase esta introducción](https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/what-is-modular-arithmetic)

![Graph](/im%C3%A1genes/AritemticaModular.png)

Cuando escribimos n mod k nos referimos simplemente al residuo cuando n se divide por k. Así:

```bash
25 mod 3 = 1
15 mod 4 = 3
```
El resto debe ser positivo.

- [Guía Completa con ejemplos desde Starknet-ES](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Aritm%C3%A9tica%20Modular/Readme.md)

---

### Teoría de grupos y campos
Un grupo es un conjunto de elementos {a,b,c,...} (nos referimos a grupos de números, pero pueden ser cualquier cosa) más una operación binaria, que aquí representamos como `•`. 

Para ser considerada un grupo, esta combinación debe tener ciertas propiedades

1. Cierre.
    * Para todo a, b en G,  el resultado de la operación, a • b, también está en G
2. Asociatividad.
    * Para todos los a, b y c en G, (a • b) • c = a • (b • c)
3. Elementos de identidad.
    * Existe un elemento e en G tal que, para cada elemento a en G, la ecuación e • a = a • e = a se mantiene. Tal elemento es único y por lo tanto se habla del elemento identidad.
4. Elemento inverso.
    * Para cada a en G, existe un elemento b en G, comúnmente denotado α⁻¹ (o -a, si la operación se denota "+"), tal que a • b = b • a = e, donde e es el elemento identidad.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Subgrupos
Si un subconjunto de los elementos de un grupo también satisface las propiedades del grupo, entonces es un subgrupo del grupo original.

### Grupos cíclicos y generadores
Un grupo finito puede ser cíclico. Esto significa que tiene un elemento generador. Si se empieza en un punto cualquiera y luego se aplica la operación de grupo con el generador como argumento un cierto número de veces, se da la vuelta a todo el grupo y se termina en el mismo sitio, véase más abajo.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Encontrar una inversa
Del pequeño teorema de Fermat

![Graph](/im%C3%A1genes/inver.png)

Sea p = 7 y a = 2. Podemos calcular la inversa de a como:

![Graph](/im%C3%A1genes/inver2.png)

Esto es fácil de verificar: 2 x 4 ≡ 1 mod 7.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Campos
Un campo es un conjunto de números enteros junto con dos operaciones llamadas suma y multiplicación.

Un ejemplo de campo son los números reales sometidos a adición y multiplicación, otro es un conjunto de números enteros mod un número primo con adición y multiplicación. 

Se requiere que las operaciones de campo satisfagan los siguientes axiomas de campo. En estos axiomas, a, b y c son elementos arbitrarios del campo `𝔽`.

1. Asociatividad de la suma y la multiplicación: `a + (b + c) = (a + b) + c` y `a • (b • c) = (a • b) • c`.
2. Conmutatividad de la suma y la multiplicación: `a + b = b + a` y `a • b = b • a`
3. Identidad aditiva y multiplicativa: existen dos elementos diferentes 0 y 1 en `𝔽` tales que `a + 0 = a` y `a • 1 = a`.
4. Inversos aditivos: para cada a en F, existe un elemento en F, denotado -a, llamado inverso aditivo de a, tal que `a + (-a) = 0`.
5. Inversos multiplicativos: para cada a ≠ 0 en F, existe un elemento en F, denotado por α⁻¹, llamado inverso multiplicativo de a, tal que `a•α⁻¹ = 1`.
6. Distributividad de la multiplicación sobre la suma: `a•(b + c) = (a•b) + (a•c)`.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Campos finitos y generadores
Un campo finito es un campo con un conjunto finito de elementos, como el conjunto de enteros mod p donde p es un primo.

Para probar operaciones en campos finitos, [consulte aquí](https://asecuritysite.com/encryption/finite)

El orden del campo es el número de elementos del conjunto del campo.

Un elemento puede representarse como un número entero mayor o igual que 0 y menor que el orden del campo: {0, 1, ..., p-1} en un campo simple.

Todo campo finito tiene un generador. Un generador es capaz de generar todos los elementos del conjunto exponenciando el generador.
Así que para el generador g podemos tomar g⁰, g¹ ,g² y finalmente esto nos dará todos los elementos del grupo.

Por ejemplo, si tomamos el conjunto de los números enteros y el primo p = 5, obtenemos el grupo ℤ*₅ = {0,1,2,3,4}. 
En el grupo ℤ*₅ las operaciones se realizan en módulo 5; por lo tanto, no tenemos 3 × 4 = 12 sino 3 × 4 = 2, porque 12 mod 5 = 2.

ℤ*₅ es cíclico y tiene dos generadores, 2 y 3, porque `2¹ = 2, 2² = 4, 2³ = 3, 2⁴ = 1`, y `3¹ = 3, 3² = 4, 3³ = 2, 3⁴ = 1`

En un campo finito de orden 𝔮, el polinomio X elevado 𝔮 - X tiene todos los q elementos del campo finito como raíces.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Polinomios
Un polinomio es una ecuación de la forma

![Graph](/im%C3%A1genes/poli.png)

Donde los valores a son constantes, y x es una variable, si nuestro polinomio tiene una sola variable, se llama polinomio univariante.

Un hecho básico sobre los polinomios y sus raíces es que si `p(x)` es un polinomio, entonces `p(a) = 0` para algún valor específico `𝔞`

Si y sólo si existe un polinomio `q(x)` tal que `(x-a)q(x) = p(x)`, y por lo tanto 

![Graph](/im%C3%A1genes/poli2.png)

Esto es válido para todas las raíces, volveremos sobre ello más adelante.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Lemma de Schwartz-Zippel
"diferentes polinomios son diferentes en la mayoría de los puntos".

Los polinomios tienen una propiedad ventajosa, a saber, si tenemos dos polinomios no iguales de grado como máximo `d`, no pueden intersecarse en más de `d` puntos.

![Graph](/im%C3%A1genes/Zippel2.png)

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Interpolación de Lagrange
Si tienes un conjunto de puntos, al hacer una interpolación de Lagrange en esos puntos obtienes un polinomio que pasa por todos esos puntos.
Si tienes dos puntos en un plano, puedes definir una única recta que pase por ambos, para 3 puntos, una única curva de 2º grado `(por ejemplo, 5x2 + 2x + 1)` pasará por ellos, etc.
Para `n` puntos, puedes crear un polinomio de grado `n-1` que pase por todos los puntos.

![Graph](/im%C3%A1genes/Lagrange.png)

- [Guía Completa con ejemplos desde Starknet-ES]

---

## Sistemas de prueba de conocimiento cero
### Qué es una prueba de conocimiento cero
#### Una definición imprecisa
Es una prueba de que existe o de que sabemos algo, más un aspecto de conocimiento cero, es decir, la persona que verifica la prueba sólo obtiene una información: que la prueba es válida o inválida.

### Actores en un sistema a prueba de conocimiento cero
* Creador - opcional, puede combinarse con el prover
* Prover (probador)
* Verificador

El prover creará una prueba para convencer al verificador de que conoce un valor secreto (el testigo) o de que un cálculo se ha realizado correctamente.

El sistema de comprobación puede ser interactivo, en el que el prover y el verificador intercambian mensajes para verificar la prueba, o puede consistir únicamente en que el prover envíe la prueba al verificador, que puede aceptarla o rechazarla en un solo paso.

A menudo, la verificación será automática, realizada por un contrato inteligente en Ethereum, por ejemplo.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Tipos de sistema ZK
![Graph](/im%C3%A1genes/zk.png)

### ¿Qué exigimos de una prueba?
* Completitud: existe un prover honesto P que puede convencer al verificador honesto V de cualquier afirmación correcta con una probabilidad muy alta.

* Solidez: ni siquiera un prover deshonesto P que funcione en tiempo superpolinómico puede convencer a un verificador honesto V de una afirmación incorrecta.

* Para que nuestra prueba sea de conocimiento cero, también necesitamos "conocimiento cero".

Para simplificar demasiado: representado en un ordenador, un ZKP no es más que una secuencia de números, cuidadosamente calculados por Peggy, junto con un montón de comprobaciones booleanas que Victor puede ejecutar para verificar la prueba de corrección del cálculo.

Un protocolo de conocimiento-cero es, por tanto, el mecanismo utilizado para derivar estos números y definir las comprobaciones de verificación.

Nos interesa la integridad computacional (CI), por ejemplo, saber que el programa de Cairo
que escribiste se calculó correctamente.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Sucintos o Succinctness
Exigimos que nuestras pruebas y, en particular, el tiempo de verificación sean sucintos, es
decir, que sean órdenes de magnitud más pequeños que el tiempo necesario para calcular
nuestro programa; de lo contrario, no habría ninguna ventaja para el verificador en utilizar
la prueba en lugar del cálculo original.

### Sistema de comprobación idealizado para la integridad computacional

Hay muchas cosas que se omiten y se asumen aquí, esto es sólo para mostrar un proceso general.

Pasos:

1. Prover afirma Declaración _S_
2. El verificador proporciona `dᵢ...dₙ` los límites de grado de los polinomios deseados
3. El prover proporciona (o se compromete a) `Pᵢ..Pₖ`: polinomios limitados por esos grados
4. El verificador proporciona `z ∈ 0,..p - 1`
5. Prover proporciona evaluaciones de polinomios: `P₁(z)...Pₖz)`
6. El verificador decide si acepta _S_

Los grados esperados son típicamente alrededor de 10⁶ (todavía se considera bajo grado). Tenga en cuenta que la probabilidad de aceptar una prueba falsa es `< 10.d/p`, donde `p` es el tamaño del campo, por tanto del orden de `2⁻²³⁰` si nuestro campo finito tiene `p` de `~ 2²⁵⁶`.

Normalmente el número de consultas es de 3 - 10, mucho menos que el grado.

La única aleatoriedad que utilizamos aquí es el muestreo de `z` entre `0,..p-1`, en general la aleatoriedad que utilizamos en el proceso es esencial tanto para la concisión como para el conocimiento cero.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Uso de polinomios y restricciones
#### ¿Por qué no evalúa el verificador los propios polinomios?
- Porque, en realidad, el prover no envía todos los polinomios al verificador, si lo hiciera perderíamos concisión, contienen más información que nuestra declaración original, por lo que el prover sólo proporciona un compromiso con los polinomios.

#### ¿Qué propiedades de los polinomios son importantes en este caso?
1. Los polinomios son buenos códigos de corrección de errores.

Si tenemos polinomios de grado `d` sobre un dominio de codificación `D`, y dos mensajes `m1 y m2`, entonces m1 y m2 diferirán en `|D|-d` puntos. Esto es importante porque queremos que la diferencia entre una declaración correcta y una incorrecta sea grande, de modo que sea fácil de encontrar.

Esto conduce a un buen muestreo, lo que ayuda a la concisión, sólo necesitamos muestrear unos pocos valores para estar seguros de que la probabilidad de error es lo suficientemente baja como para ser insignificante.

2. Disponer de pruebas eficaces de lote cero. Esto también ayuda a
la concisión

Imaginemos que queremos demostrar que un polinomio de grado grande `P(x) (grado ~ 10 millones)` evalúa a cero en los puntos `1...1 millón`, pero queremos hacerlo con una sola consulta.

Imaginemos que nuestra afirmación es que P desaparece en estos puntos. Si el verificador sólo utiliza el muestreo, el prover podría hacer trampas fácilmente proporcionando un punto que se evalúe como cero, pero los otros 999.999 podrían ser distintos de cero.

Lo resolvemos

Tomar un conjunto S = 1...10⁶

Definir `V` como el único polinomio que desaparece en estos puntos, es
decir:
 `(x - 1)(x - 2)(x - 3)...` 
 el grado de `V=tamaño de S`, esto es bueno porque 
    1. `P(x)=P'(x)•V(x)`
    2. `Grado de P=grado de P - tamaño de S`

 Es la introducción de `V(x)` lo que nos permite comprobar en todo el dominio.
 
3. Tienen propiedad "multiplicadora". Podemos "envolver" una restricción alrededor de un polinomio.
 
Por ejemplo, si tenemos la restricción `C`, que nuestra evaluación siempre será un cero o un uno, podríamos escribirlo como `C(x)=x•(x-1)`
Se podría imaginar esto restringiendo una salida a ser un booleano, algo que puede ser útil para la integridad computacional.

Pero aquí en vez de ser x un simple punto podría ser la evaluación de un polinomio `P₁(x)` en un punto, es decir,
`C(P₁(x)) = P₁(x)•(P₁(x)-1)`

y los grados de los polinomios producidos por la multiplicación son entonces aditivos por lo que el grado de `C(x)=2. grado de P₁(x)`

Entonces podemos afirmar que si ` P₁(x)` cumple esta restricción para nuestro conjunto `S`, entonces, como antes, podemos decir que existe un polinomio `P'(x)` tal que

`C(P₁(x)) = P'(x)•V(x)`

Si `P₁(x)` no cumpliera la restricción (por ejemplo si para un valor de `x,P₁(x)= 93)` entonces no podríamos encontrar tales polinomios, la igualdad no se cumpliría y habría efectivamente un residuo en la ecuación anterior.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Códigos Reed Solomon
Véase [http://pfister.ee.duke.edu/courses/ecen604/rspoly.pdf](http://pfister.ee.duke.edu/courses/ecen604/rspoly.pdf)

Un código Reed-Solomon es un conjunto de vectores de longitud `n` (denominados codewords), en el que los elementos del vector (denominados símbolos) constan de `m` dígitos binarios. Nuestra única restricción es que `n` no debe ser mayor que `2m`. De los `n` símbolos de cada palabra de código, `k` llevan información y los otros `(n - k)` son símbolos redundantes.

Supongamos que, del total de `n` símbolos, exactamente `t` de ellos se reciben con error (y los otros `n - t` se reciben correctamente).

Los códigos Reed-Solomon tienen la notable propiedad de que si `t ≤ (n - k)/2`, la información correcta puede calcularse a partir de este codeword defectuoso.

Además, si `s` de los símbolos recibidos se borran (es decir, se etiquetan como probablemente defectuosos) y otros `t` símbolos se reciben erróneos, la información correcta puede calcularse a partir de la codeword defectuosa siempre que `s + 2t ≤ n - k`.

El dispositivo que reconstruye la información a partir del vector recibido se denomina decodificador.

- [Guía Completa con ejemplos desde Starknet-ES]

---

## Integridad Computacional 
Una de las características (notables) de los sistemas de prueba de conocimiento cero es que pueden utilizarse para demostrar que algún cálculo se ha realizado correctamente.
Por ejemplo, si tenemos un programa cairo que comprueba que un prover conoce la raíz cuadrada de 25, puede ejecutar el programa para comprobarlo, pero el verificador necesita saber que el cálculo se ha realizado correctamente.

La cuestión de la concisión es importante aquí, queremos que el tiempo necesario para verificar el cálculo sea sustancialmente menor que el tiempo necesario para ejecutar el cálculo, de lo contrario el verificador se limitaría a repetir el cálculo.

En la L2 de Starknet nos preocupa principalmente que un lote de transacciones se haya ejecutado correctamente dando lugar a un cambio de estado válido. Los participantes en la L1 desean comprobarlo sin necesidad de ejecutar ellos mismos todas las transacciones.

En el contexto de Starknet, la integridad computacional es más importante que el conocimiento cero, todos los datos de Starknet son públicos.

- [Guía Completa con ejemplos desde Starknet-ES]

---

## Starks
### Visión general del proceso Stark
![Graph](/im%C3%A1genes/Starks.png)

Nos interesa la Integridad Computacional (CI), por ejemplo, saber que el programa de Cairo que escribiste se calculó correctamente.

Tenemos que pasar por una serie de transformaciones desde el trazado de nuestro programa, hasta la prueba.

La primera parte de esto se llama aritmetización, y consiste en tomar nuestra traza y convertirla en un conjunto de polinomios.

Nuestro problema se convierte entonces en uno en el que el prover intenta convencer a un verificador de que el polinomio es de grado bajo.

El verificador está convencido de que el polinomio es de grado bajo si y sólo si el cálculo original es correcto (salvo una probabilidad infinitesimalmente pequeña).

### Uso de la aletoriedad
El prover utiliza la aleatoriedad para alcanzar el conocimiento cero, el verificador utiliza la aleatoriedad al generar consultas al prover, para detectar trampas por parte del prover.

### Concisón y rendimiento
Gran parte del trabajo que se realiza al crear una prueba consiste en garantizar que sea sucinta y que pueda elaborarse y verificarse en un tiempo razonable.

Por tanto, nuestro plan consiste en

1. Reformular la traza de ejecución como un polinomio,
2. Extenderlo a un gran dominio,
3. Transformarlo, utilizando las restricciones polinómicas, en otro polinomio que se garantiza que es de grado bajo si y sólo si la traza de ejecución es válida.

Queremos lograr una verificación sucinta, en la que el verificador de la declaración CI requiera exponencialmente menos recursos que los necesarios para la repetición ingenua.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Aritmetización
Hay dos pasos

1. Generación de una traza de ejecución y restricciones polinómicas
2. Transformar estos dos objetos en un único polinomio de bajo grado.


En términos de interacción prover-verificador, lo que realmente ocurre es que el prover y el verificador acuerdan de antemano cuáles son las restricciones polinómicas.

A continuación, el prover genera una traza de ejecución y, en la interacción posterior, intenta convencer al verificador de que las restricciones polinómicas se cumplen en esta traza de ejecución, sin que el verificador lo vea.

La traza de ejecución es una tabla que representa los pasos del cálculo subyacente, donde cada fila representa un único paso.

El tipo de traza de ejecución que buscamos generar debe tener la característica especial de ser sucintamente comprobable - "cada fila puede ser verificada basándose sólo en las filas que están cerca de ella en la traza, y el mismo procedimiento de verificación se aplica a cada par de filas".

Por ejemplo, imaginemos que nuestra traza representa un total en ejecución, con cada paso de la siguiente manera.

```bash
╔════════╦═══════════╦═══════╗
║  PASO  ║  IMPORTE  ║ TOTAL ║
╠════════╬═══════════╬═══════╣
║   0    ║     0     ║   0   ║    
╠════════╬═══════════╬═══════╣
║   1    ║     5     ║   5   ║
╠════════╬═══════════╬═══════╣
║   2    ║     2     ║   7   ║
╠════════╬═══════════╬═══════╣
║   3    ║     2     ║   9   ║
╠════════╬═══════════╬═══════╣
║   4    ║     3     ║   12  ║
╠════════╬═══════════╬═══════╣
║   5    ║     6     ║   18  ║
╚════════╩═══════════╩═══════╝
```

Si representamos la fila como `i` , y la columna como `j` , y los valores como `Aᵢ,ⱼ` Podríamos escribir algunas restricciones sobre esto de la siguiente manera

`A₀,₂=0`

`∀1 >= i <= 5 : Aᵢ,₂ − Aᵢ,₁ − Aᵢ-₁,₂ = 0`

`A₅,₂ = 18`

Se trata de restricciones polinómicas lineales en `Aᵢ,ⱼ`

Nótese que aquí estamos consiguiendo cierta concisión porque podríamos representar un número mucho mayor de filas con sólo estas 3 restricciones.

El sistema de restricciones aritméticas define al menos dos tipos de restricciones sobre la traza de ejecución algebraica:

* Restricciones de contorno: al principio o al final del cálculo, un registro indicado tiene un valor determinado.
* Restricciones de transición: dos tuplas de estado consecutivas cualesquiera evolucionan de acuerdo con la función de transición de estado.

En conjunto, estas restricciones se conocen como representación algebraica intermedia o AIR.

Las STARKs avanzadas pueden definir más tipos de restricciones para tratar con la memoria o con la consistencia de los registros dentro de un ciclo.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Crear un polinomio para nuestra traza
También en este caso definimos un polinomio `f(x)` tal que los elementos de la traza de ejecución son evaluaciones de `f` en potencias de algún generador `g`.

Recordemos que nuestro campo finito tendrá generadores, que utilizaremos para indexar los pasos de nuestra traza. Tomando el ejemplo de fibonacci del [artículo de medium](https://medium.com/starkware/arithmetization-ii-403c3b3f4355) podemos crear restricciones como:

`∀ x ∈ {1,g²,g³...g⁵⁰⁹}: f(g²x) ₋ f(gx) ₋ f(x) = 0`

Esto restringe los valores entre las filas subsiguientes.
También significa que los valores g son raíces de este polinomio.

Por lo tanto, podemos utilizar el enfoque que vimos anteriormente para proporcionar el polinomio de fuga utilizando el término `(x - gⁱ)` y a partir de él creamos el polinomio de composición.

![Graph](/im%C3%A1genes/compocisionpo.png)

El hecho básico sobre polinomios y sus raíces es que si `p(x)` es un polinomio, entonces `p(a)=0` para algún valor específico `a`, si y sólo si existe un polinomio `q(x)` tal que `(x-a)q(x)=p(x)`, y `deg(p)=deg(q)+1`.

Esta expresión coincide con el polinomio de grado 2 como máximo si nuestra traza de ejecución ha sido correcta, es decir, ha obedecido a la restricción de paso que hemos definido.

Si la traza difiere de eso, entonces es poco probable que esta expresión produzca un polinomio de bajo grado.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Polinomio de Composición
Para demostrar eficazmente la validez del rastro de ejecución, nos esforzamos por alcanzar los dos objetivos siguientes:

1. Componer las restricciones sobre los polinomios de la traza para hacerlas cumplir en la traza.
2. Combinar las restricciones en un único polinomio (más grande), denominado `Polinomio de Composición`, de modo que se pueda utilizar una única prueba de grado bajo para atestiguar su grado bajo.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Ampliando nuestro polinomio
Como hemos visto antes, los polinomios pueden utilizarse para construir buenos códigos de corrección de errores, ya que dos polinomios de grado `d`, evaluados en un dominio considerablemente mayor que `d`, son diferentes en casi todas partes.

Observando esto, podemos extender la traza de ejecución pensando en ella como una evaluación de un polinomio en algún dominio, y evaluando este mismo polinomio en un dominio mucho mayor. Extendiendo de manera similar una traza de ejecución incorrecta, se obtiene una cadena muy diferente, lo que a su vez hace posible que el verificador distinga entre estos casos utilizando un pequeño número de consultas.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### De las restricciones polinómicas al problema de las pruebas de bajo grado

En general, si nuestro cálculo implica `N` pasos, la traza de ejecución estará representada por polinomios de grado inferior a `N`

`f(X) = c₀ + c₁X + c₂X² +⋯+ cɴ-₁Xᴺ⁻¹`

"Los coeficientes `cᵢ` están en el campo `F` y el límite `N` en el grado es típicamente grande, quizá del orden de unos pocos millones. A pesar de ello, estos polinomios se denominan de bajo grado.

Esto se debe a que el punto de comparación es el tamaño del campo.
Por interpolación, toda función sobre `𝔽` puede representarse mediante un polinomio.

La mayoría de ellos tendrán un grado igual al tamaño total del campo, por lo que, comparado con éste, `N` es realmente bajo.

Este tipo de funciones, coherentes con un polinomio de bajo grado, también se conocen como códigos `Reed-Solomon`.

Tras la generación de la traza, el prover se compromete con ella. Recordemos que no queremos enviar los polinomios al verificador como un todo, pero necesitamos que el prover se comprometa con ellos.

En todo el sistema, los compromisos se ejecutan construyendo árboles de Merkle sobre las series de elementos de campo y enviando las raíces de Merkle al verificador.

Queremos que un verificador plantee al prover un número muy reducido de preguntas y decida si acepta o rechaza la prueba con un alto nivel de precisión garantizado.
Idealmente, al verificador le gustaría pedir al prover que proporcione los valores en unos pocos lugares (aleatorios) en la traza de ejecución, y comprobar que las restricciones polinómicas se mantienen para estos lugares.

Una traza de ejecución correcta pasará naturalmente esta prueba.

Sin embargo, no es difícil construir una traza de ejecución completamente errónea (especialmente si sabíamos de antemano qué puntos se comprobarían), que viole las restricciones sólo en un punto de la traza único y, al hacerlo, llegar a un resultado completamente alejado y diferente. 
Identificar este fallo mediante un pequeño número de consultas aleatorias es altamente improbable.

Pero recuerda que los polinomios tienen algunas propiedades útiles aquí

* Dos polinomios (diferentes) de grado `d` evaluados en un dominio considerablemente mayor que `d` son diferentes en casi todas partes.

Así que si tenemos un prover deshonesto, que crea un polinomio de bajo grado representando su traza (que es incorrecta en algún punto) y lo evalúa en un dominio grande, será fácil ver que este es diferente al polinomio correcto.

En [`estas`](https://www.sikoba.com/docs/zklux1_slides_dmitry.pdf) diapositivas se ofrece un buen ejemplo de este proceso

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Pruebas de bajo grado
Las pruebas de bajo grado son realmente el corazón del proceso de verificación.

#### En General
El supuesto de comprobación de bajo grado establece la existencia de un verificador probabilístico que comprueba si una función `f` es de grado como máximo `d ≪ |𝔽|`
.
El verificador debe distinguir entre los dos casos siguientes.

* La función `f` es igual a un polinomio de bajo grado.
    * Es decir, existe un polinomio `p(x)` sobre `𝔽`, de grado menor que `d`, que coincide con `f` en todas partes.
* La función `f` está lejos de TODOS los polinomios de bajo grado.
    * Por ejemplo, necesitamos modificar al menos el 10% de los valores de `f` antes de obtener una función que concuerde con un polinomio de grado inferior a `d`.

La aritmetización muestra que un prover honesto que trate con una afirmación verdadera caerá en el primer caso, mientras que un prover (posiblemente malicioso) que intente "probar" una afirmación falsa caerá, con alta probabilidad, en el segundo caso.

Otra forma de ver esto es que el polinomio de traza correcto combinado con las restricciones será necesariamente de grado bajo, el grado proviene del número de pasos en nuestra traza (probablemente unos pocos millones), y la combinación de esto con los polinomios de restricción (probablemente < 10).

En general, cabría esperar que los polinomios "correctos" tuvieran un grado de alrededor de `10⁷` , mientras que un prover tramposo que eligiera puntos al azar del campo `𝔽` obtendría, tras la interpolación, polinomios de grado comparable al tamaño del campo, es decir, del orden de `2²⁵⁶`

- [Guía Completa con ejemplos desde Starknet-ES]

---

## FRI
FRI son las siglas de `Fast Reed-Solomon IOP of Proximity`, es un protocolo que establece que un polinomio comprometido tiene un grado limitado.

El FRI es complejo y gran parte del procesamiento que lo compone está diseñado para que las pruebas sean factibles y sucintas.
También hay mucho procesamiento involucrado con la protección contra diversos tipos de ataques que podrían ser realizados por el prover, y garantizar que todo se lleva a cabo en el conocimiento cero.

Su objetivo es encontrar si un conjunto de puntos se encuentran mayoritariamente en un polinomio de bajo grado y puede alcanzar una complejidad de prueba lineal y una complejidad de verificación logarítmica.

En general, hay 2 etapas : commit y query, contenidas en los siguientes pasos repetidos.

1. El verificador envía un número aleatorio al prover
2. El prover genera un nuevo polinomio
3. El verificador genera los conjuntos puntuales de consultas y los envía al prover
4. El prover evalúa los valores polinómicos correspondientes
5. El verificador realiza una comprobación de validez.

En este [artículo](https://aszepieniec.github.io/stark-anatomy/) se explica con más detalle.

"FRI es un protocolo entre un probador y un verificador, que establece que una codeword dada pertenece a un polinomio de grado bajo.

El prover conoce explícitamente este codeword, mientras que el verificador sólo conoce su raíz Merkle y las hojas de su elección, suponiendo la validación satisfactoria de las rutas de autenticación que establecen la pertenencia de las hojas al "árbol Merkle".

"Una de las grandes ideas para los sistemas de pruebas de los últimos años ha sido la técnica de dividir y doblar. La idea es reducir una afirmación a dos afirmaciones de la mitad de tamaño. A continuación, ambas afirmaciones se fusionan en una sola utilizando pesos aleatorios proporcionados por el verificador.

Después de muchos pasos, la afirmación se ha reducido a una de tamaño trivial que es verdadera si y sólo si (modulo alguna degradación de seguridad insignificante) la afirmación original era verdadera."

El verificador inspecciona los árboles de Merkle (en concreto: pide al prover que proporcione las hojas indicadas con sus rutas de autenticación) de rondas consecutivas para comprobar una relación lineal simple. 

Para los provers honestos, el grado de los polinomios representados también se reduce a la mitad en cada ronda y, por tanto, es mucho menor que la longitud de la codeword.

Sin embargo, para los provers maliciosos, este grado es uno menos que la longitud de la codeword. En el último paso, el prover envía una codeword no trivial correspondiente a un polinomio constante.

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Heurística Fiat-Shamir
Véase [https://aszepieniec.github.io/stark-anatomy/basic-tools](https://aszepieniec.github.io/stark-anatomy/basic-tools)

Este es un proceso mediante el cual podemos hacer que una prueba interactiva no sea interactiva.

Funciona proporcionando commitments (compromisos) a los mensajes que formarían la interacción. Las funciones hash se utilizan como fuente de aleatoriedad.

![Graph](/im%C3%A1genes/Shamir.png)

- [Guía Completa con ejemplos desde Starknet-ES]

---

### Cairo y el no determinismo
Nos interesa la integridad computacional y, como hemos visto, todos los pasos de un cálculo pueden representarse como polinomios.
Esta forma se denomina representación algebraica intermedia (AIR).

Los bloques de cálculo representados como una AIR pueden combinarse entre sí, lo que constituye la base de Cairo.

Por utilizar una analogía de hardware

* ASIC (AIR)
* CPU (varios AIR)

El nombre Cairo proviene de: una CPU construida a partir de AIRs (CPU-AIR, Oh nice -> CAIRO).

CAIRO es un lenguaje funcional de alto nivel, no determinista y completo en turing. 
Tiene un modelo de memoria basado en registros y un compilador. El compilador produce una tabla de pasos computacionales llamada traza.

El prover utiliza la traza para construir AIRs que se combinan y se convierten en una prueba STARK.

En los programas de Cairo, se escribe qué resultados son aceptables, no cómo obtenerlos.

```cairo
func main{}() {
    alloc_locals;
    local x;

    assert x + 3 = 10;
    return ();
}
```

Esto es esperar que el prover proporcione un valor para x. 

Podemos añadir una pista de la siguiente manera

```cairo
func main{}() {
    alloc_locals;
    local x;

    %{
    ids.x = 4
    %}

    assert x + 3 = 10;
    return ();
}
```

Así que esto fallaría pero si producimos una pista aceptable.

```cairo
    %{
    ids.x = 7
    %}
```

`Entonces nuestro código tendrá éxito`

- [Guía Completa con ejemplos desde Starknet-ES]

---

## Referencias y lecturas complementarias
- Estos [artículos de medium](https://medium.com/starkware/stark-math-the-journey-begins-51bd2b063c71) de Starkware te llevan a través de las matemáticas a un nivel relativamente alto.
- Esta [serie de artículos](https://aszepieniec.github.io/stark-anatomy/) proporciona muchos detalles y la implementación python del proceso
STARK.
- La [serie de artículos](https://vitalik.ca/general/2017/11/09/starks_part_1.html) de Vitalik Buterin explican el proceso STARK

- [White Paper STARK](https://eprint.iacr.org/2018/046.pdf)
- [White Paper Cairo](https://eprint.iacr.org/2021/1063)

- La serie de vídeos [STARK@Home](https://www.youtube.com/watch?v=9J9rhKJk4RM&list=PLcIyXLwiPilUFGw7r2uyWerOkbx4GFMXq) en Youtube es excelente y ofrece varios niveles de detalle sobre STARKS.
