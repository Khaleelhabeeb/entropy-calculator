package main

import (
	"flag"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"strings"
)

func bitCount(i uint8) uint8 {
	i = i - ((i >> 1) & 0x55)
	i = (i & 0x33) + ((i >> 2) & 0x33)
	return (i + (i >> 4)) & 0x0F
}

func calculateBitLevelEntropy(counts [256]uint32, total uint64) float64 {
	var bitCounts [9]uint32 // 0 to 8 bits
	for byteValue, count := range counts {
		bitSum := bitCount(uint8(byteValue))
		bitCounts[bitSum] += count
	}

	var bitEntropySum float64
	for _, count := range bitCounts {
		if count > 0 {
			p := float64(count) / float64(total)
			bitEntropySum -= p * math.Log2(p)
		}
	}

	return bitEntropySum
}

func main() {
	bitEntropy := flag.Bool("b", false, "Calculate bit-level informational entropy")
	flag.BoolVar(bitEntropy, "bit", false, "Calculate bit-level informational entropy")
	flag.Parse()

	if len(flag.Args()) == 0 {
		fmt.Println("Provide one or more files")
		return
	}

	var output strings.Builder // Create a string builder to collect output

	for _, filename := range flag.Args() {
		f, err := os.Open(filename)
		if err != nil {
			output.WriteString(fmt.Sprintf("Error opening file %s: %v\n", filename, err))
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

		var byteEntropy float64
		for _, count := range counts {
			if count > 0 {
				p := float64(count) / float64(total)
				byteEntropy -= p * math.Log2(p)
			}
		}

		output.WriteString(fmt.Sprintf("--- File: %s ---\n", filepath.Base(filename)))
		output.WriteString(fmt.Sprintf("Entropy per byte: %.6f bits or %.6f bytes\n", byteEntropy, byteEntropy/8))
		output.WriteString(fmt.Sprintf("Entropy of file: %.6f bits or %.6f bytes\n", byteEntropy*float64(total), byteEntropy*float64(total)/8))
		output.WriteString(fmt.Sprintf("Size of file: %d bytes\n", total))
		output.WriteString(fmt.Sprintf("Delta: %.6f bytes compressible theoretically\n", float64(total)-(byteEntropy*float64(total)/8)))
		output.WriteString(fmt.Sprintf("Best Theoretical Coding ratio: %.6f\n", 8/byteEntropy))

		if *bitEntropy {
			bitLevelEntropy := calculateBitLevelEntropy(counts, total)
			output.WriteString(fmt.Sprintf("Informational entropy per bit: %.6f bits\n", bitLevelEntropy))
			output.WriteString(fmt.Sprintf("Entropy per byte (bit-level): %.6f bits\n", bitLevelEntropy*8))
			output.WriteString(fmt.Sprintf("Entropy of entire file (bit-level): %.6f bits\n", bitLevelEntropy*float64(total)))
		}

		output.WriteString("\n")
	}

	// Print the output content
	fmt.Print(output.String())
}
