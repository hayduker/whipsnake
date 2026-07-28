class MetaDemo:
    print("Executing class body right now!")
    
    a = 5
    b = a * 2

    while True:
        x = 11
        break

    def call_me(self):
        return self.x

m = MetaDemo()
print(MetaDemo.b + m.call_me())