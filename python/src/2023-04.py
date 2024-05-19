import re
import sys
from pathlib import Path
from typing import List, Optional

# Puzzle link:
# https://adventofcode.com/2023/day/4

INPUT_FILENAME = "2023-04-sample.txt"


def logic(buffer: List[str]) -> None:
    """Puzzle logic"""

    sum_points = get_sum(buffer)
    scratchcard_count = get_scratchcard_count(buffer)

    # assert sum_points == 27454
    # assert sum_spelled_out_digits == 53592

    print(f"Part 1 | Total winning points {sum_points}")
    print(f"Part 2 | Scratchcard {scratchcard_count}")


def get_sum(buffer: List[str]) -> int:
    """
    Given a list of scratchcard numbers and the corresponding
    winning numbers, get the total points based on the number
    of instances in the set of winning numbers
    """
    allsum = 0
    for line in buffer:
        _, numbers = line.split(":")
        winning_numbers = numbers.split("|")[0].split()
        scratchcard_numbers = numbers.split("|")[1].split()

        card_point = 0
        for num in scratchcard_numbers:
            if num in winning_numbers:
                if card_point != 0:
                    card_point *= 2
                else:
                    card_point = 1

        allsum += card_point
    return allsum


def get_scratchcard_count(buffer: List[str]) -> int:
    """ """
    allsum = 0
    modifiedbuffer = []
    for index, line in enumerate(buffer):
        _, numbers = line.split(":")
        winning_numbers = numbers.split("|")[0].split()
        scratchcard_numbers = numbers.split("|")[1].split()

        new_scratchcards = 0
        for num in scratchcard_numbers:
            if num in winning_numbers:
                new_scratchcards += 1

        modifiedbuffer.append(line)
        for i in range(1, new_scratchcards + 1):
            modifiedbuffer.append(buffer[index + i] + numbers)

    for line in modifiedbuffer:
        card, numbers = line.split(":")

        winning_numbers = numbers.split("|")[0].split()
        scratchcard_numbers = numbers.split("|")[1].split()

        new_scratchcards = 0
        for num in scratchcard_numbers:
            if num in winning_numbers:
                new_scratchcards += 1

        print(card, new_scratchcards)
        allsum += new_scratchcards
    return allsum


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
