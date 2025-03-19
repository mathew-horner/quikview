# `quikview`

`quikview` is an image viewer for the [Netpbm Portable PixMap](https://en.wikipedia.org/wiki/Netpbm) image file format.

## What is a `ppm` image file?

`ppm` is an image file format used by the Netpbm graphics suite.

The two main reasons I like to use it (in certain situations) are:

1. It is a very simple format that is easy to understand for both humans and computers.
2. You can encode your image data in plain text, which makes it easy to edit simple images with a text editor.

## How does `quikview` work?

`quikview` uses my `quikpix` PPM library to read the input file and parse it into a pixel grid and
renders to a window using the `winit` and `pixels` libraries.

## Usage

```text
cd quikview
cargo run -- [-s <SCALE>] <FILE>
```
