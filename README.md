# 🚀 deNux - The Rust-Powered Auto-Import Optimizer

> ✨ A blazing-fast Rust tool to replace and optimize auto-imports in Vue/Nuxt projects. Say goodbye to unnecessary imports and keep your code clean & efficient!

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat&logo=rust) ![License](https://img.shields.io/github/license/mikinovation/denux)

---

## 🦀 Why deNux?

`deNux` is a **high-performance** and **developer-friendly** auto-import optimizer built with **Rust**. It automatically detects, replaces, and optimizes import statements in your Vue/Nuxt projects, keeping your codebase clean and well-organized.

### 🔥 Key Features
- **Ultra-fast processing** with Rust’s performance edge 🚀
- **Smart auto-import detection** for used components & functions 🧠
- **Explicitly adds necessary imports** by converting auto-imported modules into import statements 🧹
- **Supports Vue 3, Nuxt 3, and modern JS frameworks** 🏗️
- **Seamless integration** into your development workflow ⚡

---

## 📦 Installation

### **Using Cargo (Recommended)**
```sh
cargo install denux
```

### **From Source**
```sh
git clone https://github.com/mikinovation/denux.git
cd denux
cargo build --release
```

---

## 🚀 Usage

### **Basic Command**
```sh
denux
```

### **Options**
| Option            | Description |
|------------------|-------------|
| `--dry-run`       | Show changes without applying them |
| `--verbose`       | Display detailed logs |

Example:
```sh
denux --dry-run --verbose ./src
```

---

## ⚙️ How It Works
1. **Parses Vue/Nuxt files** to extract `<script setup>` and `<template>` contents.
2. **Identifies missing & unnecessary imports** automatically.
3. **Optimizes import statements** while keeping your code readable and clean.
4. **Writes back the optimized files** (unless `--dry-run` is specified).

---

## 🛠️ Contributing
We welcome contributions! Feel free to **fork** this repository, submit a **PR**, or open an **issue** if you find a bug or have a feature request.

```sh
git clone https://github.com/mikinovation/denux.git
cd denux
cargo run
```

---

## 📜 License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## ⭐ Show Your Support!
If you like `deNux`, give it a ⭐ on GitHub and help spread the word!

[![GitHub stars](https://img.shields.io/github/stars/mikinovation/denux?style=social)](https://github.com/mikinovation/denux)
