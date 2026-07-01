from piliwela.models.span import Span


class Line:
    def __init__(
        self,
        spans: list[Span],
        y0: float,
    ):
        self.spans = spans
        self.y0 = y0

    def text(self):
        return "".join(
            span.text
            for span in self.spans
        )