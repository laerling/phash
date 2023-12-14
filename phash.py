#!/usr/bin/env python3

import sys
from PIL import Image

TO_DIM = 4

if len(sys.argv) <= 1:
    print("Too few arguments. Expect image file")
    sys.exit(1)

for f in sys.argv[1:]:

    # load and convert to grayscale, since we're not interested in color shifts
    i = Image.open(f).convert('L')
    i = i.resize((TO_DIM,TO_DIM), Image.Resampling.BOX)

    # FIXME make bit array and print as bytes (e. g. 4*256/16 would be 4 bytes,
    # but 4*256/8 would be 5)
    px = i.load()
    for i in range(TO_DIM):
        for j in range(TO_DIM):
            print(f"{px[i,j]//16:1x}",end='')

    # print filename
    print(f" {f}")
