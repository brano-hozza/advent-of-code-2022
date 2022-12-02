with open('input.txt') as f:
    lines = f.readlines()
    max = 0
    l_total = 0
    for line in lines:
        if line == "\n":
            if max < l_total:
                max = l_total
            l_total = 0
            continue
        l_total += int(line[:-1])
    print(max)
