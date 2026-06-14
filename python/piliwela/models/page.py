from dataclasses import dataclass, field
from typing import Any

from .block import Block

@dataclass
class Page:
    number: int

    width: float | None = None
    height: float | None = None

    blocks: list[Block] = field(default_factory=list)

    metadata: dict[str, Any] = field(default_factory=dict)