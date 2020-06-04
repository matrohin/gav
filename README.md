# Geometry Algorithms Visualization (gav)

This application demonstrates some of the classic computational geometry algorithms.

## Building

To buid gav, you need the [latest stable version of Rust](https://www.rust-lang.org/tools/install).

Then just clone and use `cargo`:
```
git clone https://github.com/matrohin/gav.git
cd gav/
cargo build --release
```

Binary will be located at `./target/release/gav`

## Command-line arguments

```
USAGE:
    gav [OPTIONS] <algo>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --number <number>               [default: 50]
    -s, --seed <seed>
    -w, --window_size <window size>     [default: 1000]

ARGS:
    <algo>     [possible values: closest_pair_dnc, closest_pair_sl, convex_hull_dnc, graham, graham_andrew,
              shamos_hoey]
```

## Example

Here is the result of running `gav graham -w 400 -n 20`:

![Graham](<example/graham.gif>)
