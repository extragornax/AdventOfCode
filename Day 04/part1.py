# 6 digits number
# range 145852-616942
# 2 adjacent numbers are the same (like 22 in 122345)
# left to right digits never decrease (> or ==)

totalValid = 0

def isValid(num):
    global totalValid
    if num < 145852 or num > 616942:
        return

    num = str(num)
    isSame = 0
    for idx, val in enumerate(num):
        if idx > 0:
            if val == num[idx - 1]:
                isSame += 1
            if val < num[idx - 1]:
                return
    if isSame == 0:
        return
    totalValid += 1
    # print(num)

min = 145889 # 145852
max = 616942

for a in range(1, 7):
    print(a)
    for b in range(10):
        for c in range(10):
            for d in range(10):
                for e in range(10):
                    for f in range(10):
                        num = a * 100000 + \
                                b * 10000 + \
                                c * 1000 + \
                                d * 100 + \
                                e * 10 + \
                                f
                        isValid(num)
                        # print(num)

print("Total:", totalValid)