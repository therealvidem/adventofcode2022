import sys

if __name__ == '__main__':
    with open('input.txt', 'r') as f:
        max_calories = 0
        curr_total_calories = 0
        i = 1
        for raw_line in f:
            line = raw_line.strip()
            if len(line) > 0:
                try:
                    num = int(line)
                    curr_total_calories += num
                except ValueError:
                    print(f'Line {i} does not have a valid number', file=sys.stderr) 
            else:
                max_calories = max(curr_total_calories, max_calories)
                curr_total_calories = 0
            i += 1
        print(f'Total calories of max: {max_calories}')
