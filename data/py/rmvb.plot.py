import matplotlib.pyplot as plt
import numpy as np
import sys
if len(sys.argv) < 2:
    print("Usage: python script.py <filename>")
    sys.exit(1)

filename = sys.argv[1]
h, rho, mnt, bz = np.loadtxt(filename, unpack=True, skiprows=1)
v = mnt / rho

fig, axs = plt.subplots(2, 2, figsize=(8, 6))
fig.suptitle('Four Panes Example') # Add a overall title

# Top-left pane (row 0, column 0)
axs[0, 0].plot(h, rho, 'tab:blue')
axs[0, 0].set_title('rho')
axs[0, 0].set_ylim(0, 2)

# Top-right pane (row 0, column 1)
axs[0, 1].plot(h, mnt, 'tab:blue')
axs[0, 1].set_title('mnt')
axs[0, 1].set_ylim(-1, 1)

# Bottom-left pane (row 1, column 0)
axs[1, 0].plot(h, v, 'tab:blue')
axs[1, 0].set_title('v')
axs[1, 0].set_ylim(-1, 1)

# Bottom-right pane (row 1, column 1)
axs[1, 1].plot(h, bz, 'tab:blue')
axs[1, 1].set_title('bz')

# 3. Adjust layout to prevent labels from overlapping
plt.tight_layout()

plt.show()
