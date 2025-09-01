from fastapi import FastAPI
from pydantic import BaseModel
from my_rust_crate import LoremIpsumOptions, make_lorem_ipsum

app = FastAPI(title="Lorem Ipsum HTTP API")


class PythonLoremIpsumOptions(BaseModel):
    repeat: int
    crab_emoji: bool
    newlines: bool


@app.post("/lorem-ipsum")
async def lorem_ipsum(options: LoremIpsumOptions):
    return make_lorem_ipsum(options)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
