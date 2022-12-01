#!/usr/bin/env python

from pathlib import Path

def main() -> None:
    """https://adventofcode.com/2022/day/1"""
    with open(Path(__file__).parent.parent / 'input.txt') as f:
        nums = ' '.join(f.read().splitlines()).split('  ')

    print(sum(sorted([sum([int(x) for x in n.split(' ')]) for n in nums], reverse=True)[:3]))

if __name__ == '__main__':
    main()
