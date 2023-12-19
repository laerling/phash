# Crop-invariant Perceptual Hash

My goal is to create a crop-invariant perceptual hash function.
Of course crop-invariant perceptual hashes will not be the same between two pictures when one is cropped. However, similarity should be detectable!


# Idea

The idea is to treat an image as a superposition of wave functions (basically a fourrier transform), but in a way that allows finding out whether one wave function superposition is part of another
- without the hashes being different lengths, if possible (though image size correlation might be acceptable in a PoC)


# Previous work

Big TODO! I still have to research whether there is any previous work on crop invariance in perceptual hash algorithms.
