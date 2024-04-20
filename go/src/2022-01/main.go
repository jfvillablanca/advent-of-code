package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
)

const INPUT_FILENAME = "2022-01-input.txt"

func logic(scanner *bufio.Scanner) {
	outputArr := make([]int, 0)
	accumulator := 0

	for i := 0; scanner.Scan(); i++ {
		text := scanner.Text()
		if text == "" {
			outputArr = append(outputArr, accumulator)
			accumulator = 0
			continue
		}
		current, _ := strconv.Atoi(text)
		accumulator += current
	}
	fmt.Printf("Part 1 | Highest total num: %d\n", slices.Max(outputArr))
}

func main() {
	input_path := fmt.Sprintf("../shared/%s", INPUT_FILENAME)
	file, err := os.Open(input_path)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	logic(scanner)
}
