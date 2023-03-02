def mod_add(a, b, m):
    # Retorna la suma (a + b) % m
    return (a + b) % m

def mod_subtract(a, b, m):
    # Retorna la resta (a - b) % m
    return (a - b) % m

def mod_multiply(a, b, m):
    # Retorna el producto (a * b) % m
    return (a * b) % m

def mod_divide(a, b, m):
    # Retorna la división (a / b) % m
    # Nota: Esto solo es válido si m es primo y b es coprimo con m
    # En otras palabras, b no tiene factores en común con m
    b_inv = pow(b, m-2, m)
    return (a * b_inv) % m

# Suma modular de 15 y 7 modulo 10
result = mod_add(15, 7, 10)
print(result)  # Output: 2

# Resta modular de 15 y 7 modulo 10
result = mod_subtract(15, 7, 10)
print(result)  # Output: 8

# Producto modular de 15 y 7 modulo 10
result = mod_multiply(15, 7, 10)
print(result)  # Output: 5

# División modular de 15 y 7 modulo 10
result = mod_divide(15, 7, 10)
print(result)  # Output: 5
