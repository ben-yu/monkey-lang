# 🐒 Monkey Language Interpreter

A Rust implementation of the Monkey programming language from the book "Writing an Interpreter in Go" by Thorsten Ball, with WebAssembly support for running in the browser.

## 🌐 Try it Online

**Live Demo:** [https://YOUR_USERNAME.github.io/monkey-lang](https://YOUR_USERNAME.github.io/monkey-lang)

The interpreter runs entirely in your browser using WebAssembly!

## Features

- ✅ **Arithmetic expressions** (`5 + 5`, `10 - 5`, `2 * 3`, `10 / 2`)
- ✅ **Boolean expressions** (`true`, `false`, `!true`, `1 < 2`)
- ✅ **Conditional expressions** (`if (x > 5) { "greater" } else { "less" }`)
- ✅ **Variable bindings** (`let x = 10;`)
- ✅ **Functions** (`let add = fn(a, b) { a + b };`)
- ✅ **Function calls** (`add(5, 5)`)
- ✅ **Closures** (functions that capture their environment)
- ✅ **Return statements** (`return 42;`)

## Language Examples

```monkey
// Variables
let age = 25;
let name = "Monkey";

// Functions
let fibonacci = fn(x) {
  if (x == 0) {
    return 0;
  } else {
    if (x == 1) {
      return 1;
    } else {
      return fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

// Closures
let newAdder = fn(x) {
  fn(y) { x + y };
};
let addTwo = newAdder(2);
addTwo(3); // returns 5

// Conditionals
let max = fn(a, b) {
  if (a > b) {
    a
  } else {
    b
  }
};
```

## 🚀 Local Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/): `cargo install wasm-bindgen-cli`

### Running the CLI REPL

```bash
cargo run
```

### Building for WebAssembly

1. Add the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Build the project:
```bash
./build-wasm.sh
```

3. Serve locally:
```bash
cd web
python3 -m http.server 8000
```

4. Open http://localhost:8000

## 📦 GitHub Pages Deployment

This project includes automated GitHub Actions deployment to GitHub Pages.

### Setup Instructions

1. **Fork/Clone this repository**

2. **Enable GitHub Pages:**
   - Go to your repository settings
   - Navigate to "Pages" section
   - Set source to "GitHub Actions"

3. **Push to main branch:**
   ```bash
   git add .
   git commit -m "Deploy Monkey language interpreter"
   git push origin main
   ```

4. **Access your site:**
   - Your site will be available at: `https://YOUR_USERNAME.github.io/monkey-lang`
   - Check the Actions tab to monitor deployment progress

### How it Works

- **GitHub Actions** automatically builds the WASM module on every push to `main`
- **wasm-bindgen** generates JavaScript bindings for the Rust code
- **GitHub Pages** serves the static files from the `docs/` directory
- **Zero configuration** - just push and deploy!

## 🏗️ Project Structure

```
monkey-lang/
├── src/
│   ├── lib.rs           # WASM bindings
│   ├── main.rs          # CLI entry point
│   ├── ast.rs           # Abstract Syntax Tree
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # Parser
│   ├── evaluator.rs     # Interpreter/Evaluator
│   ├── object.rs        # Runtime objects
│   ├── environment.rs   # Variable environment
│   ├── repl.rs          # CLI REPL
│   └── token.rs         # Token definitions
├── docs/                # GitHub Pages files
│   ├── index.html       # Web interface
│   └── pkg/             # Generated WASM files
├── .github/workflows/   # GitHub Actions
└── build-wasm.sh        # Local build script
```

## 🧪 Running Tests

```bash
cargo test
```

## 🛠️ Built With

- **Rust** - Systems programming language
- **WebAssembly** - For browser compatibility
- **wasm-bindgen** - Rust/WASM/JS interop
- **GitHub Actions** - CI/CD pipeline
- **GitHub Pages** - Static site hosting

## 📚 References

- [Writing an Interpreter in Go](https://interpreterbook.com/) by Thorsten Ball
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
