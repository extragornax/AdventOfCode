def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    size = len(data)
    index = 0
    acc = 0
    visitedIndex = []
    while index < size:
        if index in visitedIndex:
            break
        visitedIndex.append(index)
        if data[index].split()[0] == "nop":
            index += 1
            continue
        elif data[index].split()[0] == "acc":
            d = data[index].split()[1]
            if d[0] == '+':
                acc += int(d[1:])
            elif d[0] == '-':
                acc -= int(d[1:])
            index += 1
            continue
        elif data[index].split()[0] == "jmp":
            d = data[index].split()[1]
            if d[0] == '+':
                index += int(d[1:])
            elif d[0] == '-':
                index -= int(d[1:])
            continue
        index += 1
    print(acc)

def editNextLine(data, index):
    while index < len(data):
        if data[index].split()[0] == "nop":
            data[index] = data[index].replace("nop", "jmp")
            return index + 1, data
        elif data[index].split()[0] == "jmp":
            data[index] = data[index].replace("jmp", "nop")
            return index + 1, data
        index += 1
    return index + 1, data

def two():
    file = open("input.txt")
    primeData = []
    for i in file:
        primeData.append(i.rstrip().lstrip())
    size = len(primeData)
    isFinish = False
    acc = 0
    editIndex = -1
    while isFinish == False:
        data = primeData.copy()
        editIndex, data = editNextLine(data, editIndex)
        index = 0
        acc = 0
        visitedIndex = []
        wasBroken = False
        while index < size:
            if index in visitedIndex:
                wasBroken = True
                break
            visitedIndex.append(index)
            if data[index].split()[0] == "nop":
                index += 1
                continue
            elif data[index].split()[0] == "acc":
                d = data[index].split()[1]
                if d[0] == '+':
                    acc += int(d[1:])
                elif d[0] == '-':
                    acc -= int(d[1:])
                index += 1
                continue
            elif data[index].split()[0] == "jmp":
                d = data[index].split()[1]
                if d[0] == '+':
                    index += int(d[1:])
                elif d[0] == '-':
                    index -= int(d[1:])
                continue
            index += 1
        if wasBroken == False:
            isFinish = True
    print(acc)


print("one")
one()
print("two")
two()