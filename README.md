# 🔍 Multi-Threaded Keyword Scanner (Rust)

A simple Rust CLI tool that scans a `.txt` file for a keyword using **multiple OS threads** and **channels** for communication.

This project is focused on learning:
- `std::thread::spawn`
- `std::sync::mpsc::channel`
- `Sender` / `Receiver`
- `move` closures
- Safe concurrency without shared mutable state

---

## ✨ What This Program Does

- Takes a text file and a keyword as input
- Splits file lines across multiple threads
- Each thread scans its assigned lines
- When the keyword is found, the thread sends the **line number** to the main thread
- The main thread collects and prints all matching line numbers

---

## 📦 Requirements

- Rust (stable)
- Cargo

Check installation:
```bash
rustc --version
cargo --version
