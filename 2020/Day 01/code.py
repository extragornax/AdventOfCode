def main():
    file = open("input.txt")
    data = []
    for i in file:
        data.append(i)
    for l1 in data:
        one = int(l1)
        for l2 in data:
            two = int(l2)
            if one + two == 2020:
                print(one * two)
                return


main()