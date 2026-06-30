# Piliwela

> **Sinhala Document Understanding Toolkit for Legacy and Unicode PDFs**

Piliwela is an open-source toolkit designed to process Sinhala documents, including legacy Sinhala fonts, scanned PDFs, and mixed Sinhala-English content. It provides a complete pipeline for document extraction, Unicode conversion, metadata extraction, chunking, and semantic retrieval.

The project aims to make Sinhala documents searchable and usable for modern NLP, RAG, and AI applications.

---

## ✨ Features

### 📄 Document Processing

* PDF text extraction
* Scanned PDF support via OCR
* Mixed Sinhala-English document handling
* Page-level and document-level metadata extraction

### 🔤 Legacy Sinhala Font Conversion

Supports conversion from legacy Sinhala fonts to Unicode, including:

* FM Abhaya
* DL Fonts
* Kaputa
* Wijeya
* Other extensible mappings

### 🧠 Document Understanding

* Automatic title extraction
* Subtitle and heading detection
* Table extraction
* Semantic chunk generation

### 🔎 Retrieval Ready

* Chunk metadata generation
* Embedding-ready output
* Integration with vector databases such as:

  * Qdrant
  * pgvector
  * Pinecone
  * Weaviate

### ⚡ High Performance

* Rust-based conversion engine
* Memory-efficient processing
* Extensible mapping architecture
* Designed for large-scale document pipelines

---

# Architecture

```text
PDF
 │
 ├── Text PDF ───────────────┐
 │                           │
 └── Scanned PDF ── OCR ─────┘
             │
             ▼
     Legacy Font Detection
             │
             ▼
      Unicode Conversion
             │
             ▼
      Metadata Extraction
             │
             ▼
      Chunk Generation
             │
             ▼
     Embeddings / Vector DB
```

---

# Installation

## Python Package

```bash
pip install piliwela
```

## Development Installation

```bash
git clone https://github.com/apeironaut/piliwela.git
cd piliwela

python -m venv .venv
source .venv/bin/activate

pip install -r requirements.txt
```

---

# Rust Core

Build the Rust conversion engine:

```bash
cd piliwela_core
cargo build --release
```

Run tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test test_name
```

Run tests in a specific module:

```bash
cargo test fm
```

---

# Quick Start

## Extract Text

```python
from piliwela import Document

doc = Document("example.pdf")

print(doc.text())
```

---

## Extract Titles

```python
doc.titles()
```

---

## Extract Subtitles

```python
doc.subtitles()
```

---

## Extract Tables

```python
doc.tables(page=1)
```

---

## Generate Chunks

```python
doc.chunks()
```

Example:

```python
[
    {
        "text": "...",
        "page": 1,
        "title": "...",
        "subtitle": "...",
        "chunk_id": "..."
    }
]
```

---

# Legacy Font Conversion

```python
from piliwela.converter import convert

text = "flda"
unicode_text = convert(text)
```

---

# Supported Fonts

| Font      | Status |
| --------- | ------ |
| FM Abhaya | ✅      |
| DL        | ✅      |
| Kaputa    | ✅      |
| Wijeya    | ✅      |
| Sarasavi  | 🚧     |

---

# Project Structure

```text
piliwela/
├── piliwela/
│   ├── extractor/
│   ├── detector/
│   ├── converter/
│   ├── metadata/
│   ├── chunking/
│   └── embeddings/
│
├── piliwela_core/
│   ├── engine/
│   ├── mappings/
│   ├── detector/
│   └── tests/
│
├── examples/
├── tests/
└── docs/
```

---

# Use Cases

* Retrieval-Augmented Generation (RAG)
* Semantic Search
* Government Document Digitization
* Sinhala Knowledge Bases
* Educational Content Processing
* Legal Document Processing
* Historical Archive Preservation

---

# Roadmap

* [x] Legacy font conversion engine
* [x] Mixed-language support
* [x] Metadata extraction
* [ ] OCR pipeline improvements
* [ ] Layout-aware chunking
* [ ] Table understanding
* [ ] Multimodal document processing
* [ ] Distributed processing

---

# Contributing

Contributions are welcome.

```bash
git checkout -b feature/my-feature
git commit -m "Add feature"
git push origin feature/my-feature
```

Then open a Pull Request.

---

# Benchmarks

Coming soon.

---

# License

Apache License 2.0

---

# Citation

```bibtex
@software{piliwela,
  title={Piliwela: Sinhala Document Understanding Toolkit},
  author={Apeironaut},
  year={2026},
  url={https://github.com/apeironaut/piliwela}
}
```

---

# Vision

> Build the infrastructure layer that enables Sinhala documents to participate in modern AI systems.
