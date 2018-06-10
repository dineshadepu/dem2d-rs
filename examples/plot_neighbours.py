import numpy as np
import matplotlib.pyplot as plt


x = np.loadtxt("points.txt")
plt.scatter(x[:, 0], x[:, 1])
plt.show()
