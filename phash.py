#!/usr/bin/env python3

import sys
from PIL import Image, UnidentifiedImageError

if len(sys.argv) <= 1:
    print("Too few arguments. Expect image file")
    sys.exit(1)

# We only have a "rough look" at the picture to see whether two pictures look
# roughly the same. Therefore it's recommended to use this script in
# conjunction with a conventional hash, because this script only shows very
# rough differences, while a hash shows whether two pictures differ even in
# just a small detail.
for f in sys.argv[1:]:

    # load image, silently ignoring non-image files
    try:
        img = Image.open(f)
    except UnidentifiedImageError:
        continue
    (width, height) = img.size

    # filter for resizing
    # TODO: Which filter gives best result?
    #       BOX, BILINEA, HAMMING, BICUBIC, LANCZOS
    resampling_filter = Image.Resampling.LANCZOS

    # sampling resolution
    for depth_p in range(2,3+1):
        depth = 2**depth_p

        for dim in range(2,3+1):
            img_ = img.resize((dim,dim), resampling_filter)
            px = img_.load()

            # iterate over pixels
            for i in range(dim):
                for j in range(dim):
                    p = px[i,j]
                    pv = 0
                    if type(p) == tuple:
                        for c in range(3):
                            pv += p[c]
                        pv //= 3
                    else:
                        pv = p
                    print(f"{pv // (256 // depth):1x}",end='')

    # print filename
    print(f"  {f}")
