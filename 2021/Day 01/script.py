import os

filepath = os.path.dirname(__file__)

def part1():
    file = open(os.path.join(filepath, "input.txt"))
    data = file.read().splitlines()
    prev = -1
    counter = 1
    for d in data:
        if prev == -1:
            prev = d
        else:
            if prev < d:
                counter += 1
            prev = d
    print("counter", counter)
        
print("One")
part1()