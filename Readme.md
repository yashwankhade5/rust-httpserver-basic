# rust_server

A simple Rust HTTP server that serves a static HTML file.

## Features

- Listens on `127.0.0.1:3000`
- Serves the contents of `hello.html` on every request

## Usage

1. **Build the project:**
   ```sh
   cargo build
   ```

2. **Run the server:**
   ```sh
   cargo run
   ```

3. **Open your browser and visit:**
   ```
   http://127.0.0.1:3000
   ```

You should see the contents of `hello.html`.

## Project Structure

- `src/main.rs` — Main server code
- `hello.html` — HTML file served to clients

## Requirements

- Rust (edition 2021 or later)