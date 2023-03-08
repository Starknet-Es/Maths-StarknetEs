<div align="center">
  <h1 style="font-size: larger;">
    <strong>Comienza el viaje</strong> 
    </h1>
    <img src="https://github.com/dimeyad/Matematicas-Stark/blob/main/im%C3%A1genes/1.1.png" width="600">
</div>

## Temas

- [Introducción](#introducción)
- [Confianza frente a verificación](#confianza-frente-a-verificación)
    - [El Viejo Mundo: Confianza o responsabilidad delegada](#el-viejo-mundo-confianza-o-responsabilidad-delegada)
    - [El Nuevo mundo: Verificar, o rendición de cuentas inclusiva](#el-nuevo-mundo-verificar-o-rendición-de-cuentas-inclusiva)
- [Sistemas de prueba](#sistemas-de-prueba)
- [STARK](#stark)
    - [Escalabilidad: Aceleración exponencial de la verificación](#escalabilidad-aceleración-exponencial-de-la-verificación) 
    - [Transparencia: con confianza hacia nadie, con integridad para todos](#transparencia-con-confianza-hacia-nadie-con-integridad-para-todos)  
    - [Lean Cryptography: Seguro y rápido](#lean-cryptography-seguro-y-rápido)
- [Resumen](#resumen)

## Parte 1
La misión de StarkWare es llevar STARKs al mundo real. Este es el primero de una serie de posts que explican la teoría detrás de STARKs y nuestra aplicación de la misma. Empezaremos de forma ligera e iremos avanzando en los siguientes posts.

## Introducción
La integridad computacional ("CI", por sus siglas en inglés) es una propiedad fundamental en la que se basa el comercio. En términos sencillos, significa que el resultado de un determinado cálculo es correcto. La CI es lo que nos permite confiar en el saldo de una cuenta que nos presentan o en la factura de una tienda. Este artículo se adentrará en cómo las blockchains sin permisos logran la CI sin requerir confianza, el precio dramático que pagan por ello en términos de escalabilidad y privacidad, y cómo las STARK pueden salvar el día.

## Confianza frente a verificación
### El "Viejo Mundo": Confianza o responsabilidad delegada
Los sistemas financieros (bancos, corredores, bolsas, etc.) necesitan operar con integridad para cumplir sus funciones sociales. ¿Qué mecanismos les incentivan a operar con integridad? El _"viejo mundo"_ asume la confianza como sustituto de la integridad. Confiamos en que los bancos, los fondos de pensiones, etc., actúen con honradez. Vayamos a la madriguera del conejo y examinemos la base de esta confianza, ignorando el "teatro de la integridad" -los edificios altos y los trajes elegantes- preparado para impresionarnos. Desde una perspectiva puramente racional y utilitaria, lo que impide que el sistema financiero se apodere de todos nuestros fondos es la amenaza de la desgracia social, la cárcel y las multas. También hay una zanahoria: la reputación, que atrae a futuros clientes y genera futuros beneficios. Al firmar con su nombre los estados financieros, la gente del "viejo mundo" se juega su libertad personal y sus finanzas actuales y futuras como garantía de integridad, y nosotros, el público, basamos nuestra confianza en este acuerdo. La verificación de esta integridad se delega en expertos como contables, auditores y reguladores. A esto lo llamaremos rendición de cuentas delegada. No es un mal sistema: ha servido fielmente a las economías modernas durante bastante tiempo.

Una nueva variante del enfoque del "viejo mundo" es el Entorno de Ejecución de Confianza (TEE). Un fabricante de hardware de confianza (como Intel) produce una máquina física (como el chip SGX) que no puede desviarse del cálculo especificado y firma los estados correctos utilizando una clave secreta que sólo conoce esa máquina física. La integridad se basa ahora en la confianza en el hardware y su fabricante y en la suposición de que es imposible extraer claves secretas de tales dispositivos físicos.

### El "Nuevo Mundo": Verificar, o rendición de cuentas inclusiva
Las cadenas de bloques ofrecen una forma más directa de alcanzar la integridad, plasmada en el lema "No confíes, verifica". Este "nuevo mundo" no requiere un teatro de la integridad, no depende de contables, ni sus desarrolladores y mantenedores de red se juegan su libertad personal para ganarse la confianza del público. La integridad está garantizada por la Rendición de Cuentas Inclusiva: un nodo con una configuración computacional estándar (un ordenador portátil conectado a Internet) debería ser capaz de verificar la integridad de todas las transacciones del sistema.

El método predominante para verificar la CI en blockchains sin permisos es la repetición ingenua: se pide a todos los nodos que vuelvan a ejecutar (repetición) los cálculos que verifican todas y cada una de las transacciones. La rendición de cuentas inclusiva, en esta forma ingenua, conlleva dos retos inmediatos:

* **Privacidad:** Si todo el mundo puede inspeccionar todas las transacciones, la privacidad puede verse comprometida. La ausencia de privacidad disuade a las empresas, ya que significa que la información sensible puede no seguir siendo confidencial. También disuade a los individuos, ya que erosiona la [dignidad humana](https://en.wikipedia.org/wiki/The_Right_to_Privacy_(article)).
* **Escalabilidad:** Exigir que el sistema sea compatible con un ordenador portátil estándar significa que no puede ampliarse simplemente con ordenadores más grandes y un mayor ancho de banda. Esto limita mucho el rendimiento del sistema.

Los sistemas de pruebas (de los que hablaremos a continuación) son una excelente solución a ambos problemas. Los sistemas de pruebas Zero Knowledge (ZK) son ya una herramienta establecida para abordar la privacidad en blockchains y se explican de forma excelente en varios posts de Zcash [[1](https://z.cash/blog/shielded-ecosystem/),[2](https://z.cash/technology/),[3](https://z.cash/technology/zksnarks/)]. Aunque los ZK-STARKs ofrecen Conocimiento Cero, este post no hablará de ello sino que se centrará en la Escalabilidad y la Transparencia (la S y la T de STARK).

## Sistemas de prueba
Los sistemas de prueba comenzaron con la introducción del modelo de [prueba interactiva](https://en.wikipedia.org/wiki/Interactive_proof_system) (IP) por Goldwasser, Micali y Rackoff en 1985. Las pruebas interactivas son protocolos en los que intervienen dos tipos de entidades: un probador y un verificador, que interactúan a lo largo de varias rondas mediante el envío de mensajes. El prover y el verificador tienen objetivos opuestos: el prover quiere convencer al verificador de la integridad de un determinado cálculo, y el verificador es un guardián sospechoso al que el público ha confiado la tarea de distinguir entre verdades y falsedades. El demostrador y el verificador se comunican de forma interactiva, enviándose mensajes por turnos. Estos mensajes dependen de la afirmación que se está probando, de los mensajes anteriores y también pueden utilizar cierta aleatoriedad. Por parte del verificador, la aleatoriedad es necesaria para generar consultas al verificador. Al final del proceso interactivo, el verificador toma una decisión: acepta el nuevo estado o lo rechaza.

Una buena analogía es el proceso de examen que se practica en un tribunal cuando una parte presenta una reclamación y su contraparte cuestiona su validez. Para que la demanda sea aceptada como cierta, las respuestas del demandante (prover) a las preguntas del examinador (verifier) deben ser coherentes y válidas. Se espera que el proceso de examen ponga de manifiesto cualquier desajuste entre una afirmación y la realidad y, por tanto, la exponga como falsa.

Decimos que un sistema de pruebas resuelve la CI si al actualizar el sistema del estado A al estado B, se cumplen las siguientes propiedades:

* Completitud: Si el probador sabe cómo cambiar el estado de A a B de forma válida, entonces conseguirá convencer al verificador de que acepte el cambio.
* Solidez: Si el prover no sabe cómo cambiar el estado de A a B, el verificador detectará una incoherencia en la interacción y rechazará la transición de estado sugerida. Queda una pequeña probabilidad de falso positivo, es decir, una probabilidad de que el verificador acepte una prueba no válida. Esta probabilidad es un parámetro de seguridad del sistema que puede fijarse a un nivel aceptable como 1/(2¹²⁸), una probabilidad similar a ganar la lotería Powerball cinco veces seguidas.

Este par de propiedades tiene una implicación crucial para el principio de Responsabilidad Inclusiva discutido anteriormente. El verificador puede aceptar la transición de estado sugerida por el prover sin hacer ninguna suposición sobre la integridad del prover. De hecho, el prover puede ejecutarse en un hardware defectuoso, puede ser de código cerrado y puede ejecutarse en un ordenador controlado por una entidad maliciosa. Lo único que importa¹ es que los mensajes enviados por el prover lleven al verificador a aceptar la declaración. Si es así, sabemos que la integridad computacional se mantiene.

## STARK
A estas alturas existen bastantes construcciones teóricas de sistemas de prueba, junto con implementaciones. Algunos están implementados en criptomonedas, como los [SNARKs](https://z.cash/technology/zksnarks/) utilizados por [Zerocash](http://zerocash-project.org/paper)/[Zcash](https://z.cash/), y los [Bulletproofs](https://eprint.iacr.org/2017/1066) (BP) implementados en [Monero](https://ww.getmonero.org/)(Para obtener información general sobre los sistemas de prueba vaya [aquí](https://zkp.science/)). Lo que distingue a los [STARK](https://eprint.iacr.org/2018/046) es la combinación de las tres propiedades siguientes: escalabilidad (la S de STARK), transparencia (la T de STARK) y criptografía simplificada.

### Escalabilidad: Aceleración exponencial de la verificación
Escalabilidad significa que se cumplen simultáneamente dos propiedades de eficiencia:

* **Prover escalable:** El tiempo de ejecución del comprobador es "casi lineal" al tiempo que tardaría un ordenador de confianza en comprobar la CI simplemente volviendo a ejecutar el cálculo y comprobando que el resultado coincide con lo que alguien afirma. La proporción de "sobrecarga" (tiempo necesario para generar una prueba/tiempo necesario para ejecutar el cálculo) sigue siendo razonablemente baja.
* **Verificador escalable:** El tiempo de ejecución del verificador es polinómico en el logaritmo del tiempo de reproducción ingenua. En otras palabras, el tiempo de ejecución del verificador es exponencialmente menor que la simple reproducción del cálculo (recuérdese que la "replay" es el método actual de blockchain para lograr la Rendición de Cuentas Inclusiva).

<div align="center">
<img src="https://github.com/dimeyad/Matematicas-Stark/blob/main/im%C3%A1genes/1.2.png" width="700">
</div>

Aplique esta noción de escalabilidad a una cadena de bloques. En lugar del modo actual de verificación por repetición ingenua, imagine cómo serán las cosas cuando una cadena de bloques pase a la verificación mediante el uso de sistemas de pruebas. En lugar de limitarse a enviar las transacciones que se añadirán a la cadena de bloques, un nodo verificador tendrá que generar una prueba, pero gracias al verificador escalable su tiempo de ejecución es casi lineal al tiempo de ejecución de la solución de reproducción ingenua. Y el verificador escalable se beneficiará de una reducción exponencial de su tiempo de verificación. Además, a medida que aumente el rendimiento de la cadena de bloques, la mayor parte del efecto recaerá sobre los nodos prover (que podrían funcionar con hardware dedicado, como los mineros), mientras que los verificadores, que constituirían la mayoría de los nodos de la red, apenas se verían afectados.

Consideremos un ejemplo hipotético concreto, suponiendo que el tiempo del verificador (en milisegundos) escala como el cuadrado del logaritmo del número de transacciones (tx). Supongamos que empezamos con 10.000 tx/bloque. Entonces el tiempo de ejecución del verificador es:

* VTime = (log₂ 10.000)² ~ (13,2)² ~ 177 ms

Aumente ahora el tamaño del bloque cien veces (a 1.000.000 tx/bloque). El nuevo tiempo de ejecución del verificador es:

* VTime = (log₂ 1.000.000)² ~ 20² ~ 400 ms

En otras palabras, ¡un aumento de 100 veces en el rendimiento de las transacciones sólo supuso un aumento de 2,25 veces en el tiempo de ejecución del verificador!

En algunos casos, el verificador aún tendrá que descargar y verificar la disponibilidad de los datos, lo cual es un proceso de tiempo lineal, pero la descarga de datos suele ser mucho más barata y rápida que la comprobación de su validez.

### Transparencia: con confianza hacia nadie, con integridad para todos
Transparencia significa² que no hay configuración de confianza - no hay uso de secretos en la configuración del sistema. La transparencia ofrece muchas ventajas. Elimina el procedimiento de generación de parámetros, que constituye un único punto de fallo. La falta de una configuración de confianza permite incluso a entidades poderosas -grandes corporaciones, monopolios y gobiernos, que controlan el sistema financiero del "viejo mundo"- probar la CI y obtener la aceptación pública de sus afirmaciones porque no hay forma conocida de falsificar las pruebas de falsedad de STARK, incluso por parte de las entidades más poderosas. En un plano más táctico, facilita enormemente el despliegue de nuevas herramientas e infraestructuras y la modificación de las existentes sin necesidad de elaboradas ceremonias de generación de parámetros. Y lo que es más importante, la transparencia se alinea bien con el "nuevo mundo" que exige una rendición de cuentas inclusiva bajo ningún supuesto de confianza. [Parafraseando a Abraham Lincoln](https://en.wikipedia.org/wiki/Abraham_Lincoln%27s_second_inaugural_address), los sistemas transparentes permiten operar sin confianza hacia nadie, con integridad para todos.

### Lean Cryptography: Seguro y rápido
La seguridad de STARK se basa en unos supuestos criptográficos mínimos: la existencia de funciones hash criptográficas seguras y [resistentes a las colisiones](https://en.wikipedia.org/wiki/Collision_resistance). Muchas de estas primitivas existen hoy en día como instrucciones de hardware, y la criptografía magra conduce a dos beneficios más:

* **Seguridad poscuántica:** Los STARK son plausiblemente seguros frente a ordenadores cuánticos eficientes.
* **Eficiencia concreta:** Para un cálculo dado, el prover STARK es al menos 10 veces más rápido que el prover SNARK y el prover Bulletproofs. El verificador STARK es al menos 2 veces más rápido que el verificador SNARK y más de 10 veces más rápido que el verificador Bulletproof. A medida que StarkWare continúe optimizando STARKs estos ratios probablemente mejorarán. Sin embargo, la longitud de una prueba STARK es ~100x mayor que la correspondiente SNARK y ~20x mayor que BulletProofs.

## Resumen
Empezamos explicando el "nuevo mundo" de las cadenas de bloques sin permisos, en el que el lema es "No confíes, verifica". El principio de Rendición de Cuentas Inclusiva exige que la integridad del sistema financiero pueda ser verificada fácilmente por cualquiera, en contraposición a la Rendición de Cuentas Delegada del "viejo mundo". Para que las cadenas de bloques sean escalables, necesitamos métodos que permitan verificar la integridad computacional con mayor rapidez que la repetición ingenua.

Los STARK son un tipo especial de sistema de prueba que ofrece escalabilidad y transparencia, y se basan en la criptografía simplificada. Su escalabilidad y transparencia permiten una verificación barata y fiable de la IC. Esta es la promesa de STARK y la misión de StarkWare.

Nuestro próximo post profundizará un poco más en las matemáticas de la construcción de STARKs.

---

**Traduciones realizadas por el equipo oficial de Starknet-Es, gracias a [Dimeyad](https://github.com/dimeyad) y [Nadai](https://github.com/Nadai2010) por estos aportes**.

Gracias a Vitalik Buterin y Arthur Breitman por revisar los borradores de este artículo.

Michael Riabzev y Eli Ben-Sasson
Industrias StarkWare

---

* ¹La preservación de la privacidad (ZK) exige que el código del prover garantice que los mensajes del prover no filtren información sobre el testigo a través de canales secundarios. Pero la solidez no requiere suposiciones de confianza.

* ²Formalmente, un sistema de prueba transparente es aquel en el que todos los mensajes del verificador son cadenas aleatorias públicas. Estos sistemas también se conocen como protocolos [Arthur-Merlin](https://en.wikipedia.org/wiki/Arthur%E2%80%93Merlin_protocol)

* ³Esta minimización de los supuestos criptográficos es válida para los STARK interactivos (iSTARK). Los STARKs no interactivos (nSTARKs) requieren la heurística Fiat-Shamir que es una bestia diferente.
