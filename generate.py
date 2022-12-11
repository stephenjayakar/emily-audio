vals = []
for i in range(48000):
    x = i % 250
    if x < 50:
        vals.append(1)
    elif x < 100:
        vals.append(-1)
    elif x < 250:
        vals.append(0)
print(vals)
