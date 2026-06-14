from dataclasses import dataclass, field
from typing import Any, Literal

@dataclass
class Block:
    id: str

    text: str

    page_number: int
    
    confidence: float | None = None
    
    block_type: Literal[
        "paragraph",
        "heading",
        "table",
        "image",
        "caption",
        "list",
        "code",
        "header",
        "footer"
    ] = "paragraph"
    
    order: int = 0

    font_name: str | None = None
    font_size: float | None = None

    is_bold: bool = False
    is_italic: bool = False

    hierarchy: list[str] = field(default_factory=list)

    bbox: tuple[float, float, float, float] | None = None

    metadata: dict[str, Any] = field(default_factory=dict)