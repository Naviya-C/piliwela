class Page:
    def __init__(
        self,
        number: int,
        lines,
    ):
        self.number = number
        self.lines = lines

    def text(self):
        return "\n".join(
            line.text()
            for line in self.lines
        )