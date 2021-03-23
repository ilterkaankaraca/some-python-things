import os
from os import listdir
from os.path import isfile, join
onlyfiles = [f for f in listdir() if isfile(join(f))]
for i in onlyfiles:
    if os.path.splitext(i)[1] != '.py':
        base = os.path.splitext(i)[0]
        os.rename(i, base + '.sql')