from my_rust_crate import RustOptionsObject, make_lorem_ipsum
import pydantic


class PythonOptionsObject(pydantic.BaseModel):
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
    options = RustOptionsObject.from_json_string(json_string)
    print(make_lorem_ipsum(options))


if __name__ == "__main__":
    main()
