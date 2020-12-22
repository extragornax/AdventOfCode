import re

def checkHex(s):
    for ch in s:
        if ((ch < '0' or ch > '9') and
            (ch < 'a' or ch > 'f')):             
            return False
    return True

def two():
    file = open("input.txt")
    data = []
    tmpVal = {}
    for i in file:
        i = i.rstrip()
        if len(i) == 0:
            if len(tmpVal) > 0:
                data.append(tmpVal)
                tmpVal = {}
            continue
        # print(i)
        spl = i.split()
        for item in spl:
            key = item.split(':')[0]
            val = item.split(':')[1]
            tmpVal[key] = val

    if len(tmpVal) > 0:
        data.append(tmpVal)
        tmpVal = {}
    goodPassword = 0
    regex = re.compile("^(?P<numbers>\d*)(?P<letters>\w*)$")
    for i in data:
        if "byr" in i and len(i['byr']) == 4 and i['byr'].isnumeric() and int(i['byr']) >= 1920 and int(i['byr']) <= 2002:
            if "iyr" in i and len(i['iyr']) == 4 and i['iyr'].isnumeric() and int(i['iyr']) >= 2010 and int(i['iyr']) <= 2020:
                if "eyr" in i and len(i['eyr']) == 4 and i['eyr'].isnumeric() and int(i['eyr']) >= 2020 and int(i['eyr']) <= 2030:
                    if "hgt" in i:
                        (height, cmOrIn) = regex.search(i['hgt']).groups()
                        if cmOrIn == "cm":
                            if int(height) < 150 or int(height) > 193:
                                continue
                        elif cmOrIn == "in":
                            if int(height) < 59 or int(height) > 76:
                                continue
                        else:
                            continue
                        if "hcl" in i and len(i['hcl']) == 7 and i['hcl'][0] == '#' and checkHex(i['hcl'][1:]):                            
                            if "ecl" in i:
                                test = i['ecl']
                                if test != "amb" and test != "blu" and test != "brn" and test != "gry" and test != "grn" and test != "hzl" and test != "oth":
                                    continue 
                                if "pid" in i:
                                    if i['pid'].isnumeric() == False or len(i['pid']) != 9:
                                        continue
                                    goodPassword += 1
    print(goodPassword)
    
def one():
    file = open("input.txt")
    data = []
    tmpVal = {}
    for i in file:
        i = i.rstrip()
        if len(i) == 0:
            if len(tmpVal) > 0:
                data.append(tmpVal)
                tmpVal = {}
            continue
        # print(i)
        spl = i.split()
        for item in spl:
            key = item.split(':')[0]
            val = item.split(':')[1]
            tmpVal[key] = val

    if len(tmpVal) > 0:
        data.append(tmpVal)
        tmpVal = {}
    goodPassword = 0
    for i in data:
        if "byr" in i:
            if "iyr" in i:
                if "eyr" in i:
                    if "hgt" in i:
                        if "hcl" in i:
                            if "ecl" in i:
                                if "pid" in i:
                                    goodPassword += 1
    print(goodPassword)

print("one")
one()
print("two")
two()