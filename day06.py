from __future__ import annotations

import sys
from typing import Iterator, NamedTuple


class Point(NamedTuple):
    x: int
    y: int

    @classmethod
    def parse(cls, s: str) -> Point:
        return cls(*[int(c) for c in s.split(",")])

    def __add__(self, other: Point) -> Point:
        return Point(self.x + other.x, self.y + other.y)


class Area(NamedTuple):
    tl: Point
    br: Point

    @classmethod
    def parse(cls, tl: str, br: str) -> Area:
        tl = Point.parse(tl)
        br = Point.parse(br)
        assert tl.x <= br.x and tl.y <= br.y
        return cls(tl, br + Point(1, 1))

    def points(self) -> Iterator[Point]:
        for x in range(self.tl.x, self.br.x):
            for y in range(self.tl.y, self.br.y):
                yield Point(x, y)


def parse(f):
    for line in f:
        instruction, topleft, through, bottom_right = line.rstrip().rsplit(" ", 3)
        assert through == "through"
        yield instruction, Area.parse(topleft, bottom_right)


input = list(parse(sys.stdin))

# Part 1
on: set[Point] = set()
for instruction, area in input:
    for p in area.points():
        if instruction == "turn on":
            on.add(p)
        elif instruction == "turn off":
            on.discard(p)
        else:
            if p in on:
                on.discard(p)
            else:
                on.add(p)
print(f"Part 1: {len(on)}")

# Part 2
on: dict[Point, int] = {}
for instruction, area in input:
    for p in area.points():
        if instruction == "turn on":
            on[p] = on.get(p, 0) + 1
        elif instruction == "turn off":
            on[p] = max(0, on.get(p, 0) - 1)
        else:
            on[p] = on.get(p, 0) + 2
print(f"Part 2: {sum(on.values())}")
