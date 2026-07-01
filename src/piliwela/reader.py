from pathlib import Path
from piliwela.readers.pdf_reader import read_pdf


def read(path: str):
    path = Path(path)

    suffix = path.suffix.lower()

    if suffix == ".pdf":
        return read_pdf(str(path))

    raise ValueError(
        f"Unsupported file type: {suffix}"
    )