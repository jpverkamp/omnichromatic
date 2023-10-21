Create an omnichromatic image: an image that (if large enough) contains every 8bit RGB color exactly once. 

# Usage

The specific subcommands that can be run:

```
Usage: omnichromatic [OPTIONS] --color-provider <color-provider> --point-provider <point-provider> <COMMAND>

Commands:
  render   Render a single image
  animate  Render an animation of the image being built
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --debug
  -x, --width <width>                    Width of the generated image [default: 640]
  -y, --height <height>                  Height of the generated image [default: 480]
  -c, --color-provider <color-provider>  Provide the order of RGB colors
  -p, --point-provider <point-provider>  Provide the order of XY points
  -o, --output <output>                  Output file name [default: omnichromatic.png]
  -h, --help                             Print help
  -V, --version                          Print version
```

# Examples

```
./omnichromatic \
  --width 1024 --height 768 \
  --color-provider target/release/libcolor_ordered.dylib \
  --point-provider target/release/libwriting.dylib \
  render
```