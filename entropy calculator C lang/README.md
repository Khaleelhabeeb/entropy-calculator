# Entropy Calculator
This program calculates the entropy of one or more files, offering statistics on the information content including entropy per byte, total entropy, file size, compressible bytes, and best theoretical coding ratio, while supporting multiple file analysis.

## How to run

Make sure you have C compiller installed on your system.

### To run a single file

#### compile
```
gcc -o main main.c -lm
```
#### run
```
./main file1 ...

```

### To run multiple files

```
./ file1 file2 ...

```