# cinit

`cinit` is a command-line tool written in Rust that initializes CI/CD configuration files for GitLab CI, GitHub Actions, Bitbucket Pipelines, and Jenkins. It helps developers quickly set up continuous integration and deployment pipelines for their projects.

## Features

- Initialize GitLab CI configuration file.
- Initialize GitHub Actions configuration file.
- Initialize Bitbucket Pipelines configuration file.
- Initialize Jenkins configuration file.

## Requirements

- Rust (latest stable version)
- Docker (optional, for building and running the project in a container)

## Installation

### From Source

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/cinit.git
   cd cinit
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Install the binary:   

   ```sh
   sudo cp target/release/cinit /usr/local/bin/
   ```

### Using Docker

1. Build the Docker image:

   ```sh
   docker build -t cinit .
   ```

2. Run the Docker container to check the version:

   ```sh
   docker run --rm cinit
   ```

## Usage

After installation, you can use the cinit command to initialize CI/CD configuration files.

### Commands

1. Initialize GitLab CI configuration:

   ```sh
   cinit gitlabci
   ```

2. Initialize GitHub Actions configuration:

   ```sh
   cinit github
   ```

3. Initialize Bitbucket Pipelines configuration:

   ```sh
   cinit bitbucket
   ```

4. Initialize Jenkins configuration:

   ```sh
   cinit jenkins
   ```

## Help

For help with commands, you can use the --help flag:

   ```sh
   cinit --help
   ```

## Development

### Testing

To run the tests, use:

   ```sh
   cargo test
   ```

### Makefile

A **Makefile** is provided to simplify common tasks.

- Build the project:

   ```sh
   make build
   ```

- Install the binary:

   ```sh
   make install
   ```

- Uninstall the binary:

   ```sh
   make uninstall
   ```

- Clean the project:

   ```sh
   make clean
   ```

- Run tests:

   ```sh
   make test
   ```

## Dockerfile

A Dockerfile is provided to build and run the project in a Docker container:   

### Build the Docker image

   ```sh
   docker build -t cinit .
   ```
   

### Run the Docker container to check the version

   ```sh
   docker run --rm cinit
   ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## Acknowledgments

- Clap - A simple to use, full-featured Command Line Argument Parser for Rust.
- Rust - A language empowering everyone to build reliable and efficient software.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions or suggestions, feel free to reach out at [kukuhsatriowibowo@gmail.com](mailto:kukuhsatriowibowo@gmail.com).


**Happy Coding!**
