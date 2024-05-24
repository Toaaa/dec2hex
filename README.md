# dec2hex

**dec2hex** is a command-line Rust application that converts decimal numbers to hexadecimal. Whether you need to convert a single decimal number or process multiple numbers stored in a text file, **dec2hex** provides a reliable solution.

## Features
- Convert individual decimal numbers to hexadecimal.
- Process decimal numbers stored in a text file and output their hexadecimal representations.
- Easy to build and run on any platform.

## Usage
### Convert a Single Decimal Number

To convert a single decimal number to hexadecimal, simply run the following command:
```sh
dec2hex <DECIMAL_NUMBER>
```

Replace `<DECIMAL_NUMBER>` with the decimal number you want to convert.

### Process Decimal Numbers from a File

If you have decimal numbers stored in a text file and want to convert them to hexadecimal, use the following command:
```sh
dec2hex -f <INPUT_FILE> -o <OUTPUT_FILE>
```

Replace `<INPUT_FILE>` with the path to your input file containing decimal numbers, and `<OUTPUT_FILE>` with the desired name for the output file containing hexadecimal representations.
> Please note that `<OUTPUT_FILE>` is an optional parameter and is not required by `<INPUT_FILE>`.

## Installation
Currently there is a [precompiled version](https://github.com/Toaaa/dec2hex/releases) for **Windows only**.

To build the **dec2hex** application, make sure you have Rust installed on your system. Then, clone this repository and navigate to its directory:
```sh
git clone https://github.com/toaaa/dec2hex.git
cd dec2hex
```

Next, build the application using Cargo:
```sh
cargo build --release
```
Once the build process is complete, you can find the executable file in the `target/release` directory.