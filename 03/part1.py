with open('input.txt') as f:
    lines = f.readlines()
    lines[-1] += '\n'
    bags = [line[:-1]for line in lines]
    total = 0
    for bag in bags:
        items_c = len(bag)
        firstHalf = bag[:items_c//2]
        secondHalf = bag[items_c//2:]
        firstChars = set()
        for (char) in firstHalf:
            firstChars.add(char)
        # find the first char that is in the second half
        found_char = None
        for (char) in secondHalf:
            if char in firstChars:
                found_char = char
                break
        # get char code
        char_code = ord(found_char)
        if char_code > 96:
            total += char_code - 96
        else:
            total += char_code - 38
    print(total)
