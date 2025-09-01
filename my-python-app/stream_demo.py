import my_rust_crate
import asyncio
import queue


async def rust_backed_stream(url: str):
    q = queue.Queue()
    my_rust_crate.CallbackDrivenStream(q.put_nowait, url)
    while True:
        item = q.get()
        if item is None:
            break
        yield item


async def main():
    async for event_data in rust_backed_stream("https://sse.dev/test"):
        print(f"Received stream event: {event_data}")


if __name__ == "__main__":
    asyncio.run(main())
