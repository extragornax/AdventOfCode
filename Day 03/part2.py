import os

current_x = 0  # left right
current_y = 0  # up down
step_Number = 0
allInterSteps = []


def up(map, quantity):
    global current_x
    global current_y
    global step_Number
    global allInterSteps
    while quantity > 0:
        current_y += 1
        map[current_y][current_x] += 1
        quantity -= 1
        step_Number += 1
        if map[current_y][current_x] > 1:
            allInterSteps.append({'steps':step_Number, 'x':current_x, 'y':current_y})


def down(map, quantity):
    global current_x
    global current_y
    global step_Number
    global allInterSteps
    while quantity > 0:
        current_y -= 1
        map[current_y][current_x] += 1
        quantity -= 1
        step_Number += 1
        if map[current_y][current_x] > 1:
            allInterSteps.append({'steps':step_Number, 'x':current_x, 'y':current_y})


def left(map, quantity):
    global current_x
    global current_y
    global step_Number
    global allInterSteps
    while quantity > 0:
        current_x -= 1
        map[current_y][current_x] += 1
        quantity -= 1
        step_Number += 1
        if map[current_y][current_x] > 1:
            allInterSteps.append({'steps':step_Number, 'x':current_x, 'y':current_y})


def right(map, quantity):
    global current_x
    global current_y
    global step_Number
    global allInterSteps
    while quantity > 0:
        quantity -= 1
        current_x += 1
        map[current_y][current_x] += 1
        step_Number += 1
        if map[current_y][current_x] > 1:
            allInterSteps.append({'steps':step_Number, 'x':current_x, 'y':current_y})


def setData(map, startX, startY, lst):
    global current_x
    global current_y
    current_x = startX
    current_y = startY
    for item in lst:
        switch = {
            'U': up,
            'D': down,
            'L': left,
            'R': right
        }
        # print(item[0])
        func = switch.get(item[0], lambda: "Invalid month")
        func(map, int(item[1:]))


def searchCloseInter(map, sizeX, sizeY):
    inter = []
    for y in range(sizeY - 1):
        for x in range(sizeX - 1):
            if map[y][x] > 1:
                inter.append({'y':y, 'x':x})
    return inter

def findClosest(lst, originX, originY):
    shortest = None
    for item in lst:
        len = abs(item['y'] - originY) + abs(item['x'] - originX)
        # print(originX, originY, item, abs(item['y'] - originY), abs(item['x'] - originX))
        if shortest == None or shortest > len:
            shortest = len
    return shortest

def findShortestSteps(allInterLists):
    shortest = None
    one = allInterLists[0]
    two = allInterLists[1]
    for item in one:
        for find in two:
            if item['y'] == find['y'] and item['x'] == find['x']:
                total = item['steps'] + find['steps']
                if shortest == None or shortest > total:
                    shortest = total
    return shortest

def main():
    global allInterSteps
    map = []
    maxY = 15000
    maxX = 15000
    originY = int(maxY/2)
    originX = int(maxX/2)
    print("Generate map")
    for _ in range(maxY):
        tmp = []
        for _ in range(maxX):
            tmp.append(0)
        map.append(tmp)
    print("Done")
    print("Starting to put data")
    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'data.txt')
    # filename = os.path.join(dirname, 'test.txt')
    data = open(filename)
    allLines = []
    allInterLists = []
    recuNum = 0
    for line in data:
        step_Number = 0
        parsed = line.strip().split(',')
        setData(map, originX, originY, parsed)
        allLines.append(parsed)
        if recuNum != 1:
            allInterLists.append(allInterSteps.copy())
        recuNum += 1
        allInterSteps = []
    print("Done")
    print("Searching for intersections")
    allInter = searchCloseInter(map, maxX, maxY)
    closest = findClosest(allInter, originX, originY)
    print("SHORTEST", closest)
    print("Done")
    print("find shortest number of steps")

    stepsC = findShortestSteps(allInterLists)
    print("SHORTEST", stepsC)
    print("done")

if __name__ == "__main__":
    main()