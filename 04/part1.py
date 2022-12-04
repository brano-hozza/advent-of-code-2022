with open('input.txt') as f:
    lines = f.readlines()
    lines[-1] += '\n'
    lines = [line[:-1]for line in lines]
    pairs = [line.split(',') for line in lines]
    total = 0
    for pair in pairs:
        firstRange = pair[0].split('-')
        secondRange = pair[1].split('-')
        # check if first range is inclusive in second range
        if int(firstRange[0]) >= int(secondRange[0]) and int(firstRange[1]) <= int(secondRange[1]):
            total += 1
            continue

        # check if second range is inclusive in first range
        if int(secondRange[0]) >= int(firstRange[0]) and int(secondRange[1]) <= int(firstRange[1]):
            total += 1

    print(total)
