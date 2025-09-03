#!/usr/bin/env python3
from my_rust_crate import (
    RustOptionsObject,
    make_lorem_ipsum,
    make_lorem_ipsum_from_python,
)
import multiprocessing
import pydantic


class PythonOptionsObject(pydantic.BaseModel):
    repeat: int
    crab_emoji: bool
    newlines: bool


def main():
    opts = RustOptionsObject(repeat=3, crab_emoji=True, newlines=False)

    p1 = multiprocessing.Process(target=make_lorem_ipsum, args=(opts,))
    p1.start()
    print(p1.join())


if __name__ == "__main__":
    main()
