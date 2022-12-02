from enum import Enum


class Oponent(Enum):
    A = 1  # Rock
    B = 2  # Paper
    C = 3  # Scissors


class State(Enum):
    X = {1: 3, 2: 1, 3: 2, 'val': 0}
    Y = {1: 1, 2: 2, 3: 3, 'val': 3}
    Z = {1: 2, 2: 3, 3: 1, 'val': 6}


with open('input.txt') as f:
    lines = f.readlines()
    lines[-1] += '\n'
    matches = [line[:-1]for line in lines]
    points = 0
    for match in matches:
        x, y = match.split(' ')
        x_val = Oponent[x].value
        result = State[y].value
        y_val = result[x_val]
        points += y_val + result['val']
    print(points)
