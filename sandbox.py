class Person:
    def say_name(self):
        return self.name

bill = Person()
bill.name = "Bill"
say_bill = bill.say_name

jane = Person()
jane.name = "Jane"
say_jane = jane.say_name

bill.name = jane.name
assert say_bill() == "Jane"

