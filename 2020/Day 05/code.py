def getRowNumber(s):
    mi = 0
    ma = 127
    for i in s:
        if i == 'F':
            ma -= int((ma - mi) / 2) + 1
        elif i == 'B':
            mi = mi + int((ma - mi) / 2) + 1
    return mi

def getSeatCol(s):
    mi = 0
    ma = 7
    for i in s:
        if i == 'L':
            ma -= int((ma - mi) / 2) + 1
        elif i == 'R':
            mi = mi + int((ma - mi) / 2) + 1
    return mi

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    maxId = 0
    for i in data:
        row = getRowNumber(i[0:7])
        seat = getSeatCol(i[7:])
        seatId = row * 8 + seat
        if seatId > maxId:
            maxId = seatId
    print(maxId)

def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    allId = []
    for i in data:
        row = getRowNumber(i[0:7])
        seat = getSeatCol(i[7:])
        seatId = row * 8 + seat
        allId.append(seatId)
    sortedId = sorted(allId)
    previous = sortedId[0] - 1
    for i in sortedId:
        if previous + 1 != i:
            print(i - 1)
        previous = i

print("one")
one()
print("two")
two()