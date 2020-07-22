# rustymusic

![Rust](https://github.com/nvasudevan/rustymusic/workflows/Rust/badge.svg)

## Play compositions in Hindustani music

### Swar Notations

- shuddh swars are represented as `sa`, `re` and so on.
- komal (or flat) swars are prefixed by a dot (.), e.g. `.re`
- Tivra (or sharp) swars are shown with an apostrophe, e.g. `Ma'` 
- lower octave notes are prefixed by a dash (-), e.g. `-dha`
- higher octave notes are suffixed by a plus (+), e.g. `sa+`
- swars played for longer than a beat is shown as a dash, e.g. to play `re` for two beats, `re - `

### Usage

There are three options:

- play a raag (`-r <raag>`), currently raag `durga` and `bhupali` are supported
- play swars from a file (`-f <file>`). See files in `config` directory.
- play random swars (`-z <N>`), where N indicates number of random swars to play.


