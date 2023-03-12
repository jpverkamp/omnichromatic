# omnichromatic

A follow up to my old blog post in Racket: [Generating omnichromatic images](https://blog.jverkamp.com/2015/01/01/generating-omnichromatic-images/)

The goal is to generate images that do not contain the same RGB color more than once. So for a full 4096x4096 image, every single 8-bit RGB color is in the image exactly once. 

That that end, I'm writing a bunch of color producers (that will generate colors in a specific order) and order producers (that will produce x/y points) and then pair them up to generate images. 

More to come!
