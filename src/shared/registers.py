import enum
from typing import Tuple, List

@enum.unique
class Registers(enum.IntEnum):

    def _generate_next_value_(name: str, start: int, count: int, last_values: List[int]) -> int:
        return count

    # General purpose registers

    A = enum.auto()
    B = enum.auto()
    C = enum.auto()
    D = enum.auto()
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


register_names: Tuple[str] = \
(
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",

    "STACK_POINTER",

    "PROGRAM_COUNTER"

    "ZERO_FLAG",
    "SIGN_FLAG"
)

