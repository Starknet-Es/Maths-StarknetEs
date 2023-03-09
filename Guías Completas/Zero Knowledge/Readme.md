## Zero knowledge
Zero knowledge o la prueba de conocimiento cero, es un concepto interesante que probablemente se volverá cada vez más importante en el ecosistema criptográfico en los próximos años. En términos simples, es una forma de demostrar algo sin revelar información sobre ello. Por ejemplo, si alguien afirma conocer la combinación de una caja fuerte, puede demostrarlo a alguien sin revelar la combinación real.

![](https://github.com/Nadai2010/Clases-Maths/blob/master/im%C3%A1genes/zkProof.gif)

Aquí hay algunos ejemplos:

- **Ejemplo 1:**
  Imagina que un chico llamado Miguel dice que sabe cómo abrir cualquier caja fuerte del mundo, pero no quiere revelar su método. Entonces, diferentes    personas de distintos países le ofrecen cajas fuertes para verificar su afirmación, y Miguel las abre todas sin revelar su método. El número de cajas fuertes que abra determinará el nivel de confianza que la gente tenga en su afirmación.

- **Ejemplo 2:**
  Supongamos que una persona quiere demostrar que conoce la combinación de una caja fuerte sin revelar la combinación real. En este caso, la persona puede pedir a alguien que verifique que conoce la combinación abriendo la caja fuerte en su presencia. Entonces, la persona puede taparse los ojos y dejar que el verificador ingrese la combinación para abrir la caja fuerte.

En criptografía, se utilizan pruebas de conocimiento cero para proteger la información. Aunque la forma en que se implementan en la criptografía es fascinante y compleja, la idea básica es que una persona puede demostrar algo sin revelar información sobre ello.

Tuve una experiencia interesante con un hilo en el que mostré a mis padres, quienes dijeron lo siguiente:

- ✅ "El concepto se entiende perfectamente".
- ✅ "Hay algo que queda oculto", dijo mi madre.
- ✅ "Te permite probar algo sin revelarlo todo", dijo mi padre.

Sin embargo, aún no está claro cómo se puede aplicar este tipo de prueba en el mundo digital y, por ende, en el ecosistema de crypto.

Hoy vamos a aclarar este punto 

Recordemos los ejemplos anteriores:

* **En el primer ejemplo:** mi amigo afirmó que conocía la clave de la caja fuerte y la abrió sin revelármela.
* **En el segundo ejemplo:** Miguel afirmó que podía abrir cualquier caja fuerte del mundo y las abrió todas sin revelar cómo lo hizo.

Podemos realizar otro tipo de prueba de manera similar a los ejemplos anteriores. Podemos afirmar que conocemos un número para el cual se cumple una expresión matemática.

Aquí hay un ejemplo muy simple 

Puedo decirles a mis padres: `"Acabo de pensar en un número X que si le sumo 2, el resultado es igual a 3"`. La declaración matemática es muy básica: `x + 2 = 3`. Conociendo cómo son mis padres, seguramente me responderán: "Obviamente, nosotros también sabemos la respuesta" 

* Como muchos sabrán, `1 + 2 = 3`. Por lo tanto, el valor de `x es 1`.

Pero, en crypto, lo que buscamos es probar que sabemos el número SIN revelarlo. En el caso anterior, no es que lo haya revelado, pero la expresión matemática era muy básica y ellos pudieron encontrar el número sin inconvenientes. Es por eso que en criptografía generalmente se trabaja con expresiones matemáticas un poco más complejas, como estas: `3^x ≡ 6 (mod 7)`, donde `x` puede ser cualquier dato que no queramos revelar, es el valor que se "esconde" pero que deseamos probar que conocemos.

## ¿cómo se implementan las "zero knowledge proofs"? 
Se prueba que conocemos el valor que tiene la incógnita, en este caso x, en alguna expresión matemática pero sin revelarla. Es importante entender cómo funcionan las "relaciones de congruencia" y son muy utilizadas en criptografía. Podemos establecer expresiones como esta: `a ≡ b (mod n)`, donde `a` y `b` son números y `n` es un entero positivo. Esto significa que `a` y `b` tienen el mismo resto cuando se dividen por `n`.

## ¿Cómo podemos demostrar que conocemos el valor de "x" en alguna expresión sin revelarla? 
Imagínate que conozco un número x para el cual se cumple esta expresión matemática:

```bash
3 ^ x ≡ 6 (mod 7)
```

Y quiero demostrarte que sé cuál es el valor de `x` sin revelártelo directamente. En este caso, yo seré el "probador" y tú serás el "verificador", ¿está bien? 

Entonces, ¿qué podemos hacer para demostrar que conozco el valor de x sin revelarlo? Bueno, en primer lugar, podemos sustituir los números de la expresión matemática por variables (letras), de esta manera:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-7.jpeg" width="500">
</div>

* Los pasos para realizar la prueba serían los siguientes:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-8.jpeg" width="450">
</div>

* Ahora vamos a ir explicando todo poco a poco:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-9.jpeg" width="400">
</div>

¡Muy bien!

Siguiendo con los pasos anteriormente establecidos, ahora llega el momento de "lanzar una moneda" (más adelante explicaré el motivo). Supongamos que al lanzar la moneda sale cruz:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-10.jpeg" width="400">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-11.jpeg" width="400">
</div>

**¡Excelente!. Pero... ¿por qué lanzamos la moneda?**

Esta es una pregunta muy interesante. Si no conozco el valor de `x`, entonces cuando tenga que enviarte `b'`, podría mentirte y darte una respuesta incorrecta. Por ejemplo, podría enviarte lo siguiente:

<div align="center">
<img src="https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Im%C3%A1genes/1-12.jpeg" width="500">
</div>

Pero si la moneda sale cara, estaré en problemas, ya que no conozco la potencia de `b` que da como resultado `b'`. Por esa razón se propone "lanzar la moneda". Como ya se dijo, se recomienda repetir el procedimiento hasta que el **"verificador"** esté convencido de que el **"probador"** realmente conoce `x`.

Hay algo que no se ha mencionado:

En el ejemplo ofrecido se utilizan números relativamente pequeños en comparación con los que se utilizan en los protocolos criptográficos de la actualidad. Esto se hizo así para facilitar la explicación.

Como el procedimiento puede ser un poco tedioso de comprobar, hemos elaborado código en Python, Rust y Cairo 1.0 para que puedas experimentar con este tipo de pruebas. En este código se establece una "zero knowledge proof" de que se conoce el valor de x en la siguiente expresión matemática:

```bash
499 ^ x ≡ 440 (mod 509)
```
Te enseñamos como hacerlo en nuestras guías:

- [Python](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgePY.md)
- [Rust](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeRS.md)
- [Cairo](https://github.com/Starknet-Es/Maths-StarknetEs/blob/main/Gu%C3%ADas%20Completas/Zero%20Knowledge/Contracts/Zero_KnowledgeCAIRO.md)

Además de eso, aquí abajo te dejamos algunos enlaces de interés:

- [Link del Índice de la Guía Completa Maths](https://github.com/Starknet-Es/Maths-StarknetEs/tree/main/Gu%C3%ADas%20Completas)
- [Link de la explicación de 0xhasher_ en Twitter](https://twitter.com/0xhasher_/status/1590844232599732224)


