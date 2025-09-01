#!/usr/bin/env python3
from my_rust_crate import LoremIpsumOptions, make_lorem_ipsum
import multiprocessing


def main():
    opts = LoremIpsumOptions(repeat=3, crab_emoji=True, newlines=False)

    p1 = multiprocessing.Process(target=make_lorem_ipsum, args=(opts,))
    p2 = multiprocessing.Process(target=make_lorem_ipsum, args=(opts,))

    p1.start()
    p2.start()

    print(p1.join())
    print(p2.join())


if __name__ == "__main__":
    main()
