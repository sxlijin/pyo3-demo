import my_rust_crate
from pydantic import BaseModel


class PythonLoremIpsumOptions(BaseModel):
    repeat: int
    crab_emoji: bool
    newlines: bool


def main():
    json_string = """
    {
        "repeat": 3,
        "crab_emoji": true,
        "newlines": true
    }
    """
    rs_options = my_rust_crate.LoremIpsumOptions.from_json_string(json_string)
    print(f"Rust options: {rs_options}")
    py_options = PythonLoremIpsumOptions.model_validate_json(json_string)
    print(f"Python options object: {py_options}")

    options = my_rust_crate.LoremIpsumOptions(repeat=3, crab_emoji=True, newlines=False)
    print(my_rust_crate.make_lorem_ipsum(options))


if __name__ == "__main__":
    main()
