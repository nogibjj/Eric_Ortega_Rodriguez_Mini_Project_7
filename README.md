# Week 7 Mini Project: Rust Tool Packaging

## Eric Ortega Rodriguez 

[![CI/CD](https://github.com/nogibjj/Eric_Ortega_Rodriguez_Mini_Project_7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Eric_Ortega_Rodriguez_Mini_Project_7/actions/workflows/CI.yml)

![alt text](image.png)

## Project Overview

This project demonstrates the creation and packaging of a basic Rust command-line interface (CLI) tool. The tool uses the `clap` library to parse user input, allowing users to provide their name as an argument, and displays a personalized greeting message.

Additionally, the project includes a Continuous Integration/Continuous Deployment (CI/CD) pipeline implemented using GitHub Actions. This pipeline automates the build process, tests the code, and produces the binary artifact for distribution.

### Features
- **Rust CLI Tool**: A simple CLI tool that takes user input and displays a greeting.
- **Packaging**: The tool is packaged for easy distribution.
- **CI/CD Pipeline**: Automated build and packaging of the Rust binary using GitHub Actions.

## Tool Usage

### How It Works
The tool uses the `clap` library for argument parsing. Users can run the tool by providing their name as a command-line argument, and the tool will display a personalized greeting message.

Example:
```bash
$ ./basic_cli --name "Eric"
Hello, Eric!
```

### Code Snippet
Below is the Rust code for the tool:
```rust
use clap::Parser;

/// A basic example of using clap
#[derive(Parser)]
#[command(name = "basic_cli")]
#[command(about = "A basic CLI example using clap", long_about = None)]
struct Cli {
    /// Name of the person to greet
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}
```

## Getting Started

### Prerequisites
To build and run the project, ensure you have the following:
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: The Rust package manager (`cargo`) is required to build and run the project.
- **Git**: A Git/GitHub account is needed to clone the repository.
- **CI/CD**: GitHub Actions setup for the CI/CD pipeline.

### Installation
Clone the repository:
```bash
git clone <repository-url>
cd <repository-directory>
```

Build the project:
```bash
cargo build --release
```

Run the tool:
```bash
./target/release/basic_cli --name "YourName"
```

## Download the Binary

To download the latest compiled binary of the tool, visit the [GitHub Actions page](https://github.com/nogibjj/Eric_Ortega_Rodriguez_Mini_Project_7/actions/runs/12283034388/artifacts/2307685516) for the latest successful build and download the artifact named `Eric_Ortega_Rodriguez_Mini_Project_7`.

You can also download the binary from any recent build by navigating to the **"Actions"** tab in this repository, selecting the latest run, and scrolling down to the **Artifacts** section.

### Verify Installation
Check if Rust and Cargo are installed by running:
```bash
rustc --version
cargo --version
```

## CI/CD Pipeline
The GitHub Actions workflow automates the following:
- Building the project
- Running tests
- Packaging the binary as an artifact

To ensure the pipeline is running correctly:
1. Push your changes to the repository.
2. Monitor the **"Actions"** tab for the workflow status.
3. Download the binary artifact after a successful build.
