## Homework Week 1

Make a program that takes an `.bmp` image and outputs the histogram of pixel brightness and its PMF.

### Usage

By default program constructs output using the `cat.bmp` image located in the `/images` folder. To run the code using the `cat` image:

```
cargo run --release
```
If you want to run with a different image add the image in the `/images` folder and give the name of the image as arguement (without file format).

```
cargo run --release -- <image_name>
```
For example to run the code with the dog image:

```
cargo run --release -- dog
```

### Example

<img src=./images/cat.bmp width="300">
<img src=cat_pixel_histogram.svg width="550">
<img src=cat_pmf_histogram.svg width="550">