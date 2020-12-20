# artop

This CLI tool read information about your computer load at regular interval and, at the end, generate an abstract square PNG image with those data.

## Installation

There is no quick installation method for now, you need to clone this repository and use `cargo build --release`.

## Usage

Currently, artop only provides two options:

* `--output` for image file output path
* `--fill-method` to define how the image is constructed

You can see usage through `artop --help`.

### Filling methods

This program read a given number of values and use those to build the final image's diagonal (from top left to bottom right).
The filling method is the algorithm which calculates pixels colors in order to fill the entire image.

Current filling methods:

* `smooth`
* `gradient`

## License

Unless otherwise stated, all source code files found in this repository are under the GNU GPL 3.0 public license.
