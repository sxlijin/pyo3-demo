from my_rust_crate import RustOptionsObject, make_lorem_ipsum


def main():
    options = RustOptionsObject(repeat=3, crab_emoji=True, newlines=True)
    print(make_lorem_ipsum(options))


if __name__ == "__main__":
    main()
