# Mandelbrot-rs

Mandelbrot-rs is an real-time renderer of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set).
With it you can explore this amazing frctal and watch the many shapes it contains, unfortunately it's not able to deep zoom at high levels
because it uses only double precision floating point numbers for the computation.
However the calculations are all made on the GPU which allows it to run in real time.
The program uses the [escape time algorithm](https://en.wikipedia.org/wiki/Mandelbrot_set#Escape_time_algorithm)
to check for each pixel if it is part of the set or not thus, to reduce the lag,
you can resize the window to make the number of pixels to compute smaller.

Others sources:
- [Faster fractals through algebra](https://randomascii.wordpress.com/2011/08/13/faster-fractals-through-algebra/) article by Bruce Dawson
- [Mu-Ency - The Encyclopedia of the Mandelbrot Set](https://mrob.com/pub/muency.html) by Robert Munafo

# How to use it

- To zoom in and out use to mouse wheel
- To pan around drag with the left mouse button
- To increase or decrease the number of iterations(level of detail) use the + and - on the keypad

If you hold CTRL when pressing + or - the number of iterations will be incremented or decremented by 100 instead of 1.

# Dependecies

The program uses [sdl2-rs]() for creating the window and managing the user input
