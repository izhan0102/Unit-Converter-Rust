# Unit Converter

A command-line unit conversion tool written in Rust that allows users to convert between different units of measurement.

## Features

- Length conversions (meters ↔ feet)
- Weight conversions (kilograms ↔ pounds)
- Temperature conversions (Celsius ↔ Fahrenheit)
- User-friendly command-line interface
- Real-time conversion results

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [https://rustup.rs/](https://rustup.rs/)

2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/unit-converter.git
   cd unit-converter
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Run the program:
   ```bash
   cargo run
   ```

2. Select the type of conversion you want to perform:
   - Enter `1` for Length (meters ↔ feet)
   - Enter `2` for Weight (kilograms ↔ pounds)
   - Enter `3` for Temperature (Celsius ↔ Fahrenheit)
   - Enter `q` to quit the program

3. Enter the value you want to convert

4. The program will display the conversion results in both directions

## Example

```
Welcome to Unit Converter!
Available conversions:
1. Length (meters <-> feet)
2. Weight (kilograms <-> pounds)
3. Temperature (Celsius <-> Fahrenheit)

Select conversion type (1-3) or 'q' to quit: 1
Enter value to convert: 10
10 meters = 32.8084 feet
10 feet = 3.048 meters
```

## License

This project is open source and available under the MIT License.