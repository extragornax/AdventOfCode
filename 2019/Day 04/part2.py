# 6 digits number
# range 145852-616942
# 2 adjacent numbers are the same (like 22 in 122345)
# left to right digits never decrease (> or ==)

import re

totalValid = 0

def matchesContainDoubleDigit(matches):
    for match in matches:
        if(len(match[0]) == 2):
            return True
    
    return False

def isValid(number):
    global totalValid
    
    if number < 145852 or number > 616942:
        return
    
    numberAsString = str(number)
    ascending = re.match("^0*1*2*3*4*5*6*7*8*9*$", numberAsString)
    repeating = re.findall(r'((\d)\2+)', numberAsString)
    if bool(ascending and matchesContainDoubleDigit(repeating)):
        totalValid += 1
        # print(number)

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