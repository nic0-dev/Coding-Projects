import sys

if __name__ == "__main__":
    sys.stdin = open("input.txt", "r")
    sys.stdout = open("output.txt", "w")

    choice = ['A. ', 'B. ', 'C. ', 'D. ']
    multiple = [8, 9, 14, 16, 18, 20, 21, 22, 23, 26, 41, 43, 48, 50]
    ctr = 0
    s = ' You may choose more than one answer.'
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
                n = line[0:2]
                if n[-1] == ".":
                    if int(n[0]) in multiple:
                        print('\n' + line[0:-1] + '?' + s)
                    else:
                        print('\n' + line[0:-1] + '?')
                elif int(n) in multiple:
                    print('\n' + line[0:-1] + '?', end = '')
                    print(s)
                else:
                    print('\n' + line[0:-1] + '?\n', end = '')
                # print(int(line[0:3]))
            ctr += 1