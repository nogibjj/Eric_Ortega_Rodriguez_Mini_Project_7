# Week 7 Mini Project: Rust Tool Packaging
## Eric Ortega Rodriguez 

![alt text](image.png)
## Project Overview
Users can run the tool to multiply two predefined numbers (which can be customized in the code), and the output will display the result of the multiplication. In addition to showcasing the packaging of a Rust tool.

This project also includes a Continuous Integration/Continuous Deployment (CI/CD) pipeline using GitHub Actions. 
- This automates the build process, tests the code, and produces the binary artifact for distribution.

## Features
- Package a Rust tool
- A user guide on how to install and use the tool
- Continuous Integration/Continuous Deployment (CI/CD) pipeline that builds and packages the Rust binary

## Deliverables 
- Packaged tool
- Written Guide
- If you choose to use Rust, provide the binary file as an artifact in CI/CD

## Using This Repo 
To build and run the project, ensure you have the following:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: The Rust package manager (`cargo`) is required to build and run the project
- **Git**: A Git/github account is needed to clone the repository
- **CI/CD**: GitHub Actions setup for the CI/CD pipeline

A way to verify if Rust and Cargo are installed is you can run the following commands:
```bash
rustc --version
cargo --version

