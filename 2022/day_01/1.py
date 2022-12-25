with open("././1_input.txt") as f:
    value = 0
    max_value = 0
    for line in f:
        if line == "\n":
            max_value = max(value, max_value)
            value = 0
            continue
        value += int(line)
    print(max_value)
