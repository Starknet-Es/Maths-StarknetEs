# Operaciones básicas en los números naturales

# Suma
def suma(a, b):
  return a + b

# Elemento neutro para la suma
elemento_neutro_suma = 0

# Resta
def resta(a, b):
  return a - b

# Elemento neutro para la resta
elemento_neutro_resta = 0

# Multiplicación
def multiplicacion(a, b):
  return a * b

# Elemento neutro para la multiplicación
elemento_neutro_multiplicacion = 1

# División
def division(a, b):
  return a / b

# Comprobación de la propiedad asociativa de la suma
def asociativa_suma(a, b, c):
  return (a + b) + c == a + (b + c)

# Comprobación de la propiedad conmutativa de la suma
def conmutativa_suma(a, b):
  return a + b == b + a

# Comprobación de la propiedad distributiva de la multiplicación sobre la suma
def distributiva_multiplicacion_sobre_suma(a, b, c):
  return a * (b + c) == a * b + a * c

# Ejemplo de uso:
resultado_suma = suma(2, 7)
print(resultado_suma) # Imprime 9

resultado_resta = resta(4, 6)
print(resultado_resta) # Imprime -2

resultado_multiplicacion = multiplicacion(7, 1)
print(resultado_multiplicacion) # Imprime 7

resultado_division = division(7, 2)
print(resultado_division) # Imprime 3.5

es_asociativa_suma = asociativa_suma(2, 7, 3)
print(es_asociativa_suma) # Imprime True

es_conmutativa_suma = conmutativa_suma(2, 7)
print(es_conmutativa_suma) # Imprime False

es_distributiva_multiplicacion_sobre_suma = distributiva_multiplicacion_sobre_suma(2, 3, 4)
print(es_distributiva_multiplicacion_sobre_suma) # Imprime True
