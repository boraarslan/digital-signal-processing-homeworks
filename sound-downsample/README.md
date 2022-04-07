## Homework Week 1

Downsamples the given `.wav` formatted file.

### Usage

By default program constructs output using the `tone.wav` sound located in the `/assets` folder. 
To run the program using `cargo` and pass the arguements at the same time run with the:
```
cargo run -- <ARGS>
```

```
USAGE:
    cargo run -- [OPTIONS] --target-khz <TARGET_KHZ>

OPTIONS:
    -f, --file <FILE>                [default: tone]
    -h, --help                       Print help information
    -t, --target-khz <TARGET_KHZ>    [possible values: two, four, six, eight, twelve, sixteen,
                                     thirtytwo, fortyeight, sixtyfour]
```

For example, to downsample the `count.wav` sound to 8kHz:

```
cargo run -- -t eight -f count
```



### Example

Examples are located in `/assets` folder.