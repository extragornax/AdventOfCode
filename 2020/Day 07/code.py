def getParentColor(arr, s):
    allColor = []
    for line in arr:
        for son in line['sons']:
            if son == s:
                allColor.append(line['color'])
    return allColor

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    allPairs = []
    for i in data:
        split = i.split()
        color = split[0] + ' ' + split[1]
        content = split[4:]
        numOfItems = int(len(content) / 4)
        sons = []
        pair = {}
        for l in range(numOfItems):
            if content[l * 4] == "no":
                break
            childcolor = content[l * 4 + 1] + ' ' + content[l * 4 + 2]
            sons.append(childcolor)
        pair['color'] = color
        pair['sons'] = sons
        allPairs.append(pair)
    # print(allPairs)
    allParents = getParentColor(allPairs, "shiny gold")
    previous = allParents
    while True:
        thisRun = []
        store = []
        for i in previous:
            thisRun = getParentColor(allPairs, i)
            for t in thisRun:
                allParents.append(t)
                store.append(t)
        if len(store) == 0:
            break
        previous = store
    allParents = sorted(allParents)
    allParents = list(dict.fromkeys(allParents))
    print(len(allParents))

def retColItem(data, s):
    for i in data:
        if i['color'] == s:
            return i

def iterColor(data, s):
    dat = retColItem(data, s)
    total = 1
    for i in dat['sons']:
        total += iterColor(data, i['color']) * i['count']
    return total

def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    allPairs = []
    for i in data:
        split = i.split()
        color = split[0] + ' ' + split[1]
        content = split[4:]
        numOfItems = int(len(content) / 4)
        sons = []
        pair = {}
        for l in range(numOfItems):
            childcolor = content[l * 4 + 1] + ' ' + content[l * 4 + 2]
            child = {
                'color': childcolor,
                'count': int(content[l * 4])
            }
            sons.append(child)
        pair['color'] = color
        pair['sons'] = sons
        allPairs.append(pair)
    print(iterColor(allPairs, "shiny gold") - 1)

print("one")
one()
print("two")
two()