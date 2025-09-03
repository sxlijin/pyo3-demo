import pydantic
from my_rust_crate import (
    RustOptionsObject,
    make_lorem_ipsum,
    make_lorem_ipsum_from_python,
)


class PythonOptionsObject(pydantic.BaseModel):
    repeat: int
    crab_emoji: bool
    newlines: bool


def main():
    options = RustOptionsObject(repeat=3, crab_emoji=True, newlines=True)
    print(make_lorem_ipsum(options))


if __name__ == "__main__":
    main()
