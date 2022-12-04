with open('input.txt') as f:
    lines = f.readlines()
    elves = []
    l_total = 0
    for line in lines:
        if line == "\n":
            elves.append(l_total)
            l_total = 0
            continue
        l_total += int(line[:-1])
    elves.sort()
    print(sum(elves[-3:]))
