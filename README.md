# Estimate entropy

This small program is designed to estimate entropy available from the system
timer.

Note: all observations apply only to a small sample of systems, namely two
Intel Haswell-generation Linux 4.x machines.

By plotting a histogram of the low bits of each call it can be observed that
numbers are well distributed — there is no obvious bias at any level.

By plotting a histogram of the differences between calls, it immediately becomes
obvious that the time of each cycle of the loop is nearly constant, however
not exactly constant.

By plotting a histogram of the low bits of the differences (and varying the
number of bits), it can be seen that there is bias in the differences at
anything above the lowest bit, but in total there appears to be roughly 2-3 bits
of entropy per call.

### Usage

Tweak the three constants, and toggle which graphs are plotted. Make sure
gnuplot is installed, then:

> cargo run
