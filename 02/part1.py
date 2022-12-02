from enum import Enum


class Oponent(Enum):
    A = 1  # Rock
    B = 2  # Paper
    C = 3  # Scissors


class You(Enum):
    X = 1
    Y = 2
    Z = 3


def evaluate(x, y):
    res = x - y
    if res == -1 or res == 2:
        return 6
    if res == 0:
        return 3
    return 0


with open('input.txt') as f:
    lines = f.readlines()
    lines[-1] += '\n'
    matches = [line[:-1]for line in lines]
    points = 0
    for match in matches:
        x, y = match.split(' ')
        x_val = Oponent[x].value
        y_val = You[y].value
        r_points = evaluate(x_val, y_val)
        points += r_points + y_val
        print(r_points, y_val)
    print(points)
