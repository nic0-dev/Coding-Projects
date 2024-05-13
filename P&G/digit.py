import itertools

def solve(exp, res):
    blanks = exp.count('_')
    digits = [1,2,3,4,5,6,7,8,9]
    # example: (_ * _ * _) - _ = 107
    for combi in itertools.combinations(digits, blanks):
        for permu in itertools.permutations(combi):
            temp = exp
            for elem in permu:
                temp = temp.replace('_', str(elem), 1)
            if eval(temp) == int(res): # return first possible answer
                return permu

if __name__=="__main__":
  while(True):
    print("Enter equation: ") # use */+- for operations
    inp = input()
    expression, result = inp.split('=')
    print(*solve(expression, result), '\n')