import os

filepath = os.path.dirname(__file__)

def part2():
    file = open(os.path.join(filepath, "input.txt"))
    data = file.read().splitlines()
    one = -1
    two = -1
    curr = -1
    counter = 1
    for d in data:
        if one == -1:
            one = int(d)
        elif two == -1:
            two = one
            one = int(d)
        else:
            print("curr va", one, two, d)
            if curr == -1:
                curr = one + two + int(d)
            else:
                print("curr:", curr, "next:", one + two + int(d))
                if curr < one + two + int(d):
                    counter += 1
                curr = one + two + int(d)
                one = two
                two = int(d)
    print("Total:", counter)
            
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
    print("Total:", counter)
        
print("One")
part1()
print("Two")
part2()