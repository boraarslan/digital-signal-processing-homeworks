## Homework Week 1

Make a program that takes an `.bmp` image and outputs the histogram of pixel brightness and its PMF.

### Usage

By default program constructs output using the `cat.bmp` image located in the `/images` folder. To run the code using the `cat` image:

```
cargo run --release
```
To run with options:

```
cargo run --release -- <OPTIONS>
```
For example to run the code with the dog image:

```
cargo run --release -- --file dog
```

For the help message:
```
cargo run --release -- --help
```

```
week1 0.1.0

USAGE:
    week1 [OPTIONS]

OPTIONS:
        --bin <BIN>      [default: 1]
        --file <FILE>    [default: cat]
    -h, --help           Print help information
    -V, --version        Print version information
```

### Example

<img src=./images/cat.bmp width="300">
<img src=cat_pixel_histogram.svg width="550">
<img src=cat_pmf_histogram.svg width="550">