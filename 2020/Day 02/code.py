def two():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)

    lineWithError = 0

    for line in data:
        line = line.rstrip()
        passwd = config = line.split(':')[1].lstrip()
        config = line.split(':')[0]
        minMax = config.split()[0]
        letter = config.split()[1][0]
        minNum = int(minMax.split('-')[0]) - 1
        maxNum = int(minMax.split('-')[1]) - 1

        count = 0
        # print(passwd, minNum, maxNum, letter)
        # print(minNum, maxNum, len(passwd), passwd)
        if minNum >= len(passwd) and maxNum >= len(passwd):
            lineWithError += 1
        elif minNum >= len(passwd) and maxNum < len(passwd) and passwd[maxNum] != letter:
            if passwd[maxNum] != letter:
                lineWithError += 1
        elif maxNum >= len(passwd) and minNum < len(passwd):
            if passwd[minNum] != letter:
                lineWithError += 1
        elif passwd[minNum] == letter and passwd[maxNum] == letter:
            lineWithError += 1
        elif passwd[minNum] != letter and passwd[maxNum] != letter:
            lineWithError += 1
    print(len(data) - lineWithError)

def one():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)

    lineWithError = 0

    for line in data:
        line = line.rstrip()
        passwd = config = line.split(':')[1].lstrip()
        config = line.split(':')[0]
        minMax = config.split()[0]
        letter = config.split()[1][0]
        minNum = int(minMax.split('-')[0])
        maxNum = int(minMax.split('-')[1])

        count = 0
        for l in passwd:
            if l == letter:
                count += 1
        if count < minNum or count > maxNum:
            lineWithError += 1
    print(len(data) - lineWithError)

print("one")
one()
print("two")
two()