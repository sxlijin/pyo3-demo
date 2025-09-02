from fastapi import FastAPI
from pydantic import BaseModel
from my_rust_crate import RustOptionsObject, make_lorem_ipsum


app = FastAPI(title="Lorem Ipsum HTTP API")


class PythonOptionsObject(BaseModel):
    repeat: int
    crab_emoji: bool
    newlines: bool


@app.post("/lorem-ipsum")
async def lorem_ipsum(py_options: PythonOptionsObject):
    options = RustOptionsObject(
        repeat=py_options.repeat,
        crab_emoji=py_options.crab_emoji,
        newlines=py_options.newlines,
    )
    return make_lorem_ipsum(options)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
