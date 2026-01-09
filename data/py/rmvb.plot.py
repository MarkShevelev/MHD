import matplotlib.pyplot as plt
import numpy as np
import sys
import os

if len(sys.argv) < 2:
    print("""
          Script to plot a comparasing graphs:
            -- the first file is plot in black;
            -- the second is plot in red;
            -- the third is plot in blue;
            -- the last is green.

          Usage: python3 rmvb.cmp.plot <file1> [<file2> <file3> <file4>]
          """)
    sys.exit(1)

files = sys.argv[1:5]

fig, axs = plt.subplots(2, 2, figsize=(10, 8), layout='constrained', sharex=True)
fig.suptitle('rho mnt v bz')

plot_colors = ['k','r', 'b', 'g']

axs[0, 0].set_title('rho')
axs[0, 0].set_ylim(0.75, 2)

axs[0, 1].set_title('mnt')
axs[0, 1].set_ylim(-1, 1)

axs[1, 0].set_title('v (mnt/rho)')
axs[1, 0].set_ylim(-1, 1)

axs[1, 1].set_title('bz')

for i, filename in enumerate(files):
    parts = os.path.basename(filename).split(".")
    label = parts[-2] if len(parts) > 2 else filename

    # Load data for the current file
    h, rho, mnt, bz = np.loadtxt(filename, unpack=True, skiprows=1)
    v = mnt / rho
    
    color = plot_colors[i]
    
    # Plot current file's data into the corresponding pane
    # rho
    axs[0, 0].plot(h, rho, color=color, label=label)
    
    # mnt
    axs[0, 1].plot(h, mnt, color=color)
    
    # v (mnt/rho)
    axs[1, 0].plot(h, v, color=color)
    
    # bz
    axs[1, 1].plot(h, bz, color=color)

axs[0, 0].legend(fontsize='small', frameon=False)

plt.show()
