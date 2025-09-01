from typing import Callable

class LoremIpsumOptions:
    """A Lorem Ipsum options object."""

    def __init__(self, *, repeat: int, crab_emoji: bool, newlines: bool) -> None:
        """Creates a LoremIpsumOptions."""
        ...

    @staticmethod
    def from_json_string(json_string: str) -> "LoremIpsumOptions":
        """Creates a LoremIpsumOptions from a JSON string."""
        ...

def make_lorem_ipsum(options: LoremIpsumOptions) -> str:
    """Generates a Lorem Ipsum string."""
    ...

class CallbackDrivenStream:
    """A callback-driven stream."""

    def __init__(self, callback_ref: Callable[[str], None], url: str) -> None:
        """Creates a CallbackDrivenStream."""
        ...
