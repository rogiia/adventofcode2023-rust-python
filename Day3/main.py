import collections, math

# A 2d grid stored as a dict keyed on complex values
grid = {
    x + y * 1j: e
    for y, l in enumerate(open("input.txt").read().split("\n"))
    for x, e in enumerate(l)
}

# part_sum is the answer to problem 1
# stars is a dictionary where the key will be star coordinates and the
#   values are a list of all adjacent numbers.
part_sum, stars = 0, collections.defaultdict(list)

for y in range(max(int(c.imag) for c in grid) + 1):
    # sym_adj is whether the current number is adjacent to a symbol
    # adj_stars is a set of all adjacent stars to the current number
    # cur_num is whatever the number we're currently building, zero otherwise
    sym_adj, adj_stars, cur_num = False, set(), 0
    for x in range(max(int(c.real) for c in grid) + 2):
        char = grid.get(x + y * 1j, ".")

        # When we see a number, we need to add it to the number we're constructing.
        # At the same time, we need to check if there are any adjacent symbols
        # and, if one of those symbols is a star, keep note of those coords
        if char.isnumeric():
            cur_num = cur_num * 10 + int(char)
            for x1 in range(-1, 2):
                for y1 in range(-1, 2):
                    # Adding the x1, y1 offsets to our current x,y
                    coord = x + y * 1j + x1 + y1 * 1j
                    char = grid.get(coord, ".")
                    sym_adj |= not char in "1234567890."
                    if char == "*":
                        adj_stars.add(coord)

        # When we see anything other than a number, we need to add the cur_num
        # to part_sum if we previously found an adjacent symbol. Additionally,
        # if there's an adjacent star, we add it to the stars dict.
        else:
            if cur_num and sym_adj:
                part_sum += cur_num
                for s in adj_stars:
                    stars[s].append(cur_num)

            sym_adj, adj_stars, cur_num = False, set(), 0

print(part_sum)
print(sum(map(math.prod, [s for s in stars.values() if len(s) == 2])))
