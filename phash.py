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

    # TODO
    # for dim in range(1,4+1):
    #  for color in range(2):
    #   for depth_p in range(1,4+1):

    # PHASE 1: Overall color (1x1 pixel) and rough color distribution (2x2, 3x3, ...).
    # TODO better to do subphases for 1) lightness/luminosity 2) mean color (over all channels) 3) mean color per channel
    #      Means can probably be calculated in one iteration, without having to convert the image to grayscale first.
    # TODO Calculate profile of normalized deviation from the black-white axis within the RGB cube?
    max_dim = 3
    printed = 0
    for n in range(1, max_dim+1):
        img_ = img.resize((n,n), resampling_filter)
        px = img_.load()
        #print(f"n=={n}")
        for i in range(0, n):
            for j in range(0, n):
                #print(f"{i} {j}:")
                p_avg = 0
                # FIXME treat alpha channel as letting through white, not black
                # RGB channels
                for c in range(3):
                    #print(f"{px[i,j][c]//16:1x}", end='')  # 2 1x1 to 3x3
                    # FIXME if the picture is grayscale px[i,j] is int
                    #p_avg += px[i,j][c]
                p_avg //= 3
                print(f"{p_avg//16:1x}",end='')  # 3 1x1 to 3x3 channel avg
                printed += 1
        print(end=' ')

    # remaining amount of digits to reach a power of two
    p = 1
    truehash_len = 2**p
    min_th_len = 8
    while truehash_len < printed + min_th_len:
        p += 1
        truehash_len = 2**p
    truehash_len -= printed
    truehash_len //= 2

    # print filename
    print(f"  {f}")
