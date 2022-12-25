package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

const LOST int = 0
const DRAW int = 3
const WIN int = 6

func first() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		ff := strings.Fields(scanner.Text())
		opponent := int(ff[0][0] - 'A')
		myPlay := int(ff[1][0] - 'X')
		score += myPlay + 1
		if myPlay == opponent {
			score += DRAW
			continue
		} else if myPlay == (opponent+1)%3 {
			score += WIN
		}
	}
	fmt.Println(score)
	if err := scanner.Err(); err != nil {
		check(err)
	}

}

func second() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	score := 0
	for scanner.Scan() {
		ff := strings.Fields(scanner.Text())
		opponent := int(ff[0][0] - 'A')
		myPlayDirection := int(ff[1][0] - 'X')
                myPlay := opponent
                // 0 0 -> 2 | 3
                // 0 1 -> 0 | 4
                // 0 2 -> 1 | 8

                // 1 0 -> 0 | 1
                // 1 1 -> 1 | 5
                // 1 2 -> 2 | 9

                // 2 0 -> 1 | 2
                // 2 1 -> 2 | 6
                // 2 2 -> 0 | 10

                // 0 0 -> 2 | 3
                // 1 0 -> 0 | 1
                // 2 0 -> 1 | 2

                // 0 1 -> 0 | 4
                // 1 1 -> 1 | 5
                // 2 1 -> 2 | 6

                // 0 2 -> 1 | 8
                // 1 2 -> 2 | 9
                // 2 2 -> 0 | 10


                if myPlayDirection == 0 {
                    myPlay = (opponent + 2) % 3
                } else if myPlayDirection == 2 {
                    myPlay = (opponent + 1) % 3

                }
                //fmt.Println(opponent, myPlayDirection, myPlay)
                score += myPlay + 1
		if myPlay == opponent {
			score += DRAW
			continue
		} else if myPlay == (opponent+1)%3 {
			score += WIN
		}
	}
	fmt.Println(score)
	if err := scanner.Err(); err != nil {
		check(err)
	}

}

func main() {
	first()
	second()
}
