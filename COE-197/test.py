import sys

if __name__ == "__main__":
    sys.stdin = open("input.txt", "r")
    sys.stdout = open("output.txt", "w")

    choice = ['A. ', 'B. ', 'C. ', 'D. ']
    ctr = 0
    for line in sys.stdin:
        
        if line[0] == "*":
            continue
        elif line[2:8] == "points":
            ctr = 0
            continue
        else: 
            if ctr < 4: 
                print(choice[ctr] + line, end = '')
            else:
                print('\n' + line, end = '')
            ctr += 1