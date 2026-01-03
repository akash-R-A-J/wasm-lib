# Rust + WASM + React Integration

> A demonstration project showcasing Rust library compilation to WebAssembly and integration with React applications.

## 📋 Overview

This repository demonstrates the complete workflow of:
- Building Rust libraries with `wasm-bindgen`
- Compiling Rust code to WebAssembly
- Integrating WASM modules into React applications
- Creating bidirectional JavaScript ↔ Rust communication

Perfect for developers learning to leverage Rust's performance and safety in web applications.

## 🏗️ Project Structure

```
wasm-lib/
├── my_lib/              # Rust library crate
│   ├── src/
│   │   └── lib.rs      # Core Rust implementation
│   ├── Cargo.toml      # Rust dependencies & config
│   └── Makefile        # Build automation
│
└── idmap-test/          # React consumer application  
    ├── src/
    │   └── App.jsx     # React components
    ├── public/
    └── package.json    # Node.js dependencies
```

## ✨ Features

- **🦀 Rust Performance**: Leverage Rust's zero-cost abstractions for computationally intensive tasks
- **🌐 Browser Compatibility**: WASM runs natively in modern browsers with near-native performance
- **⚡ Type Safety**: Rust's type system catches errors at compile time
- **🔄 Seamless Integration**: Easy-to-use JavaScript bindings via `wasm-bindgen`
- **📦 Modern Build Tools**: Uses wasm-pack for optimized WASM bundles

## 🚀 Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **wasm-pack**: WASM build tool
  ```bash
  cargo install wasm-pack
  ```

- **Node.js**: v16+ recommended
  ```bash
  # Verify installation
  node --version
  npm --version
  ```

### Building the Rust Library

```bash
cd my_lib

# Build for web target
wasm-pack build --target web

# Or use the Makefile
make build
```

This generates:
- `pkg/my_lib_bg.wasm` - Compiled WebAssembly binary
- `pkg/my_lib.js` - JavaScript bindings
- `pkg/my_lib.d.ts` - TypeScript type definitions

### Running the React Application

```bash
cd idmap-test

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

The React app will be available at `http://localhost:5173` (or the port shown in terminal).

## 🔧 Development Workflow

1. **Modify Rust Code**: Edit `my_lib/src/lib.rs`
2. **Rebuild WASM**: Run `cd my_lib && wasm-pack build --target web`
3. **React Hot Reload**: Changes automatically reflect in the React dev server

### Example: Adding a New Function

**In `my_lib/src/lib.rs`:**
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

**In React (`idmap-test/src/App.jsx`):**
```javascript
import init, { add, fibonacci } from '../path/to/my_lib/pkg';

function App() {
  const [result, setResult] = useState(null);

  useEffect(() => {
    init().then(() => {
      const sum = add(5, 7);
      const fib = fibonacci(10);
      setResult({ sum, fib });
    });
  }, []);

  return (
    <div>
      <p>5 + 7 = {result?.sum}</p>
      <p>Fibonacci(10) = {result?.fib}</p>
    </div>
  );
}
```

## 📚 Tech Stack

### Rust Library (`my_lib/`)
- **Rust** - Systems programming language
- **wasm-bindgen** - JavaScript ↔ Rust bindings
- **wasm-pack** - Build and package tool

### React Application (`idmap-test/`)
- **React** - UI library
- **Vite** - Fast build tool
- **WebAssembly** - Binary instruction format

## 🎯 Use Cases

This pattern is ideal for:
- **Cryptographic Operations**: Hashing, encryption, signing
- **Image Processing**: Filters, transformations, compression
- **Scientific Computing**: Simulations, mathematical computations
- **Game Logic**: Physics engines, pathfinding algorithms
- **Data Parsing**: Large file processing, complex parsing

## 🛠️ Common Commands

```bash
# Rust library
cd my_lib
cargo build                    # Standard Rust build
cargo test                     # Run Rust tests
wasm-pack build --target web   # Build WASM for web
wasm-pack build --target nodejs # Build WASM for Node.js

# React application
cd idmap-test
npm run dev                    # Development server
npm run build                  # Production build
npm run preview                # Preview production build
```

## 🐛 Troubleshooting

### WASM Module Not Found
```bash
# Ensure you've built the Rust library
cd my_lib && wasm-pack build --target web

# Check the import path in React matches the pkg/ output location
```

### Rust Compilation Errors
```bash
# Update Rust toolchain
rustup update

# Check wasm32 target is installed
rustup target add wasm32-unknown-unknown
```

### React Build Errors
```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

## 📖 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust learning resource
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) - Rust ↔ JavaScript bindings
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/) - Complete guide
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly) - WASM documentation

## 🤝 Contributing

Contributions welcome! Feel free to:
- Report bugs or request features via Issues
- Submit Pull Requests with improvements
- Share feedback and suggestions

## 📄 License

MIT License - feel free to use this project as a learning resource or template for your own applications.

## 👤 Author

Built by [akash-R-A-J](https://github.com/akash-R-A-J)

---

**💡 Tip**: This project serves as a foundation for building high-performance web applications by combining Rust's safety and speed with React's developer experience.
