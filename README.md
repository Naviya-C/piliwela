# 🌾 Piliwela

<div align="center">

<img src="docs/assets/logo.png" width="180"/>

### **Sinhala Document Understanding Toolkit**

**High-performance PDF understanding for Sinhala documents powered by Python + Rust.**

[![Python](https://img.shields.io/badge/Python-3.10+-3776AB?style=for-the-badge\&logo=python\&logoColor=white)](https://python.org)
[![Rust](https://img.shields.io/badge/Rust-1.80+-000000?style=for-the-badge\&logo=rust)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active-success?style=for-the-badge)]()
[![PyPI](https://img.shields.io/pypi/v/piliwela?style=for-the-badge)]()

---

### 🇱🇰 Bringing Sinhala documents into the modern AI ecosystem.

**Legacy Fonts → Unicode → Structured Documents → RAG**

</div>

---

# ✨ Features

## 🇱🇰 Legacy Sinhala Font Support

Convert documents written using:

* ✅ FM Abhaya
* ✅ FM Samantha
* ✅ DL Fonts
* ✅ Kaputa
* ✅ Wijeya

---

## 📄 Document Understanding API

```python
import piliwela

doc = piliwela.read("grade-10-sinhala.pdf")
```

```python
doc.text()
doc.pages()
doc.lines()
doc.spans()
doc.titles()
doc.subtitles()
doc.subsubtitles()
doc.tables()
doc.chunks()
```

---

## ⚡ Powered by Rust

* 🚀 High-performance text processing
* 🚀 Unicode normalization
* 🚀 Legacy font detection
* 🚀 Fast conversion engine

---

## 📐 Layout Preservation

Piliwela preserves:

* 📌 Pages
* 📌 Lines
* 📌 Spans
* 📌 Font metadata
* 📌 Bounding boxes
* 📌 Reading order

---

# 🏗 Architecture

```text
PDF
 │
 ▼
PyMuPDF
 │
 ▼
Document
 └── Page
      └── Line
           └── Span
```

---

## Conversion Pipeline

```text
Legacy Text
      │
      ▼
Detect Font
      │
      ▼
Apply Rules
      │
      ▼
Apply Mappings
      │
      ▼
Fix Kombuva
      │
      ▼
Normalize Unicode
      │
      ▼
Final Sinhala Text
```

---

# 📦 Installation

## Clone Repository

```bash
git clone https://github.com/your-username/piliwela.git
cd piliwela
```

## Build Rust Extension

```bash
maturin develop
```

## Install Package

```bash
pip install -e .
```

---

# 🚀 Quick Start

## Read a PDF

```python
import piliwela

doc = piliwela.read("example.pdf")
```

---

## Extract Text

```python
text = doc.text()
```

---

## Access Pages

```python
for page in doc.pages():
    print(page.text())
```

---

## Access Lines

```python
for line in doc.lines():
    print(line.text())
```

---

## Access Spans

```python
for span in doc.spans():
    print(span.text)
```

---

## Extract Titles

```python
doc.titles()
doc.subtitles()
doc.subsubtitles()
```

---

## Create Chunks for RAG

```python
chunks = doc.chunks(
    type="paragraph",
    metadata=True
)
```

---

# 🧩 Document Model

```text
Document
└── Page
    └── Line
        └── Span
```

---

## Span Example

```python
Span(
    text="අධ්‍යාපන",
    raw_text="wOHdmk",
    font="FMAbhaya",
    font_family="FM",
    size=12,
    bbox=(x0, y0, x1, y1),
    converted=True,
)
```

---

# 📚 Example Chunk

```python
Chunk(
    text="ශ්‍රී ලංකා ජාතික ගීය...",
    metadata={
        "title": "අතීතයෙන් කතාවක්",
        "subtitle": None,
        "subsubtitle": None,
        "page_number": 1,
        "source_name": "සිංහල භාෂාව හා සාහිත්‍යය",
        "filename": "grade-10-sinhala.pdf"
    }
)
```

---

# 🛣 Roadmap

## ✅ Completed

* [x] PDF extraction
* [x] Layout reconstruction
* [x] Legacy font detection
* [x] Legacy → Unicode conversion
* [x] Document object model
* [x] Mixed Sinhala/English support

---

## 🚧 In Progress

* [ ] Heading extraction
* [ ] Paragraph reconstruction
* [ ] Table extraction
* [ ] Image extraction
* [ ] Multi-column support
* [ ] OCR support
* [ ] Chunk metadata engine

---

## 🔮 Future

* [ ] Scanned document understanding
* [ ] Search engine integration
* [ ] Vector database ingestion
* [ ] RAG pipeline
* [ ] Benchmark suite
* [ ] WebAssembly support

---

# 🎯 Why Piliwela?

Most document libraries work great for English but struggle with:

❌ Legacy Sinhala fonts
❌ Educational PDFs
❌ Government publications
❌ Historical archives
❌ Mixed Sinhala + English content

Piliwela aims to solve these problems with a production-ready toolkit built specifically for Sinhala document understanding.

---

# 🤝 Contributing

Contributions are welcome!

```bash
git checkout -b feature/amazing-feature
git commit -m "Add amazing feature"
git push origin feature/amazing-feature
```

Open a Pull Request 🚀

---

# 📄 License

Distributed under the MIT License.

See [`LICENSE`](LICENSE) for more information.

---

<div align="center">

### 🌾 Built for the Sinhala language community 🇱🇰

**Python × Rust × NLP × Document AI**

⭐ Star the project if you find it useful!

</div>
