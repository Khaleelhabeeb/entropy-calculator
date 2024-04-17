package main

import (
    "fmt"
    "math"
    "os"
    "path/filepath"
)

func bitCount(i uint8) uint8 {
    i = i - ((i >> 1) & 0x55)
    i = (i & 0x33) + ((i >> 2) & 0x33)
    return (i + (i >> 4)) & 0x0F
}

func main() {
    if len(os.Args) == 1 {
        fmt.Println("Provide one or more files")
        return
    }

    for _, filename := range os.Args[1:] {
        f, err := os.Open(filename)
        if err != nil {
            fmt.Printf("Error opening file %s: %v\n", filename, err)
            continue
        }
        defer f.Close()

        var (
            total uint64
            counts [256]uint32
        )

        buf := make([]byte, 256)
        for {
            n, err := f.Read(buf)
            if n == 0 || err != nil {
                break
            }

            for i := 0; i < n; i += 8 {
                for j := 0; j < 8 && i+j < n; j++ {
                    counts[buf[i+j]]++
                    total += 8
                }
            }

            for i := (n / 8) * 8; i < n; i++ {
                counts[buf[i]]++
                total++
            }
        }

        var entropy float64
        for _, count := range counts {
            if count > 0 {
                p := float64(count) / float64(total)
                entropy -= p * math.Log2(p)
            }
        }

        fmt.Printf("--- File: %s ---\n", filepath.Base(filename))
        fmt.Printf("Entropy per byte: %.2f bits or %.2f bytes\n", entropy, entropy/8)
        fmt.Printf("Entropy of file: %.2f bits or %.2f bytes\n", entropy*float64(total), entropy*float64(total)/8)
        fmt.Printf("Size of file: %d bytes\n", total)
        fmt.Printf("Delta: %d bytes compressable theoretically\n", total-uint64(entropy*float64(total)/8))
        fmt.Printf("Best Theoretical Coding ratio: %.2f\n", 8/entropy)
        fmt.Println()
    }
}