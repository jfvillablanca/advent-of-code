import re
import sys
from pathlib import Path
from typing import List, Optional

# Puzzle link:
# https://adventofcode.com/2023/day/3

INPUT_FILENAME = "2023-03-input.txt"


def logic(buffer: List[str]) -> None:
    """Puzzle logic"""

    sum_digits = get_sum(buffer)
    # sum_spelled_out_digits = get_sum([substitute_digit(line) for line in buffer])

    # TOO HIGH: 9660531
    # TOO LOW:   489170

    # assert sum_digits == 55621
    # assert sum_spelled_out_digits == 53592

    print(f"Part 1 | Sum of combined numeric digits {sum_digits}")
    # print(
    #     f"Part 2 | Sum of combined numeric or spelled out digits {sum_spelled_out_digits}"
    # )


def get_sum(buffer: List[str]) -> int:
    """
    Given the first and last parsed digit for every
    line, create a two-digit number from both and
    get the sum for all the lines
    """
    allsum = 0

    directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]

    for x, line in enumerate(buffer):
        current_number_str = ""
        is_part_number = False
        for y, char in enumerate(line):
            # if is_dot(char):
            if is_dot(char):
                if current_number_str and is_part_number:
                    print(int(current_number_str))
                    allsum += int(current_number_str)

                current_number_str = ""
                is_part_number = False

            if is_symbol(char):
                current_number_str = ""
                is_part_number = False

            if char.isdigit():
                current_number_str += char
                # print(char, (x,y))
                # print("<==")
                for dx, dy in directions:
                    new_x = x + dx
                    new_y = y + dy
                    if 0 <= new_x < len(buffer) and 0 <= new_y < len(line):
                        if is_symbol(buffer[new_x][new_y]):
                            # print(" - ", buffer[new_x][new_y], (new_x, new_y), is_part_number)
                            is_part_number = True
                            break
                # print("==>")

        if current_number_str and is_part_number:
            print(int(current_number_str))

            allsum += int(current_number_str)

    return allsum


def is_dot(c: str) -> bool:
    return c == "."


def is_symbol(c: str) -> bool:
    return not c.isdigit() and not is_dot(c)


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
# def test_find_digit():
#     assert find_digit("two1nine") == 1
#     assert find_digit("twonine") is None
#     assert find_digit("a1b2c3d4e5f") == 1

#     assert find_digit("a1b2c3d4e5f", find_first=False) == 5


# def test_sub_spelled_out_digit():
#     assert substitute_digit("eightwothree") == "e8ght2ot3ree"
#     assert substitute_digit("eightwo") == "e8ght2o"


# def test_get_sum():
#     test_buf1 = [
#         "1abc2",  # 12
#         "pqr3stu8vwx",  # 38
#         "a1b2c3d4e5f",  # 15
#         "treb7uchet",  # 77
#     ]
#     assert get_sum(test_buf1) == 142

#     test_buf2 = [
#         "two1nine",  # 11
#         "eightwothree",  # 0
#         "abcone2threexyz",  # 22
#         "xtwone3four",  # 33
#         "4nineeightseven2",  # 42
#         "zoneight234",  # 24
#         "7pqrstsixteen",  # 77
#     ]
#     assert get_sum(test_buf2) == 209


# def test_get_sum_spelled_out():
#     test_buf = [
#         "two1nine",  # 29
#         "eightwothree",  # 83
#         "abcone2threexyz",  # 13
#         "xtwone3four",  # 24
#         "4nineeightseven2",  # 42
#         "zoneight234",  # 14
#         "7pqrstsixteen",  # 76
#     ]
#     assert get_sum([substitute_digit(line) for line in test_buf]) == 281
