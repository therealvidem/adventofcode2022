import sys

if __name__ == '__main__':
    with open('input.txt', 'r') as f:
        total_calories = []
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
                total_calories.append(curr_total_calories)
                curr_total_calories = 0
            i += 1
        total_calories.sort()
        top3_sum = sum(total_calories[-3:])
        print(f'Total calories of top 3: {top3_sum}')
