def part1():
    data = open("data.txt")
    total = 0
    for line in data:
        total += int(int(line) / 3) - 2
    print(total)

def part2():
    data = open("data.txt")
    total = 0
    for line in data:
        fuel = int(int(line) / 3) - 2
        totalFuel = fuel
        while fuel > 0:
            fuel = int(int(fuel) / 3) - 2
            if fuel > 0:
                totalFuel += fuel
        
        total += totalFuel
    print(total)
