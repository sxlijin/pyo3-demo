def hello_world():
    """This is a docstring for the hello_world function."""
    ...

def modify_string(text: str) -> str:
    """This is a docstring for the modify_string function."""
    ...

class MyRustObject:
    """A Rust object exposed to Python."""
    @staticmethod
    def my_static_method(text: str) -> "MyRustObject":
        """Creates a MyRustObject via a static method."""
        ...

    def __init__(self, text: str) -> None:
        """Creates a MyRustObject via constructor."""
        ...

    def print(self) -> None:
        """Prints the object's text."""
        ...

def start_server_via_tokio(addr: str) -> None:
    """Starts an HTTP server at the specified address.

    Args:
        addr: The address to bind to (e.g. '127.0.0.1:8000')
    """
    ...

async def start_server_via_asyncio(addr: str) -> None:
    """Starts an HTTP server at the specified address.

    Args:
        addr: The address to bind to (e.g. '127.0.0.1:8000')
    """
    ...
