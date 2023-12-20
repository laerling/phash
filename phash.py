#!/usr/bin/env python3

import sys
from PIL import Image, UnidentifiedImageError

TO_DIM = 4

if len(sys.argv) <= 1:
    print("Too few arguments. Expect image file")
    sys.exit(1)

for f in sys.argv[1:]:

    # load image, silently ignoring non-image files
    try:
        i = Image.open(f)
    except UnidentifiedImageError:
        continue

    # convert to grayscale, since we're not interested in color shifts
    i = i.convert('L')

    # resize
    # TODO: Which filter gives best result?
    #       BOX, BILINEA, HAMMING, BICUBIC, LANCZOS
    resampling_filter = Image.Resampling.LANCZOS
    i = i.resize((TO_DIM,TO_DIM), resampling_filter)

    # FIXME make bit array and print as bytes (e. g. 4*256/16 would be 4 bytes,
    # but 4*256/8 would be 5)
    px = i.load()
    for i in range(TO_DIM):
        for j in range(TO_DIM):
            print(f"{px[i,j]//16:1x}",end='')

    # print filename
    print(f" {f}")
