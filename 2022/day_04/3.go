package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func first() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	res := 0
	for scanner.Scan() {
		line := scanner.Text()
		// split line by coma
		pair1, pair2 := strings.Split(line, ",")[0], strings.Split(line, ",")[1]
		// split by - and convert to int

		start1, err := strconv.Atoi(strings.Split(pair1, "-")[0])
		check(err)
		end1, err := strconv.Atoi(strings.Split(pair1, "-")[1])
		check(err)

		start2, err := strconv.Atoi(strings.Split(pair2, "-")[0])
		check(err)

		end2, err := strconv.Atoi(strings.Split(pair2, "-")[1])

		if start1 <= start2 && end1 >= end2 {
			res++
			continue
		}
		if start2 <= start1 && end2 >= end1 {
			res++
			continue
		}

	}
	fmt.Println(res)
	if err := scanner.Err(); err != nil {
		check(err)
	}

}
func checkOverlap(start1, end1, start2, end2 int) bool {
	if start1 <= start2 && start2 <= end1 {
		return true
	}
	if start2 <= start1 && start1 <= end2 {
		return true
	}
	return false
}

func second() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	res := 0
	for scanner.Scan() {
		line := scanner.Text()
		// split line by coma
		pair1, pair2 := strings.Split(line, ",")[0], strings.Split(line, ",")[1]
		// split by - and convert to int

		start1, err := strconv.Atoi(strings.Split(pair1, "-")[0])
		check(err)
		end1, err := strconv.Atoi(strings.Split(pair1, "-")[1])
		check(err)

		start2, err := strconv.Atoi(strings.Split(pair2, "-")[0])
		check(err)

		end2, err := strconv.Atoi(strings.Split(pair2, "-")[1])
		// check if ranges overlap
		if checkOverlap(start1, end1, start2, end2) {
			res++
		}

	}
	fmt.Println(res)
	if err := scanner.Err(); err != nil {
		check(err)
	}

}
func main() {
        first()
	second()
}
