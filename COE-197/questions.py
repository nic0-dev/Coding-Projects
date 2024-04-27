import random

base_values = [66.750285, 63.806774, 60.428229, 70.760730, 58.192423, 59.274042]
random_values = {value: [random.uniform(value - 10, value + 10) for _ in range(3)] for value in base_values}

k = list([random.uniform(value - 10, value + 10) for _ in range(3)] for value in base_values)
for i in k:
    for j in i:
        print("%.6f" % j)
# print(random_values*)
