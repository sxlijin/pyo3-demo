import my_rust_crate
import queue


class StreamGlue:
    def __init__(self):
        self.q = queue.Queue()

    def __aiter__(self):
        return self

    async def __anext__(self):
        item = self.q.get()
        if item is None:
            raise StopAsyncIteration
        return item

    def put_nowait(self, item):
        self.q.put_nowait(item)


async def async_main():
    stream = StreamGlue()
    my_rust_crate.MyRustStream3.new(stream.put_nowait)

    async for event_data in stream:
        print(f"Received stream event: {event_data}")


def main():
    my_rust_crate.blocking_fetch_sse_stream("https://sse.dev/test")
    # Run async code
    # asyncio.run(async_main())
    o = my_rust_crate.MyRustObject("Hello, world!")
    print(o)
    o.print()


if __name__ == "__main__":
    main()
