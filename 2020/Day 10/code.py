def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(int(i.rstrip().lstrip()))
    data = sorted(data)
    oneInc = 0
    threeInc = 1
    for index in range(len(data)):
        if index == 0:
            if data[index] == 1:
                oneInc += 1
            elif data[index] == 3:
                threeInc += 1
            continue
        if data[index] - data[index - 1] > 3:
            return
        elif (data[index] - data[index - 1]) == 1:
            oneInc += 1
        elif (data[index] - data[index - 1]) == 3:
            threeInc += 1
    print(oneInc * threeInc)

print("one")
one()
print("two")
# two()