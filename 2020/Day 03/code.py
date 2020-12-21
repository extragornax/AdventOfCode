def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    slopes = [
        {'r': 1, 'd': 1},
        {'r': 3, 'd': 1},
        {'r': 5, 'd': 1},
        {'r': 7, 'd': 1},
        {'r': 1, 'd': 2}
    ]
    trees = 1
    for s in slopes:
        tree = 0
        line = 0
        sidePos = 0
        for i in data:
            if (line % s['d'] != 0):
                line += 1
                continue
            if i[sidePos % (len(i) - 1) ] == '#':
                tree += 1
            sidePos += s['r']
            line += 1
        if tree != 0:
            trees = trees * tree
    print(trees)
    

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    sidePos = 0
    tree = 0
    for i in data:
        if i[sidePos % (len(i) - 1) ] == '#':
            tree += 1
        sidePos += 3
    print(tree)

print("one")
one()
print("two")
two()