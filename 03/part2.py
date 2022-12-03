with open('input.txt') as f:
    lines = f.readlines()
    lines[-1] += '\n'
    bags = [line[:-1]for line in lines]
    groups_of_3 = [bags[i:i + 3] for i in range(0, len(bags), 3)]
    total = 0
    for group in groups_of_3:
        values = dict()
        found = False
        for bag in group:
            occurances = set()
            for char in bag:
                occurances.add(char)
            for key in occurances:
                if key in values:
                    values[key] += 1
                    if values[key] == 3:
                        char_code = ord(key)
                        if char_code > 96:
                            total += char_code - 96
                        else:
                            total += char_code - 38
                        found = True
                        break
                else:
                    values[key] = 1
            if found:
                break
    print(total)
