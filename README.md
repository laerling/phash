# Primitive Perceptual Hash

This is a very primitive perceptual hash implementation that satisfies my needs.
It simply converts each given image to grayscale and scales it to 4x4 pixels, then outputs each pixel value divided by 16 in hexadecimal notation, i.e. 16 hexadecimal digits.
This is stable enough for my purposes. But as you can see by running `./phash.py ./test/*`, it is not 100% reliable.
I for one have no problem with a small number of false positives, since I may want to deduplicate similar images anyway, even if they're not the exact same image.
