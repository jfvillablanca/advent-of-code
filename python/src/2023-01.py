import re
import sys
from pathlib import Path
from typing import List, Optional

# Puzzle link:
# https://adventofcode.com/2023/day/1

INPUT_FILENAME = "2023-01-input.txt"


def logic(buffer: List[str]) -> None:
    """Puzzle logic"""

    sum_digits = get_sum(buffer)
    sum_spelled_out_digits = get_sum([substitute_digit(line) for line in buffer])

    assert sum_digits == 55621
    assert sum_spelled_out_digits == 53592

    print(f"Part 1 | Sum of combined numeric digits {sum_digits}")
    print(
        f"Part 2 | Sum of combined numeric or spelled out digits {sum_spelled_out_digits}"
    )


def get_sum(buffer: List[str]) -> int:
    """
    Given the first and last parsed digit for every
    line, create a two-digit number from both and
    get the sum for all the lines
    """
    allsum = 0
    for line in buffer:
        first = find_digit(line)
        last = find_digit(line, find_first=False)

        if first is not None and last is not None:
            allsum += first * 10 + last
    return allsum


def find_digit(line: str, find_first=True) -> Optional[int]:
    """
    Return the first (last) digit in a line
    """
    line = line if find_first else list(reversed(line))
    for char in line:
        if char.isdigit():
            return int(char)
    return None


def substitute_digit(line: str) -> str:
    """
    Find instances of 'spelled out' digits then substitute
    the string with appropriate digit char
    """
    digits = {
        "one": "o1e",
        "two": "t2o",
        "three": "t3ree",
        "four": "f4ur",
        "five": "f5ve",
        "six": "s6x",
        "seven": "s7ven",
        "eight": "e8ght",
        "nine": "n9ne",
    }
    substituted = line

    for digit, sub in digits.items():
        pos = line.find(digit)
        if pos != -1:
            substituted = re.sub(digit, sub, substituted)

    return substituted


if __name__ == "__main__":
    buf = []
    try:
        file_path = Path(__file__).parent / f"../../shared/{INPUT_FILENAME}"
        with open(
            file_path,
            encoding="utf-8",
        ) as file:
            for bufline in file:
                buf.append(bufline.rstrip())

    except FileNotFoundError:
        sys.exit("Puzzle input file does not exist")

    logic(buf)


######### TESTS #########
def test_find_digit():
    assert find_digit("two1nine") == 1
    assert find_digit("twonine") is None
    assert find_digit("a1b2c3d4e5f") == 1

    assert find_digit("a1b2c3d4e5f", find_first=False) == 5


def test_sub_spelled_out_digit():
    assert substitute_digit("eightwothree") == "e8ght2ot3ree"
    assert substitute_digit("eightwo") == "e8ght2o"


def test_get_sum():
    test_buf1 = [
        "1abc2",  # 12
        "pqr3stu8vwx",  # 38
        "a1b2c3d4e5f",  # 15
        "treb7uchet",  # 77
    ]
    assert get_sum(test_buf1) == 142

    test_buf2 = [
        "two1nine",  # 11
        "eightwothree",  # 0
        "abcone2threexyz",  # 22
        "xtwone3four",  # 33
        "4nineeightseven2",  # 42
        "zoneight234",  # 24
        "7pqrstsixteen",  # 77
    ]
    assert get_sum(test_buf2) == 209


def test_get_sum_spelled_out():
    test_buf = [
        "two1nine",  # 29
        "eightwothree",  # 83
        "abcone2threexyz",  # 13
        "xtwone3four",  # 24
        "4nineeightseven2",  # 42
        "zoneight234",  # 14
        "7pqrstsixteen",  # 76
    ]
    assert get_sum([substitute_digit(line) for line in test_buf]) == 281
