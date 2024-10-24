[![CI/CD](https://github.com/nogibjj/Eric_Ortega_Rodriguez_Mini_Project_7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/Eric_Ortega_Rodriguez_Mini_Project_7/actions/workflows/CI.yml)
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

## Download the Binary

To download the latest compiled binary of the tool, visit the [GitHub Actions page](https://github.com/ericiortega/Eric_Ortega_Rodriguez_Mini_Project_7/actions/runs/1234567890) for the latest successful build and download the artifact named `Eric_Ortega_Rodriguez_Mini_Project_7`.

You can also download the binary from any recent build by navigating to the **"Actions"** tab in this repository, selecting the latest run, and scrolling down to the **Artifacts** section.
