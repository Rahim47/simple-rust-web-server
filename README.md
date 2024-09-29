# A Simple Web Server w/ Rust & Actix-Web

This project is features an HTTP server written in Rust using the Actix-Web framework.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can check by running:

```bash
rustc --version
cargo --version
```

If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

Clone the repository to your local machine:

```bash
https://github.com/Rahim47/simple-rust-web-server.git
```

Navigate into the project directory. For project development, you can use the following command:

```bash
cargo watch -c -w src -x run
```

The server will start running on http://127.0.0.1:8080.

```

## Usage

The application exposes several HTTP endpoints:

- `GET /`: Returns a simple greeting. Used to check if the server is running.
- `GET /user/{id}`: Retrieves a specfic User.
- `POST /user`: Creates a User.

You can use tools like Postman or cURL to interact with the API.
