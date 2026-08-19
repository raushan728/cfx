# Installation

CFX is designed to be fully standalone. You **do not** need Rust or Cargo installed on your system to run the pre-compiled binary.

## Linux & macOS

You can install or update to the latest version of CFX using our automated shell script. Open your terminal and run:

```bash
curl -fsSL https://raw.githubusercontent.com/raushan728/cfx/main/scripts/install.sh | bash
```

The script will download the latest binary for your operating system and place it securely in `~/.local/bin/cfx`. It will also automatically add this directory to your `$PATH` if it is not already present.

### Uninstallation
To completely remove CFX from your Linux or macOS machine, run:
```bash
curl -fsSL https://raw.githubusercontent.com/raushan728/cfx/main/scripts/uninstall.sh | bash
```

---

## Windows

You can install or update to the latest version of CFX using our automated PowerShell script. Open **PowerShell** and run:

```powershell
iwr -useb https://raw.githubusercontent.com/raushan728/cfx/main/scripts/install.ps1 | iex
```

The script will download the latest `.exe` for Windows and place it securely in `%LOCALAPPDATA%\cfx\bin`. It will also seamlessly add this directory to your User Environment `Path`.

### Uninstallation
To completely remove CFX from your Windows machine, run:
```powershell
iwr -useb https://raw.githubusercontent.com/raushan728/cfx/main/scripts/uninstall.ps1 | iex
```

---

## Building from Source

If you prefer to compile CFX yourself:

1. Ensure you have Rust installed via `rustup`.
2. Clone the repository and navigate into it:
   ```bash
   git clone https://github.com/raushan728/cfx.git
   cd cfx
   ```
3. Compile the release binary:
   ```bash
   cargo build --release
   ```

The resulting executable will be placed at `target/release/cfx`.
