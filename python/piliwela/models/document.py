from dataclasses import dataclass, field
from .page import Page

@dataclass
class Document:
    id: str
    name: str
    path: str | None = None
    source_type: str = "pdf"
    
    language: str = "si"
    
    pages: list[Page] = field(default_factory = list)
    
    metadata: dict = field(default_factory = dict)
    