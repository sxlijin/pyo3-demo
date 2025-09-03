from typing import Any, Callable

class RustOptionsObject:
    """Rust-backed options object for make_lorem_ipsum."""

    repeat: int
    crab_emoji: bool
    newlines: bool

    def __init__(self, *, repeat: int, crab_emoji: bool, newlines: bool) -> None:
        """Creates a RustOptionsObject."""
        ...

    @staticmethod
    def from_json_string(json_string: str) -> "RustOptionsObject":
        """Creates a RustOptionsObject from a JSON string."""
        ...

def make_lorem_ipsum(options: RustOptionsObject) -> str:
    """Generates a Lorem Ipsum string using Rust code."""
    ...

def make_lorem_ipsum_from_python(options: Any) -> str:
    """Generates a Lorem Ipsum string using Rust code."""
    ...

def callback_driven_stream(url: str, callback_ref: Callable[[str], None]) -> None:
    """Creates a callback-driven stream."""
    ...
