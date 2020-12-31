from os import system
import os


def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    depart = int(data[0])
    allBus = data[1].split(',')
    best = {
        'bus': -1,
        'dist': -1
    }
    for bus in allBus:
        if bus != 'x':
            busNum = int(bus)
            b = int(bus)
            while b <= depart:
                b += busNum
            dist = b - depart
            if best['dist'] == -1 or best['dist'] > dist:
                best['bus'] = busNum
                best['dist'] = dist
    print(best['dist'] * best['bus'])

def getFirstOccur(target, increment):
    t = 0
    while t <= target:
        t += increment
    if t - increment == target:
        return True
    if t == target:
        return True
    else:
        # print("bus", increment, "getFirstOccur", t, end=" - ")
        return False

def cls():
    os.system('cls' if os.name=='nt' else 'clear')

def testPrint(msg):
    cls()
    print(msg)

def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i.rstrip().lstrip())
    depart = int(data[0])
    allBus = data[1].split(',')
    busLst = []
    for idx, bus in enumerate(allBus):
        if bus != 'x':
            a = {
                'bus': int(bus),
                'index': int(idx)
            }
            busLst.append(a)
    notDone = True
    t = 100023092700000
    timeInc = busLst[0]['bus']
    busLst = busLst[1:]
    while notDone == True:
        notDone = False
        t += timeInc
        if t % 1000000 == 0:
            testPrint(str(t))
        # print('t', t)
        for i in busLst:
            num = (t + i['index']) /  i['bus']
            if  num != int(num):
                # print("wanted ", t + i['index'])
                notDone = True
                break
        # print()
    print(t)
    # print(busLst)
            



print("one")
one()
print("two")
two()