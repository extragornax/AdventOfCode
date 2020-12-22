def checkIfWorks(target, lst):
    for a in lst:
        for b in lst:
            if a == b:
                continue
            if a + b == target:
                return True
    return False

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(int(i.rstrip().lstrip()))
    lstCompare = data[:25]
    lstCheck = data[25:]
    while checkIfWorks(lstCheck[0], lstCompare):
       lstCompare.pop(0)
       lstCompare.append(lstCheck.pop(0))
    print(lstCheck[0])
    return lstCheck[0]

def two(target):
    file = open("input.txt")
    data = []
    for i in file:
        data.append(int(i.rstrip().lstrip()))
    index = 0
    baseIndex = 1
    count = 0
    while index < len(data):
        count += data[index]
        if count == target:
            d = sorted(data[baseIndex- 1:index + 1])
            print(d[0] + d[-1])
            break
        elif count > target:
            index = baseIndex
            baseIndex += 1
            count = 0
            continue
        index += 1

print("one")
v = one()
print("two")
two(v)
