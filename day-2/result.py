x = []
with open('input.txt', 'r') as f:
    x = [int(s.strip()) for s in f.readlines()]

print(len(x))
print(len(set(x)))

x_set = set(x)
sum = 2020

for i in range(0, len(x)):
    if (sum - x[i]) in x_set:
        print((sum - x[i]) * x[i])

