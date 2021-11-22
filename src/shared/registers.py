import enum
from typing import Tuple, List

@enum.unique
class Registers(enum.IntEnum):

    def _generate_next_value_(name: str, start: int, count: int, last_values: List[int]) -> int:
        return count

    # General purpose registers

    # First arithmetical register A
    A = enum.auto()
    # Second arithmetical register B
    B = enum.auto()
    C = enum.auto()
    D = enum.auto()
    # Exit status register E
    E = enum.auto()
    F = enum.auto()
    G = enum.auto()
    # Print register H
    H = enum.auto()

    # Stack pointer

    STACK_POINTER = enum.auto()

    # Program counter

    PROGRAM_COUNTER = enum.auto()

    # Flags

    ZERO_FLAG = enum.auto()
    SIGN_FLAG = enum.auto()
    REMAINDER_FLAG = enum.auto()


register_names: Tuple[str] = \
(
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",

    "sp",

    "pc"

    "zf",
    "sf",
    "rf",
)

