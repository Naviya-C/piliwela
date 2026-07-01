from dataclasses import dataclass

@dataclass
class Span:
    text: str
    raw_text: str
    font: str
    font_family: str
    size: float
    bbox: tuple[float, float, float, float]
    converted: bool