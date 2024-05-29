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
./main file1 

```

### To run multiple files

```
./ main file1 file2 ...

```

### To calculate bit informational entropy of a single file
```
./ main --b file1
```

#### To calculate bit informational entropy of a multiple files
```
./ main -b file1 file2 ...