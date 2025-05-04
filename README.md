# Maelstrom Distributed Systems Training in Rust 🚀🦀

This repository contains my Rust implementations of distributed systems challenges from the
excellent [Fly.io Distributed Systems Series](https://fly.io/dist-sys/1/), tested
using [Jepsen's Maelstrom](https://github.com/jepsen-io/maelstrom) framework.

The goal of this project is to learn core distributed systems concepts like message passing, broadcast, causal
consistency, and fault tolerance by building small, verifiable nodes that pass Maelstrom's rigorous tests.

## 🧠 What You'll Find Here

- ✅ Minimal, modular Rust implementations
- ✅ Detailed comments to explain distributed systems behavior
- ✅ JSON message parsing using [`serde`](https://docs.rs/serde/latest/serde/)

## 📚 Challenge Checklist

| Challenge    | Description                              | Status         |
|--------------|------------------------------------------|----------------|
| `echo`       | Echoes back the input message            | ✅ Completed    |
| `unique-ids` | Generates unique IDs on request          | ✅ Completed    |
| `broadcast`  | Broadcasts messages across the cluster   | ✅ Completed    |
| `grow-only`  | Implements a stateless grow-only counter | 🔄 In Progress |

## 🛠️ Getting Started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Maelstrom](https://github.com/jepsen-io/maelstrom) (requires Java JDK)

### Run a Challenge (example: `echo`)

```bash
cd challenges/echo
maelstrom test -w echo --bin target/debug/echo --nodes n1 --time-limit 10 --log-stderr
```

## 📁 Each challenge has its own directory with a binary and config.

📦 Crates Used

    [serde / serde_json] - For parsing message payloads

## 📈 Why I Built This

Distributed systems are hard, but test harnesses like Maelstrom make it possible to experiment safely. This project is
both a personal learning journey and a resource for others learning Rust and distributed systems.

## 📬 Contact

If you're working on similar projects or want to collaborate, feel free to reach out on LinkedIn or open an issue!

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
