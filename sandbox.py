def make_counter():
    i = 0
    def counter():
        i += 1
        print(i)
    
    return counter

counter = make_counter()
counter()
counter()