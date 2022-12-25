package main

import (
    "os"
    "bufio"
    "fmt"
    "strconv"
    "sort"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func main(){
    file, err := os.Open("./1_input.txt")
    check(err)
    defer file.Close()
    scanner := bufio.NewScanner(file)
    value := 0
    calories_arr := make([]int, 10)
    for scanner.Scan(){
        line := scanner.Text()
        if line ==""{
            calories_arr = append(calories_arr,value)
            value = 0
            continue
        }

        current_value, err := strconv.Atoi(line)
        value += current_value
        check(err)
    }
    if err := scanner.Err(); err != nil{
        check(err)
    }
    sort.Ints(calories_arr)

    res := 0
    for _, v := range calories_arr[len(calories_arr)-3:] {
        res += v
    }
    fmt.Println(res)
}
