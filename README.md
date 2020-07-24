# rustymusic

![Rust](https://github.com/nvasudevan/rustymusic/workflows/Rust/badge.svg)

## Play compositions in Hindustani music

### Demo

![Playing Raag Yaman](demo/demo.gif)

### Swar Notations

- `shuddh` (or pure) swars are represented as `sa`, `re` and so on.
- `komal` (or flat) swars are prefixed by a dot, e.g. `.re`
- `tivra` (or sharp) swars are shown with an apostrophe, e.g. `Ma'`
- `mandra` (or lower octave) saptak swars are prefixed by an underscore, e.g. `_dha`. A mandra komal swar is then prefixed by `_.`, e.g. `_.Ni`
- `taar` (or higher octave) saptak swars are suffixed by an underscore, e.g. `sa_`
- swars played for longer than a `matra` (or beat) is ornamented with a dash, e.g. `re - ` indicates playing of swar `re` for two matras.

### Usage

There are three options:

- play a raag (`-r <raag>`), currently raag `bhupali`, `durga`, `yaman` and `hamsadhwani` are supported
- play swars from a file (`-f <file>`). See files in `config` directory.
- play random swars (`-z <N>`), where N indicates number of random swars to play. This option is useful for vocal exercise.

