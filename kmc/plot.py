import numpy as np
import matplotlib.pyplot as plt
import csv

x1 = [] * 100
y1 = [] * 100
x2 = [] * 100
y2 = [] * 100
x3 = [] * 100
y3 = [] * 100
x4 = [] * 100
y4 = [] * 100
with open('out.csv', newline='') as csvfile:
    spamreader = csv.reader(csvfile, delimiter=',', quotechar='|')
    count1 = 0
    count2 = 0
    count3 = 0
    count4 = 0
    for row in spamreader:
        if row[2] == "0" and count1 < 50:
            x1.append(float(row[0]))
            y1.append(float(row[1]))
            count1 += 1
        if row[2] == "1" and count2 < 50:
            x2.append(float(row[0]))
            y2.append(float(row[1]))
            count2 += 1
        if row[2] == "2" and count3 < 50:
            x3.append(float(row[0]))
            y3.append(float(row[1]))
            count3 += 1
        if row[2] == "3" and count4 < 50:
            x4.append(float(row[0]))
            y4.append(float(row[1]))
            count4 += 1

plt.scatter(x1,y1, c='red')
plt.scatter(x2,y2, c='blue')
plt.scatter(x3,y3, c='yellow')
plt.scatter(x4,y4, c='green')
plt.show()