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
dateiname = "waring_2.csv"

dataload  = np.genfromtxt(dateiname, delimiter=';',skip_header=1,
                     dtype=[('begin_slice','int'),('s3','f8'),('pn','int'),('pnk','f8'),
                            ('s6','f8'),('s7','f8'),('s8','f8'),('fS','f8'),])

primes = dataload['pn']
framecount=0
for s in range(5,422):
   print(s)
   filtered = dataload['begin_slice']==s
   data = dataload[filtered]
   p = data['pn'][0]

   plt.tight_layout()
   plt.text(0.5, 3., "serif", family="serif")
   plt.text(0.5, 2., "monospace", family="monospace")
   plt.text(2.5, 2., "sans-serif", family="sans-serif")
   plt.xlim(right=4)

   fig = plt.figure()
   ax3 = fig.add_subplot(111)
   ax3.set_title("Squares: The quotient $f_\lambda({0})$ /  ${0}$ by  $(1+\lambda)$".format(p))
   ax3.set_xlabel('$(1+\lambda)$')
   ax3.set_ylabel("$f_\lambda({0})/{0}$".format(p)) 
   xdata = data['pnk']/data['pn']
   ydata = data['fS'] /data['pn']
   ax3.scatter(xdata,ydata, marker=".", color='r')
   ax3.plot(4,0,'')
   plt.xticks(np.arange(0, 10, 1.0))
   plt.yticks(np.arange(0, 25, 1.0))
   plt.grid(color='whitesmoke', linestyle='-', linewidth=0.5)
   #for k in range(2,20,1):
   #   mark = (2+k)/k
   #   x1, y1 = [mark,mark], [k+2,k+2]
   #   #x1, y1 = [mark,mark], [0,k+2]
   #   ax3.plot(x1, y1, marker = '+', color='grey')

   ##fig.show()
   framecount += 1
   bilddatei = "out/war/frame{:04}.png".format(framecount)
   plt.savefig(bilddatei,dpi=600)
   plt.close()

#ffmpeg -r 60 -f image2 -s 1920x1080 -start_number 1 -i ./out/frame%04d.png -vframes 1000 -vcodec libx264 -crf 25  -pix_fmt yuv420p test.mp4

