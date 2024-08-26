import matplotlib.pyplot as plt
import numpy as np

plt.style.use('_mpl-gallery')

# make data
vus = [10,30,50,100]
y_actix = [5.72, 6.64, 12.39, 43.08]
y_spring = [10.56, 16.52, 26.34, 96.53]

# plot
fig, ax = plt.subplots(1, 1, layout='constrained')

plt.title(f'Request Times over VUs')
ax.plot(vus, y_actix, 'o-', markeredgewidth=2, color='indigo', label='Actix')
ax.plot(vus, y_spring, 'o-', markeredgewidth=2, color='lightgreen', label='Spring')
ax.set_ylabel("ms")
ax.set_xlabel("VUs")
plt.legend()


ax.set(xlim=(-10, 120),
       ylim=(0, 140))

plt.show()