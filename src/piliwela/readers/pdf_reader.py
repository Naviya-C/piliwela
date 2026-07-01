import fitz  # PyMuPDF

import piliwela_core
from piliwela.models.document import Document
from piliwela.models.page import Page
from piliwela.models.line import Line
from piliwela.models.span import Span


# If two adjacent spans have a horizontal gap wider than this fraction of the
# font size but neither carries a boundary space, a space is inserted so the
# words don't glue together. Set to None to disable (exact original behaviour).
GAP_SPACE_RATIO = 0.20


def _span_from_dict(span_dict) -> Span:
    """Convert one PyMuPDF span dict into a Span, applying font conversion."""
    raw_text = span_dict["text"]
    font = span_dict["font"]

    family = piliwela_core.detect_from_metadata(font)
    converted = piliwela_core.convert_auto_with_metadata(raw_text, font)

    return Span(
        text=converted,
        raw_text=raw_text,
        font=font,
        font_family=family,
        size=span_dict["size"],
        bbox=tuple(span_dict["bbox"]),          # (x0, y0, x1, y1)
        converted=(converted != raw_text),
    )


def _needs_space(prev: dict, curr: dict) -> bool:
    """True if a space should be inserted between two adjacent spans."""
    if GAP_SPACE_RATIO is None:
        return False
    prev_t, curr_t = prev["text"], curr["text"]
    if not prev_t or not curr_t:
        return False
    if prev_t.endswith((" ", "\t")) or curr_t.startswith((" ", "\t")):
        return False
    gap = curr["bbox"][0] - prev["bbox"][2]
    size = prev["size"] or 10.0
    return gap > GAP_SPACE_RATIO * size


def _line_from_dict(line_dict) -> Line | None:
    """Build a Line from a PyMuPDF line dict, left-to-right, or None if empty."""
    ordered = sorted(line_dict["spans"], key=lambda s: s["bbox"][0])

    spans = []
    for i, span_dict in enumerate(ordered):
        if span_dict["text"] == "":
            continue
        # insert a synthetic space span when two real spans would glue
        if spans and _needs_space(ordered[i - 1], span_dict):
            spans.append(Span(
                text=" ", raw_text=" ", font="", font_family="Unknown",
                size=span_dict["size"], bbox=None, converted=False,
            ))
        spans.append(_span_from_dict(span_dict))

    if not spans:
        return None
    return Line(spans=spans, y0=line_dict["bbox"][1])


def read_pdf(path: str) -> Document:
    """
    Read a PDF with PyMuPDF and map it to Document -> Page -> Line -> Span,
    preserving reading order and font metadata and applying FM conversion.
    """
    doc = fitz.open(path)
    pages = []

    for page_no, page in enumerate(doc):
        blocks = [b for b in page.get_text("dict").get("blocks", []) if "lines" in b]
        # reading order: top-to-bottom, then left-to-right (single-column pages)
        blocks.sort(key=lambda b: (b["bbox"][1], b["bbox"][0]))

        page_lines = []
        for block in blocks:
            for line_dict in sorted(block["lines"], key=lambda l: l["bbox"][1]):
                line = _line_from_dict(line_dict)
                if line is not None:
                    page_lines.append(line)

        pages.append(Page(number=page_no, lines=page_lines))

    return Document(pages=pages)