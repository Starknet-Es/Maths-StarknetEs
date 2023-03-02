def method_1(a,b):

    for i in range(1,509):
        result = (a ** i) % b
        print("Result", i,":", result)

print("Method 1:")
print("")
method_1(499,509)

print("")

def method_2(a,b):

    for i in range(1,509):
        result = (a ** i) % b
        if result == 466:
            print("Result is:",i)

print("Method 2:")
print("")
method_2(499,509)
