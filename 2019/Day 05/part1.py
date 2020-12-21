POSITION_MODE = 0
IMMEDIATE_MODE = 1

def add(one, two):
    return one + two

def mutl(one, two):
    return one * two

def twoParamDest(dic):
    global POSITION_MODE
    global IMMEDIATE_MODE
    ret = {}
    if dic['first'] == POSITION_MODE:
        ret['one'] = int(dic['data'][dic['arrPos'] + 1])
    else:
        ret['one'] = dic['arrPos'] + 1
    if dic['second'] == POSITION_MODE:
        ret['two'] = int(dic['data'][dic['arrPos'] + 2])
    else:
        ret['two'] = dic['arrPos'] + 2
    if dic['third'] == POSITION_MODE:
        ret['destination'] = int(dic['data'][dic['arrPos'] + 3])
    else:
        ret['destination'] = dic['arrPos'] + 3
    return ret

def codeOne(pa):
    par = twoParamDest(pa)
    pa['data'][par['destination']] = add(pa['data'][par['one']], pa['data'][par['two']])
    return pa['arrPos'] + 4

def codeTwo(pa):
    par = twoParamDest(pa)
    pa['data'][par['destination']] = mutl(pa['data'][par['one']], pa['data'][par['two']])
    return pa['arrPos'] + 4

def codeThree(pa):
    if pa['first'] == POSITION_MODE:
        destination = int(pa['data'][pa['arrPos'] + 1])
    else:
        destination = pa['arrPos'] + 1
    pa['data'][destination] = int(input("Input > "))
    return pa['arrPos'] + 2

def codeFour(pa):
    if pa['first'] == POSITION_MODE:
        pos = int(pa['data'][pa['arrPos'] + 1])
    else:
        pos = pa['arrPos'] + 1
    print(pa['data'][pos])
    return pa['arrPos'] + 2

def main():
    global POSITION_MODE
    global IMMEDIATE_MODE
    bkpData = [3,225,1,225,6,6,1100,1,238,225,104,0,1102,46,47,225,2,122,130,224,101,-1998,224,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,1102,61,51,225,102,32,92,224,101,-800,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,61,64,225,1001,118,25,224,101,-106,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1102,33,25,225,1102,73,67,224,101,-4891,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,14,81,225,1102,17,74,225,1102,52,67,225,1101,94,27,225,101,71,39,224,101,-132,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1002,14,38,224,101,-1786,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,65,126,224,1001,224,-128,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,81,40,224,1001,224,-121,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,344,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,374,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1006,224,479,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,494,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,524,1001,223,1,223,7,226,226,224,102,2,223,223,1005,224,539,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,554,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,584,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,599,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,659,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226]
    # bkpData = [1003,1,1004,1,99]
    data = bkpData
    arrPos = 0
    while data[arrPos] != 99:
        posVal = str(data[arrPos])[::-1]
        opcode = int((posVal[0:2])[::-1])
        firstParamStatus = POSITION_MODE
        secondParamStatus = POSITION_MODE
        thirdParamStatus = POSITION_MODE
        if len(posVal) >= 3:
            firstParamStatus = int(posVal[2])
        if len(posVal) >= 4:
            secondParamStatus = int(posVal[3])
        if len(posVal) >= 5:
            thirdParamStatus = int(posVal[4])

        params = {
                'arrPos':arrPos,
                'first':firstParamStatus,
                'second':secondParamStatus,
                'third':thirdParamStatus,
                'data':data
            }
        if opcode == 1: # add
            arrPos = codeOne(params)
        elif opcode == 2: # multi
            arrPos = codeTwo(params)
        elif opcode == 3: # input to destination
            arrPos = codeThree(params)
        elif opcode == 4: # output value
            arrPos = codeFour(params)

if __name__ == "__main__":
    main()