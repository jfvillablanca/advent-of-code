import re
import sys
from functools import partial, reduce
from operator import mul
from pathlib import Path
from typing import Dict, List

# Puzzle link:
# https://adventofcode.com/2023/day/2

INPUT_FILENAME = "2023-02-input.txt"


def logic(buffer: List[str]) -> None:
    """Puzzle logic"""

    sum_game_ids = get_sum(buffer)
    sum_of_set_powers = get_power(buffer)

    assert sum_game_ids == 2416
    assert sum_of_set_powers == 63307

    print(f"Part 1 | Sum of valid game ids {sum_game_ids}")
    print(f"Part 2 | Sum of power of sets {sum_of_set_powers}")


def get_max_color_per_game(game: str) -> Dict[str, int]:
    """
    Find the max color for each color on all sets in a game
    """
    cube_colors: Dict = {"red": 0, "green": 0, "blue": 0}
    matches = re.findall(r"\b(\d+) (red|green|blue)\b", game)
    for match in matches:
        count, color = match
        if int(count) > cube_colors[color]:
            cube_colors[color] = int(count)
    return cube_colors


def check_if_game_possible(
    reference_bag: Dict[str, int], current_bag: Dict[str, int]
) -> bool:
    """
    Returns True if all colors the `current_bag` are
    less than or equal to what's inside in the `reference_bag`
    """
    for color in reference_bag.keys():
        if current_bag[color] > reference_bag[color]:
            return False
    return True


def get_sum(buffer: List[int]) -> int:
    """
    Find the sum of the game_ids that satisfy the reference_bag
    """
    allsum = 0
    reference_bag = {"red": 12, "green": 13, "blue": 14}
    check_against_reference = partial(check_if_game_possible, reference_bag)

    for game_id, game in enumerate(buffer, start=1):
        if check_against_reference(get_max_color_per_game(game)):
            allsum += game_id

    return allsum


def get_power(buffer: List[int]) -> int:
    """
    Find the power of a set of cubes which is equal to the
    numbers of red, green, and blue cubes multiplied together.
    """
    allsum = 0
    for line in buffer:
        allsum += reduce(mul, get_max_color_per_game(line).values())
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
def test_get_max_color_per_game():
    assert get_max_color_per_game(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    ) == {"red": 4, "green": 2, "blue": 6}
    assert get_max_color_per_game(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    ) == {"red": 20, "green": 13, "blue": 6}
    assert get_max_color_per_game("Game 69: 3 blue, 4 red; 1 red, 6 blue") == {
        "red": 4,
        "green": 0,
        "blue": 6,
    }


def test_get_power():
    assert (
        get_power(
            [
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",  # 4 red, 2 green, 6 blue = 48
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",  # 12
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",  # 1560
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",  # 630
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",  # 36
            ]
        )
        == 2286
    )


def test_check_if_game_possible():
    check_against_reference = partial(
        check_if_game_possible, {"red": 1, "green": 1, "blue": 0}
    )

    assert check_against_reference({"red": 1, "green": 1, "blue": 0}) is True
    assert check_against_reference({"red": 1, "green": 0, "blue": 0}) is True
    assert check_against_reference({"red": 2, "green": 1, "blue": 0}) is False
    assert check_against_reference({"red": 2, "green": 0, "blue": 0}) is False
