def checkSurround(data, x, y):
    count = 0
    if x > 0:
        if y > 0:
            if data[x - 1][y - 1] == '#':
                count += 1
        if data[x - 1][y] == '#':
            count += 1
        if y + 1 < len(data[x]):
            if data[x - 1][y + 1] == '#':
                count += 1
    if y > 0 and data[x][y - 1] == '#':
        count += 1
    if y + 1 < len(data[x]) and data[x][y + 1] == '#':
        count += 1
    if x + 1 < len(data):
        if y > 0:
            if data[x + 1][y - 1] == '#':
                count += 1
        if data[x + 1][y] == '#':
                count += 1
        if y + 1 < len(data[x]):
            if data[x + 1][y + 1] == '#':
                count += 1
    return count

def iterInDirection(data, x, y, incX, incY):
    # print(x, y)
    try:
        while data[x][y]:
            x += incX
            y += incY
            if x < 0 or y < 0:
                return 0
            if x > len(data) or y > len(data[x]):
                return 0
            if data[x][y] == '#':
                # print("found", x, y)
                return 1
            elif data[x][y] == 'L':
                return 0
    except IndexError:
        return 0
    return 0

def checkSurroundV2(data, x, y):
    count = 0
    up = -1
    down = 1
    stay = 0
    right = 1
    left = -1
    count += iterInDirection(data, x, y, up, left)
    count += iterInDirection(data, x, y, up, stay)
    count += iterInDirection(data, x, y, up, right)
    count += iterInDirection(data, x, y, stay, right)
    count += iterInDirection(data, x, y, down, right)
    count += iterInDirection(data, x, y, down, stay)
    count += iterInDirection(data, x, y, down, left)
    count += iterInDirection(data, x, y, stay, left)
    # print(x, y, count)
    return count


def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    numOfChange = 1
    while numOfChange != 0:
        currentMap = data.copy()
        numOfChange = 0
        for x in range(len(data)):
            currentMap[x] = list(currentMap[x])
            for y in range(len(data[x])):
                if currentMap[x][y] == '.':
                    continue
                ct = checkSurround(data, x, y)
                if ct == 0 and data[x][y] == 'L':
                    currentMap[x][y] = '#'
                    numOfChange += 1
                elif ct >= 4 and data[x][y] == '#':
                    currentMap[x][y] = 'L'
                    numOfChange += 1
        data = currentMap
    count = 0
    for x in range(len(data)):
        for y in range(len(data[x])):
            if data[x][y] == '#':
                count += 1
    print(count)

def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    numOfChange = 1
    loopct = 0
    while numOfChange != 0 and loopct < 3:
        # loopct += 1
        currentMap = data.copy()
        numOfChange = 0
        # for o in currentMap:
        #     print("".join(o))
        # print()
        for x in range(len(data)):
            currentMap[x] = list(currentMap[x])
            for y in range(len(data[x])):
                if currentMap[x][y] == '.':
                    continue
                ct = checkSurroundV2(data, x, y)
                if ct == 0 and data[x][y] == 'L':
                    currentMap[x][y] = '#'
                    numOfChange += 1
                elif ct >= 5 and data[x][y] == '#':
                    currentMap[x][y] = 'L'
                    numOfChange += 1
        data = currentMap
    count = 0
    for x in range(len(data)):
        for y in range(len(data[x])):
            if data[x][y] == '#':
                count += 1
    print(count)

print("one")
one()
print("two")
two()
