def parse(s: str):
    lines = s.splitlines()
    blank_found = False
    ranges = []
    ingredients = []
    for line in lines:
        if len(line) == 0:
            blank_found = True
            continue
        if blank_found:
            ingredients.append(int(line))
        else:
            ranges.append([int(n) for n in line.split("-")])
    return ranges, ingredients

def in_range(n: int, range: list[int]):
    return n >= range[0] and n <= range[1]

def in_any_range(n, ranges):
    for r in ranges:
        if in_range(n, r):
            return True
    return False

def can_combine(a: list[int], b: list[int]):
    min_a, max_a = a
    min_b, max_b = b
    c = in_range(min_a, b)
    d = in_range(max_a, b)
    e = in_range(min_b, a)
    f = in_range(max_b, a)
    if c and d:
        return b
    if e and f:
        return a
    if c or d or e or f:
        min_v = min(min_a, min_b)
        max_v = max(max_a, max_b)
        return [min_v, max_v]
    return None

def find_combinable(ranges: list[list[int]]):
    for start in range(0, len(ranges) - 1):
        for idx in range(start+1, len(ranges)):
            combo = can_combine(ranges[start], ranges[idx])
            if combo:
                return start, idx, combo
    return None

def weld_ranges(ranges: list[list[int]]):
    while True:
        found = find_combinable(ranges)
        if found:
            start, idx, combo = found
            ranges.pop(idx)
            ranges.pop(start)
            ranges.append(combo)
        else:
            break
    return ranges

def pt1():
    with open("text.txt") as f:
        ranges, ingredients = parse(f.read())
        res = 0
        for i in ingredients:
            if in_any_range(i, ranges):
                res += 1
        print(res)

def pt2():
    with open("text.txt") as f:
        ranges, _ = parse(f.read())
        res = 0
        for r in ranges:
            res += r[1] - r[0] + 1
        print(res)


if __name__ == "__main__":
    pt1()
    pt2()