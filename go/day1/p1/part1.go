package p1

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

const inputFile = "input.txt"  // Change to input.txt for final solution
const ignoreEmptyLines = false // Don't parse blank lines

func must[T any](v T, e error) T {
	if e != nil {
		panic(e)
	}
	return v
}

// #region Structs
type Input []int

// #endregion

func isNum(s string) bool {
	return s[0] >= '0' && s[0] <= '9'
}

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
		var nums []int
		chars := strings.Split(l, "")
		for _, c := range chars {
			if isNum(c) {
				nums = append(nums, must(strconv.Atoi(c)))
			}
		}

		calibrationValue := must(strconv.Atoi(strconv.Itoa(nums[0]) + strconv.Itoa(nums[len(nums)-1])))

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

func Part1() {
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
