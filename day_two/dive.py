"""Work for the second day of the advent of code."""

from typing import Tuple


def part_one_trajectory(data_file: str) -> int:
    """Compute the product of the final x- and y-coordinates of a
    submarine whose trajectory is defined by the data in dive.data.
    This depends on these definitions.

    - "forward X" means increase the horizontal coordinate by X
    - "down X" means increase the vertical coordinate by X
    - "up X" means decrease the vertical coordinate by X
    """
    position: Tuple[int, int] = (0, 0)
    with open(data_file, "r") as dive_data:
        for command in dive_data:
            command_parts = command.split()
            if command_parts[0] == "forward":
                position = (position[0] + int(command_parts[1]), position[1])
            elif command_parts[0] == "down":
                position = (position[0], position[1] + int(command_parts[1]))
            elif command_parts[0] == "up":
                position = (position[0], position[1] - int(command_parts[1]))
            else:
                raise ValueError(f"Unexpected operation `{command_parts[0]}`")
    return position[0] * position[1]


def part_two_trajectory(data_file: str) -> int:
    """Similar to part one, but the command definitions are different.

    - "down X" increases the `aim` by X
    - "up X" decreases the `aim` by X
    - "forward X" increases the horizontal coordinate by X and increases
      the vertical coordinate by aim * X
    """
    position: Tuple[int, int] = (0, 0)
    aim = 0
    with open(data_file, "r") as dive_data:
        for command in dive_data:
            command_parts = command.split()
            if command_parts[0] == "forward":
                position = (
                    position[0] + int(command_parts[1]),
                    position[1] + int(command_parts[1]) * aim,
                )
            elif command_parts[0] == "down":
                aim += int(command_parts[1])
            elif command_parts[0] == "up":
                aim -= int(command_parts[1])
            else:
                raise ValueError(f"Unexpected operation `{command_parts[0]}`")
    return position[0] * position[1]


def main():
    """Main."""
    print(part_one_trajectory("dive.data"))
    print(part_two_trajectory("dive.data"))


if __name__ == "__main__":
    main()
