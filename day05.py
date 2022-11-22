import sys


def at_least_three_vowels(line: str) -> bool:
    return sum(c in set("aeiou") for c in line) >= 3


def has_repeated_letter(line: str) -> bool:
    return any(a == b for a, b in zip(line, line[1:]))


def has_no_special_substrings(line: str) -> bool:
    return not any(special in line for special in ["ab", "cd", "pq", "xy"])


def has_repeated_letter_pair(line: str) -> bool:
    for i, p1 in enumerate(zip(line, line[1:])):
        for p2 in zip(line[i + 2:], line[i + 3:]):
            if p1 == p2:
                return True
    return False


def has_repeated_letter_with_one_in_between(line: str) -> bool:
    for a, _, c in zip(line, line[1:], line[2:]):
        if a == c:
            return True
    return False


part1_predicates = [
    at_least_three_vowels,
    has_repeated_letter,
    has_no_special_substrings,
]

part2_predicates = [
    has_repeated_letter_pair,
    has_repeated_letter_with_one_in_between,
]

part1, part2 = 0, 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        break
    part1 += 1 if all(pred(line) for pred in part1_predicates) else 0
    part2 += 1 if all(pred(line) for pred in part2_predicates) else 0

print(part1)
print(part2)
