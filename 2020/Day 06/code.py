def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    total = 0
    grData = {}
    for i in data:
        i = i.rstrip()
        i = i.lstrip()
        if len(i) < 1:
            total += len(grData)
            grData = {}
            continue
        for l in i:
            grData[l] = True
    total += len(grData)
    grData = {}
    print(total)

def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    total = 0
    grData = {}
    counter = 0
    for i in data:
        i = i.rstrip()
        i = i.lstrip()
        if len(i) < 1:
            for d in grData:
                if grData[d] == counter:
                    total += 1
            grData = {}
            counter = 0
            continue
        for l in i:
            if l in grData:
                grData[l] += 1
            else:
                grData[l] = 1
        counter += 1
    for d in grData:
        if grData[d] == counter:
            total += 1
    grData = {}
    counter = 0
    print(total)

print("one")
one()
print("two")
two()