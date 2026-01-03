import matplotlib.pyplot as plt
import numpy as np
import sys
if len(sys.argv) < 2:
    print("Usage: python script.py <filename>")
    sys.exit(1)

filename = sys.argv[1]
x, y = np.loadtxt(filename, unpack=True)

plt.plot(x, y)
plt.xlabel('Index')
plt.ylabel('Value')
plt.title('Data from test.dat')
plt.show()

