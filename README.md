<h1 align="center">
  🔐 Passnager (name suggestions welcome)
</h1>

<p align="center">
  <strong>A simple, private and local-first password manager.</strong>
</p>

<p align="center">
  Your passwords. Your device. Your data.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.11-FFC131?logo=tauri&logoColor=white" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Svelte-5.57-FF3E00?logo=svelte&logoColor=white" alt="Svelte" />
  <img src="https://img.shields.io/badge/Rust-1.98-000000?logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License" />
</p>

---

## ✨ About

**Passnager** is a desktop password manager focused on privacy, simplicity and local data ownership.

Your vault is stored locally on your device and protected with modern cryptographic primitives, without requiring a cloud account or remote server.

The project is built with **SvelteKit + Tauri 2**, combining a modern web UI with a lightweight native desktop application.

## 🖥️ Preview

<p align="center">
  <img src="./static/preview.webp" alt="Passnager passwords" width="800" />
</p>

## 🔐 Security

Passnager follows a local-first security model.

Sensitive vault data is encrypted before being persisted locally using:

| Component        | Purpose                               |
| ---------------- | ------------------------------------- |
| **Argon2id**     | Password-based key derivation         |
| **AES-256-GCM**  | Authenticated encryption              |
| **Random nonce** | Unique nonce per encryption operation |
| **SQLite**       | Local persistent storage              |

The application does not require your vault to be stored on a remote server.

> [!WARNING]  
> Passnager is currently under active development and has **not been independently security audited**.
> Do not rely on it for critical secrets until you have reviewed the implementation and accepted the associated risks.

## 🛠️ Tech Stack

**Frontend**

- Svelte
- SvelteKit
- TypeScript

**Desktop**

- Rust
- Tauri 2

**Storage**

- SQLite

**Cryptography**

- Argon2id
- AES-256-GCM

## 🚀 Development

### Requirements

Make sure you have the required tooling for your platform:

- Node.js
- pnpm
- Rust
- Tauri system dependencies

### Clone

```bash
git clone https://github.com/eriandev/passnager.git
cd passnager
```

### Install dependencies

```bash
pnpm install
```

### Start development

```bash
pnpm tauri dev
```

### Build for Linux

```bash
pnpm tauri:build:linux
```

### Build Windows

```bash
pnpm tauri:build:windows
```

## 📦 Releases

Official builds are published through GitHub Releases.

Supported targets currently include:

- 🐧 Linux — x86_64
- 🪟 Windows — x86_64

Release builds are generated automatically through GitHub Actions.

## 🗺️ Roadmap

- [ ] Password categories
- [ ] Password generator
- [ ] Password strength analysis
- [ ] Import / export
- [ ] Automatic vault locking
- [ ] Window state persistence
- [ ] Automatic updates

## 🤝 Contributing

Contributions, suggestions and bug reports are welcome.

Before opening a pull request, please make sure your changes are tested locally and follow the existing project conventions.

## 📄 License

Passnager is open source software licensed under the **MIT License**.

See [`LICENSE`](LICENSE) for the full license text.

---

<p align="center">
  Built with ❤️ by <a href="https://github.com/eriandev">eriandev</a>
</p>
