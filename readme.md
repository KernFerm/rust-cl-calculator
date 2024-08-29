# Rust Command-Line Calculator

Welcome to the Rust Command-Line Calculator! This is a simple command-line application written in Rust that allows you to perform basic arithmetic operations.

## Features

- Addition
- Subtraction
- Multiplication
- Division

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Make sure you have Rust installed on your machine)

## How to Use

1. **Download the project files**:
    - Download the project files from the provided source (e.g., a ZIP file).
    - Extract the files to a directory of your choice.

2. **Navigate to the project directory**:
    ```sh
    cd <project-directory>
    ```

3. **Build the project**:
    ```sh
    cargo build
    ```

4. **Run the project**:
    ```sh
    cargo run
    ```

5. **Using the Calculator**:
    - When you run the program, you will be greeted with a welcome message.
    - You will be prompted to enter the first number. Type the number and press Enter.
    - Next, you will be prompted to enter an operator. Choose from the following:
        - `+` for addition
        - `-` for subtraction
        - `*` for multiplication
        - `/` for division
    - Finally, you will be prompted to enter the second number. Type the number and press Enter.
    - The result of the operation will be displayed.
    - You can continue performing calculations or type `q` to quit the program.

## Example

Welcome to the Rust Command-Line Calculator!
- Enter the first number (or 'q' to quit): 10 Operators:

- : Addition
- : Subtraction
- : Multiplication / : Division Enter the operator: + Enter the second number: 5 Result: 10 + 5 = 15
- Enter the first number (or 'q' to quit): q

Thank you for using the calculator! Created by Bubbles The Dev, Goodbye.


## Building on a 64-bit System

This project is built and tested on a 64-bit system. Ensure that your Rust installation is configured for 64-bit architecture. You can verify this by running:

```sh
rustup target list --installed
```

- You should see `x86_64-unknown-linux-gnu` (or a similar 64-bit target) in the list of installed targets.

## License

- This project is licensed under the MIT License. See the LICENSE file for details

## Acknowledgements

- Created by Bubbles The Dev

------