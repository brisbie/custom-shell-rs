# Custom-Shell-RS

Custom-Shell-RS is a custom command-line shell written in Rust. It supports basic shell functionality such as executing commands, handling arguments, and managing the file system. This project was built as a hands-on exercise in systems programming and Rust development.

## Features

- Execute system commands
- Support for command-line arguments
- Navigate the file system (`cd`, `ls`, etc.)
- Simple error handling and output
- Built entirely in Rust

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/brisbie/custom-shell-rs.git
   cd custom-shell-rs
   ```

2. Build the project with Cargo:
   ```bash
   cargo build --release
   ```

3. Run the shell:
   ```bash
   cargo run
   ```

## Usage

Once running, you can execute standard shell commands:

```bash
> ls
> cd folder
> echo "Hello, world!"
```

## Contributing

Contributions are welcome! If you’d like to add features or fix bugs:

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Open a pull request

## License

This project is licensed under the MIT License. See the LICENSE file for details.
