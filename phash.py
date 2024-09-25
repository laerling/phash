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
        img = Image.open(f)
    except UnidentifiedImageError:
        continue

    # TODO: Convert alpha to white instead of black, since that's how I want alpha to be displayed

    # TODO: Increase contrast to have a unified dynamic range (or whatever it's
    #       called) across all images and to distinguish between different
    #       details in images with low contrast

    # convert to grayscale, since we're not interested in color shifts
    img = img.convert('L')

    # inner square (i. e. using n**2 of (n+2)**2 pixels) with n going from 1 to TO_DIM
    for n in range(1, TO_DIM+1):

        # resize
        # TODO: Which filter gives best result?
        #       BOX, BILINEA, HAMMING, BICUBIC, LANCZOS
        resampling_filter = Image.Resampling.LANCZOS
        img_ = img.resize((n+2,n+2), resampling_filter)

        # FIXME make bit array and print as bytes (e. g. 4*256/16 would be 4 bytes,
        # but 4*256/8 would be 5)
        px = img_.load()
        for i in range(1, n+1):
            for j in range(1, n+1):
                print(f"{px[i,j]//16:1x}", end='')
        print(end=' ')

    # print filename
    print(f" {f}")
