package main

import (
	"fmt"
	"os"

	"github.com/jung-kurt/gofpdf"
)

func generatePDF(filename string, content string) error {
	pdf := gofpdf.New("P", "mm", "A4", "") // Create a new PDF instance
	pdf.AddPage()                           // Add a new page to the PDF

	pdf.SetFont("Arial", "B", 16) // Set font and size
	pdf.Cell(40, 10, "Entropy Analysis Results") // Write a title to the PDF

	pdf.SetFont("Arial", "", 12) // Set font and size
	pdf.MultiCell(0, 10, content, "", "", false) // Write the content to the PDF

	return pdf.OutputFileAndClose(filename) // Save the PDF to the specified filename
}

func generate() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: output <output_filename> <output_content>")
		return
	}

	outputFilename := os.Args[1]
	outputContent := os.Args[2]

	// Generate PDF file with output content
	if err := generatePDF(outputFilename, outputContent); err != nil {
		fmt.Printf("Error generating PDF: %v\n", err)
		return
	}

	fmt.Printf("Output saved to %s\n", outputFilename)
}
