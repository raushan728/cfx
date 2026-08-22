# Command Reference & Usage Guide

CFX is designed to be simple and accessible, even if you aren't familiar with command-line tools. This guide will show you exactly how to use each command with practical, real-world examples.

---

## 1. Encrypting a File (`cfx encrypt`)

**What it does**: Takes any file (like a PDF, photo, or document) and securely locks it into a `.cfx` archive. By default, it safely deletes your original file once the encryption is successful, leaving only the locked version.

**Basic Usage**:
```bash
cfx encrypt <file>
```

**Beginner-Friendly Examples**:

*Scenario A: You have a tax document called `taxes_2026.pdf` that you want to securely lock.*
```bash
cfx encrypt taxes_2026.pdf
```
> **What happens:** CFX will prompt you to create a password. Once finished, you will have a new encrypted file named `taxes_2026.pdf.cfx`, and the original `taxes_2026.pdf` will be deleted.

*Scenario B: You want to encrypt `photo.jpg`, but you **don't** want CFX to delete the original file.*
```bash
cfx encrypt photo.jpg --keep
```
> **What happens:** You will now have both your original `photo.jpg` and a secure `photo.jpg.cfx`.

*Scenario C: You want to save the encrypted file to a completely different folder, like a USB drive.*
```bash
cfx encrypt private_keys.txt --output /media/usb/backup.cfx
```
> **What happens:** The encrypted file is saved directly to your USB drive as `backup.cfx`.

**Available Options**:
- `--keep`: Keep the original file instead of deleting it.
- `-o, --output <PATH>`: Choose a custom name or folder for the encrypted file.
- `-f, --force`: If a file with the same name already exists, overwrite it.
- `--no-progress`: Hide the loading bar (useful for automated scripts).

---

## 2. Decrypting a File (`cfx decrypt`)

**What it does**: Unlocks a `.cfx` file back into its original form.

**Basic Usage**:
```bash
cfx decrypt <file.cfx>
```

**Beginner-Friendly Examples**:

*Scenario A: You want to unlock `taxes_2026.pdf.cfx`.*
```bash
cfx decrypt taxes_2026.pdf.cfx
```
> **What happens:** CFX will ask for your password. If it's correct, it will recreate your original `taxes_2026.pdf`. The encrypted `.cfx` file is then deleted.

*Scenario B: You want to unlock the file, but keep the `.cfx` archive intact as a backup.*
```bash
cfx decrypt taxes_2026.pdf.cfx --keep
```
> **What happens:** You get your unlocked `taxes_2026.pdf` back, but `taxes_2026.pdf.cfx` remains safely on your computer.

*Scenario C: You want to extract the unlocked file to a specific location with a different name.*
```bash
cfx decrypt backup.cfx --output /home/user/Desktop/restored_keys.txt
```

**Available Options**:
- `--keep`: Keep the encrypted `.cfx` file instead of deleting it.
- `-o, --output <PATH>`: Choose a custom name or folder for the unlocked file.
- `-f, --force`: Overwrite an existing file if one is already in the way.
- `--no-progress`: Hide the loading bar.

---

## 3. Checking File Info (`cfx info`)

**What it does**: Safely reads the public metadata of an encrypted file (like the CFX version and algorithms used). This command **does not** require a password. It will never reveal the original filename or the size of the original file, as those are kept strictly secret.

**Basic Usage**:
```bash
cfx info secret.cfx
```

> **What happens:** CFX prints technical details about the file, confirming it is a valid CFX archive.

---

## 4. Verifying a File (`cfx verify`)

**What it does**: Checks that your `.cfx` file is perfectly intact, hasn't been corrupted, and confirms that you remember the correct password. It does all of this *without* actually saving the unlocked file to your hard drive.

**Basic Usage**:
```bash
cfx verify secret.cfx
```

> **What happens:** CFX asks for your password. It then silently reads through the entire file. If it prints "Verification successful", you know the file is safe and your password is correct. If the file was corrupted or the password was wrong, it will tell you.

**Available Options**:
- `--no-progress`: Hide the loading bar.

---

## 5. Cleaning Up Interruptions (`cfx recover`)

Safely cleans up temporary files left behind by an interrupted or failed CFX operation. It uses a specialized cryptographic marker (`CFX~`) to verify that the target is a genuine CFX temporary file before prompting for deletion.

**Usage (Exact File)**:
```bash
cfx recover <file.tmp>
```

**Usage (Scanning a Directory)**:
```bash
cfx recover --scan <directory>
```

**Options**:
- `<input>`: (Required unless `--scan` is used) The exact file path to safely clean up.
- `--scan <DIRECTORY>`: (Required unless `<input>` is used) Safely inspects the target directory for genuine CFX temporary files.
- `-f, --force`: Deletes verified CFX temporary files without asking for `[y/N]` confirmation.
- `--no-progress`: Disables progress reporting.

---

## Global Commands

You can append these to any command to get help from the application itself:

- `cfx --help`: Prints a quick summary of all available commands.
- `cfx encrypt --help`: Prints all the specific options available for the `encrypt` command (works for all commands).
- `cfx --version`: Prints the version of CFX you currently have installed.
