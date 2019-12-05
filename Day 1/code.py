data = open("data.txt")
total = 0
lineCounter = 0
for line in data:
    total += int(int(line) / 3) - 2
    print("One line", lineCounter, "got", line, int(int(line) / 3) - 2)
    lineCounter +=1
print(total)