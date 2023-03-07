import random

'''
Haremos una prueba de que conocemos el valor de x en la expresión (b ** x) % p = y,
donde b = 499, p = 509 y y = 440;
es decir, la expresión es esta: (499 ** x) % 509 = 440.

El valor de x es 383.
'''

b = 499
p = 509
y = 440
x = (int(input("Coloca el valor de x: ")))
N = p - 1

yn = (b ** x) % p

if yn != y:

    print("El valor introducido no es la solución")

else:

    print("Efectivamente, (b ** x) % p =", y)

    # Paso número 1 (Elegir un número e < N y enviar (bp = b ^ e) al verificador:

    lmt = 2
    e = random.randrange(lmt, N)

    if x == e:

        print("Número e no es válido, por favor intente de nuevo") # Hay que evitar que e tenga el mismo valor que x

    else:

        bp = (b ** e) % p
        print("bp =", bp)

        # Paso número 2 (Lanzar la moneda):

        if e % 2 == 0: # Simulamos el lanzamiento de la moneda determinando si e es par o no.

            print("La moneda ha salido cara, entonces:")
            print("El valor de e es:", e)
            print("Puedes comprobar que b ** e = bp =", bp)

        else:

            print("La moneda ha salido cruz, entonces:")
            power = (x + e) % N
            print("(x + e) % N =", power)
            b_xe = (b ** power) % p
            ybp = (y * bp) % p

            if ybp == b_xe:
                print("La prueba ha sido exitosa")
