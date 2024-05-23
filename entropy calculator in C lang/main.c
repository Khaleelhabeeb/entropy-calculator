#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define BUFFER_SIZE 256

int bit_count(uint8_t byte) {
    int count = 0;
    while (byte) {
        count += byte & 1;
        byte >>= 1;
    }
    return count;
}

void calculate_entropy(const char *filename) {
    FILE *file = fopen(filename, "rb");
    if (!file) {
        fprintf(stderr, "Error opening file: %s\n", filename);
        return;
    }

    uint32_t counts[256] = {0};
    uint8_t buffer[BUFFER_SIZE];
    size_t bytesRead;
    uint64_t totalBits = 0;

    while ((bytesRead = fread(buffer, 1, BUFFER_SIZE, file)) > 0) {
        for (size_t i = 0; i < bytesRead; i++) {
            counts[buffer[i]]++;
            totalBits += 8;
        }
    }

    fclose(file);

    double entropy = 0.0;
    for (int i = 0; i < 256; i++) {
        if (counts[i] > 0) {
            double prob = (double)counts[i] / totalBits * 8;
            entropy -= prob * log2(prob);
        }
    }

    double entropyPerByte = entropy / 8;
    double entropyOfFile = entropy * totalBits / 8;

    printf("--- File: %s ---\n", filename);
    printf("Entropy per byte: %.6f bits or %.6f bytes\n", entropy, entropyPerByte);
    printf("Entropy of file: %.6f bits or %.6f bytes\n", entropyOfFile, entropyOfFile / 8);
    printf("Size of file: %llu bytes\n", totalBits / 8);
    printf("Delta: %.6f bytes compressable theoretically\n", totalBits / 8 - entropyOfFile / 8);
    printf("Best Theoretical Coding ratio: %.6f\n", 8 / entropy);
    printf("\n");
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Provide one or more files\n");
        return 1;
    }

    for (int i = 1; i < argc; i++) {
        calculate_entropy(argv[i]);
    }

    return 0;
}