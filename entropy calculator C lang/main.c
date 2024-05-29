#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <argp.h>

#define BUFFER_SIZE 256

const char *argp_program_version = "entropy_calculator 0.10";
const char *argp_program_bug_address = "<bug-gnu-utils@gnu.org>";
static char doc[] = "Entropy calculator for files, calculates either byte-level or bit-level entropy based on a command-line argument.";
static char args_doc[] = "FILE...";
static struct argp_option options[] = {
    {"bit", 'b', 0, 0, "Calculate bit-level informational entropy"},
    {0}
};

struct arguments {
    char **files;
    int bit_level;
};

static error_t parse_opt(int key, char *arg, struct argp_state *state) {
    struct arguments *arguments = state->input;
    switch (key) {
        case 'b':
            arguments->bit_level = 1;
            break;
        case ARGP_KEY_ARG:
            arguments->files = &state->argv[state->next - 1];
            state->next = state->argc;
            break;
        case ARGP_KEY_END:
            if (state->argc < 2) {
                argp_usage(state);
            }
            break;
        default:
            return ARGP_ERR_UNKNOWN;
    }
    return 0;
}

static struct argp argp = {options, parse_opt, args_doc, doc};

void calculate_entropy(const char *filename, int bit_level);

int main(int argc, char *argv[]) {
    struct arguments arguments;
    arguments.bit_level = 0;
    arguments.files = NULL;

    argp_parse(&argp, argc, argv, 0, 0, &arguments);

    for (int i = 0; arguments.files[i]; i++) {
        calculate_entropy(arguments.files[i], arguments.bit_level);
    }

    return 0;
}

void calculate_entropy(const char *filename, int bit_level) {
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

    if (bit_level) {
        uint32_t bitCounts[9] = {0}; // To count bytes with 0 to 8 bits set
        for (int i = 0; i < 256; i++) {
            int bitSum = 0;
            for (int j = 0; j < 8; j++) {
                bitSum += (i >> j) & 1;
            }
            bitCounts[bitSum] += counts[i];
        }

        double bitEntropy = 0.0;
        for (int i = 0; i < 9; i++) {
            if (bitCounts[i] > 0) {
                double prob = (double)bitCounts[i] / (totalBits / 8);
                bitEntropy -= prob * log2(prob);
            }
        }

        printf("--- File: %s ---\n", filename);
        printf("Bit-level informational entropy: %.6f bits\n", bitEntropy);
    } else {
        double entropy = 0.0;
        for (int i = 0; i < 256; i++) {
            if (counts[i] > 0) {
                double prob = (double)counts[i] / (totalBits / 8);
                entropy -= prob * log2(prob);
            }
        }

        double entropyPerByte = entropy / 8;
        double entropyOfFile = entropy * totalBits / 8;

        printf("--- File: %s ---\n", filename);
        printf("Entropy per byte: %.6f bits or %.6f bytes\n", entropy, entropyPerByte);
        printf("Entropy of file: %.6f bits or %.6f bytes\n", entropyOfFile, entropyOfFile / 8);
        printf("Size of file: %llu bytes\n", totalBits / 8);
        printf("Delta: %.6f bytes compressible theoretically\n", totalBits / 8 - entropyOfFile / 8);
        printf("Best Theoretical Coding ratio: %.6f\n", 8 / entropy);
    }
    printf("\n");
}