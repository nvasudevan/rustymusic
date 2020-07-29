# rustymusic

![Rust](https://github.com/nvasudevan/rustymusic/workflows/Rust/badge.svg)

## Play compositions in Hindustani music

### Demo
Play Raag Yaman [MP3](https://raw.githubusercontent.com/nvasudevan/rustymusic/master/demo/demo.mp3)
![Playing Raag Yaman](demo/demo.gif)


### Swar Notations

- `shuddh` (or pure) swars are represented by uppercase letters, so `S`, `R` and so on.
- `komal` (or flat) swars are represented by lowercase, so `r` is komal `re`.
- `tivra` (or sharp) swars are shown with an apostrophe, e.g. `M'`
- `mandra` (or lower octave) saptak swars are prefixed by a dot, e.g. `.D`. 
  A mandra komal swar is then prefixed by `.`, e.g. `.n` for lower octave koman `Ni`.
- `taar` (or higher octave) saptak swars are suffixed by a dot, e.g. `R.`
- swars played for longer than a `matra` (or beat) is ornamented with a dash, e.g. `R - ` indicates playing
  of the swar `re` for two matras.
- two swars sharing a matra is split by a `:`, e.g. `S:R` indicates `sa` and `re` are each played for
  half a matra. A swar played for half a matra is shown as `S:`. 

Many of the above notations should already be familiar to classical Hindustani musicians,
some I have had to change for ease of use as `strings` within my program.

All the compositions cited in this repository have been provided by my
Guruji _Pandit Vijay Jagtap_ (https://www.vijayjagtap.com). Without his invaluable guidance,
this implementation wouldn't have been possible.

### Usage

There are three options:

- play a raag (`-r <raag>`), currently raag `bhupali`, `durga`, `yaman` and `hamsadhwani` are supported
- play swars from a file (`-f <file>`). See files in `config` directory.
- play random swars (`-z <N>`), where N indicates number of random swars to play. This option is useful for vocal exercise.

