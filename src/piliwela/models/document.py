class Document:
    def __init__(
        self,
        pages,
    ):
        self.pages = pages

    def text(self):
        return "\n\n".join(
            page.text()
            for page in self.pages
        )