import os

current_x = 0  # left right
current_y = 0  # up down


def up(map, quantity):
    global current_x
    global current_y
    while quantity > 0:
        current_y += 1
        map[current_y][current_x] += 1
        quantity -= 1


def down(map, quantity):
    global current_x
    global current_y
    while quantity > 0:
        current_y -= 1
        map[current_y][current_x] += 1
        quantity -= 1


def left(map, quantity):
    global current_x
    global current_y
    while quantity > 0:
        current_x -= 1
        map[current_y][current_x] += 1
        quantity -= 1


def right(map, quantity):
    global current_x
    global current_y
    while quantity > 0:
        quantity -= 1
        current_x += 1
        map[current_y][current_x] += 1


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

def main():
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
    for line in data:
        parsed = line.strip().split(',')
        setData(map, originX, originY, parsed)
    print("Done")
    print("Searching for intersections")
    allInter = searchCloseInter(map, maxX, maxY)
    closest = findClosest(allInter, originX, originY)
    print("SHORTEST", closest)
    print("Done")

if __name__ == "__main__":
    main()