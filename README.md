# Entropy Calculator
This program calculates the entropy of one or more files, offering statistics on the information content including entropy per byte, total entropy, file size, compressible bytes, and best theoretical coding ratio, while supporting multiple file analysis.

## How to run

Make sure you have Go installed on your system.

### To run a single file

```
go run main.go file1.txt
```

### To run multiple files

```
go run main.go file1.txt file2.txt file3.txt
```

## To calculate bit-level informational entropy in addition to byte-level entropy

### Single File with --bit flag:

```
go run main.go --bit file1.txt
```

### Multiple Files with --bit flag

```
go run main.go --bit file1.txt file2.txt file3.txt
```