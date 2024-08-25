import matplotlib.pyplot as plt
import numpy as np

plt.style.use('_mpl-gallery')

# make data
x = [10,30,50,100]
y_actix = [7.15, 13.32, 44.94, 108.95]
y_spring = [10.56, 16.52, 26.34, 96.53]

# plot
fig, ax = plt.subplots(1, 1, layout='constrained')

plt.title(f'Request Times over VUs')
ax.plot(x, y_actix, 'o-', markeredgewidth=2, color='indigo', label='Actix')
ax.plot(x, y_spring, 'o-', markeredgewidth=2, color='lightgreen', label='Spring')
ax.set_ylabel("ms")
ax.set_xlabel("VUs")
plt.legend()


ax.set(xlim=(-10, 120),
       ylim=(0, 140))

plt.show()