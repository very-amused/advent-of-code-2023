package p2

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"time"
)

const inputFile = "input.txt"  // Change to input.txt for final solution
const ignoreEmptyLines = false // Don't parse blank lines

var numRegex = map[*regexp.Regexp]int{
	regexp.MustCompile("(zero|0)"):  0,
	regexp.MustCompile("(one)|1"):   1,
	regexp.MustCompile("(two)|2"):   2,
	regexp.MustCompile("(three)|3"): 3,
	regexp.MustCompile("(four)|4"):  4,
	regexp.MustCompile("(five)|5"):  5,
	regexp.MustCompile("(six)|6"):   6,
	regexp.MustCompile("(seven)|7"): 7,
	regexp.MustCompile("(eight)|8"): 8,
	regexp.MustCompile("(nine)|9"):  9,
}

func must[T any](v T, e error) T {
	if e != nil {
		panic(e)
	}
	return v
}

// #region Structs
type Input []int

// #endregion

// Parse
func parse() (input Input) {
	// Open scanner to read input line by line
	scanner := bufio.NewScanner(must(os.Open(inputFile)))

	// Parse lines
	for scanner.Scan() {
		l := scanner.Text()
		if len(l) == 0 {
			continue
		}
		// Get all regex matches
		var min, max int
		minIndex := -1
		maxIndex := -1
		for regex, value := range numRegex {
			matches := regex.FindAllStringIndex(l, -1)
			for _, match := range matches {
				if minIndex == -1 || match[0] < minIndex {
					minIndex = match[0]
					min = value
				}
				if maxIndex == -1 || match[1] > maxIndex {
					maxIndex = match[1]
					max = value
				}
			}
		}

		calibrationValue := must(strconv.Atoi(strconv.Itoa(min) + strconv.Itoa(max)))

		input = append(input, calibrationValue)
	}
	return input
}

// Solve
func solve(input Input) (solution string) {
	var sum int
	for _, v := range input {
		sum += v
	}
	return strconv.Itoa(sum)
}

func Part2() {
	// Parse
	input := parse()

	// Solve
	start := time.Now()
	solution := solve(input)

	// Report solve time and solution
	duration := time.Now().Sub(start)
	fmt.Printf("Solved in \x1b[34m%s\x1b[0m\n", duration)
	fmt.Printf("Solution: \x1b[32m%s\x1b[0m\n", solution)
}
