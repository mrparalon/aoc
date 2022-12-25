package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func priority_from_char(s rune) int {
	if s >= 97 {
		return int(s) - 97 + 1
	} else {
		return int(s) - 65 + 26 + 1
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
		left := make(map[int]bool)
		for i, s := range line {
			if i < len(line)/2 {
				left[priority_from_char(s)] = true
			} else {
				if left[priority_from_char(s)] {
					res += priority_from_char(s)
					break
				}
			}

		}
	}
	fmt.Println(res)
	if err := scanner.Err(); err != nil {
		check(err)
	}

}

func second() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	res := 0
	scanNotDone := true
	for scanNotDone {
		fmt.Println(scanNotDone)
		scanNotDone = scanner.Scan()
		line1 := scanner.Text()

		scanNotDone = scanner.Scan()
		line2 := scanner.Text()

		scanNotDone = scanner.Scan()
		line3 := scanner.Text()

		map1, map2 := make(map[rune]bool), make(map[rune]bool)
		for _, s := range line1 {
			map1[s] = true
		}
		for _, s := range line2 {
			map2[s] = true
		}
		for _, s := range line3 {
			if map1[s] && map2[s] {
				res += priority_from_char(s)
				break
			}
		}
	}
	fmt.Println(res)
	if err := scanner.Err(); err != nil {
		check(err)
	}
}

func main() {
	//first()
	second()
}
