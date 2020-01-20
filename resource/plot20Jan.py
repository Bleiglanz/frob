import numpy as np

import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.cbook as cbook
import locale

plt.rcParams.update({
    "pgf.texsystem": "pdflatex",
    "pgf.preamble": [
         r"\usepackage[utf8x]{inputenc}",
         r"\usepackage[T1]{fontenc}",
         r"\usepackage{cmbright}",
         ]
})


dateiname = "data3.csv"
bilddatei = "data.pdf"

data = np.genfromtxt(dateiname, delimiter=';',skip_header=1,
                     dtype=[('s0','f8'),('s1','f8'),('s2','f8'),('s3','f8'),('pn','f8'),('pnk','f8'),
                            ('s6','f8'),('s7','f8'),('s8','f8'),('fS','f8'),])

plt.tight_layout()
plt.text(0.5, 3., "serif", family="serif")
plt.text(0.5, 2., "monospace", family="monospace")
plt.text(2.5, 2., "sans-serif", family="sans-serif")

fig = plt.figure()
ax3 = fig.add_subplot(111)
ax3.set_title("The quotient f/p as a function of $p_{max}/p$")
ax3.set_xlabel('max prime/prime')
ax3.set_ylabel('$f/p$')
xdata = data['pnk']/data['pn']
ydata = data['fS'] /data['pn']
ax3.scatter(xdata,ydata, marker=".", color='b')

for k in range(2,20,1):
   mark = (2+k)/k
   x1, y1 = [mark,mark], [0,20]
   plt.plot(x1, y1, marker = '+')


plt.show()
plt.savefig(bilddatei)



