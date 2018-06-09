import numpy as np
import matplotlib.pyplot as plt


x = np.loadtxt("points.txt")
plt.scatter(x[0][0], x[0][1], c="red")
plt.scatter(x[1:][0], x[1:][1], c="green")
plt.show()
