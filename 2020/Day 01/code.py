def part2():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    for l1 in data:
        one = int(l1)
        for l2 in data:
            two = int(l2)
            for l3 in data:
                three = int(l3)
                if one + two + three == 2020:
                    print(one * two * three)
                    return

def part1():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    for l1 in data:
        one = int(l1)
        for l2 in data:
            two = int(l2)
            if one + two == 2020:
                print(one * two)
                return


print("One")
part1()
print("Two")
part2()