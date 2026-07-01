import piliwela

doc = piliwela.read("/home/naviya-c/Downloads/grade-10-mathematic.pdf")

with open(
    "output4.txt",
    "w",
    encoding="utf-8",
) as f:
    f.write(doc.text())

print("Saved to output.txt")