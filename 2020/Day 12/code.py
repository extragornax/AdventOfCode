north = 1
west = 2
south = 3
east = 4

def retDirection(currentDir, dire, amount):
    while amount > 0:
        if (dire == 'L'):
            currentDir += 1
        if (dire == 'R'):
            currentDir -= 1
        if currentDir == 0:
            currentDir = 4
        elif currentDir == 5:
            currentDir = 1
        amount -= 1
    return currentDir

def getDirection(dire):
    if dire == north:
        return 'north'
    elif dire == west:
        return 'west'
    elif dire == south:
        return 'south'
    elif dire == east:
        return 'east'
    else:
        return 'unknown'

def printPosition(boat):
    x = boat['x']

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    direction = east
    boat = {
        'x': 0, # left - right
        'y': 0  # up - down
    }
    for l in data:
        letter = l[0]
        amount = int(l[1:])
        if letter == 'N':
            boat['y'] += amount
        elif letter == 'S':
            boat['y'] -= amount
        elif letter == 'E':
            boat['x'] -= amount
        elif letter == 'W':
            boat['x'] += amount
        elif letter == 'L':
            turn = amount / 90
            direction = retDirection(direction, letter, turn)
        elif letter == 'R':
            turn = amount / 90
            direction = retDirection(direction, letter, turn)
        elif letter == 'F':
            if direction == north:
                boat['y'] += amount
            elif direction == south:
                boat['y'] -= amount
            elif direction == east:
                boat['x'] -= amount
            elif direction == west:
                boat['x'] += amount
        else:
            print("Unknown letter", letter)
        print(letter, amount, boat, getDirection(direction))
    # print('x', boat['x'], 'y', boat['y'], abs(boat['x']) + abs(boat['y']))
        
def two():
    file = open("test.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    direction = east
    boat = {
        'x': 0, # left - right
        'y': 0  # up - down
    }
    waypoint = {
        'x': -10, # left - right | east-west
        'y': 1    # up - down    | north-south
    }
    for l in data:
        letter = l[0]
        amount = int(l[1:])
        if letter == 'N':
            waypoint['y'] += amount
        elif letter == 'S':
            waypoint['y'] -= amount
        elif letter == 'E':
            waypoint['x'] -= amount
        elif letter == 'W':
            waypoint['x'] += amount
        elif letter == 'R':
            turn = amount / 90
            direction = retDirection(direction, letter, turn)
            for i in range(int(turn)):
                bkpX = waypoint['x']
                bkpY = waypoint['y']
                if bkpX < 0 and bkpY >= 0:
                    bkpY = -bkpY
                elif bkpX < 0 and bkpY < 0:
                    bkpX = - bkpX
                elif bkpX >= 0 and bkpY < 0:
                    bkpY = -bkpY
                elif bkpX >= 0 and bkpY >= 0:
                    bkpX = -bkpX
                waypoint['y'] = bkpX
                waypoint['x'] = bkpY
        elif letter == 'L':
            turn = amount / 90
            direction = retDirection(direction, letter, turn)
            for i in range(int(turn)):
                bkpX = waypoint['x']
                bkpY = waypoint['y']
                if bkpX < 0 and bkpY >= 0:
                    bkpX = -bkpX
                elif bkpX < 0 and bkpY < 0:
                    bkpY = -bkpY
                elif bkpX >= 0 and bkpY < 0:
                    bkpX = -bkpX
                elif bkpX >= 0 and bkpY >= 0:
                    bkpY = -bkpY
                waypoint['y'] = bkpX
                waypoint['x'] = bkpY
        elif letter == 'F':
            boat['x'] += waypoint['x'] * amount
            boat['y'] += waypoint['y'] * amount
        else:
            print("Unknown letter", letter)
        print(letter, amount, waypoint, boat, getDirection(direction))
    print('x', boat['x'], 'y', boat['y'], abs(boat['x']) + abs(boat['y']))
    # print(abs(boat['x']) + abs(boat['y']))

# print("one")
# one()
print("two")
two()