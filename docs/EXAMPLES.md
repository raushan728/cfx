# CFX Examples & Cookbook

This guide provides practical, easy-to-understand examples of how to run every CFX command. Whether you are backing up family photos or securing sensitive documents, you'll find the exact command you need here.

---

## 1. Encrypting Files (`cfx encrypt`)

The `encrypt` command locks your files. It will prompt you for a password to secure the file.

### Basic Encryption
If you just want to secure a file and don't need the original anymore:
```bash
cfx encrypt private_document.pdf
```
* **Result**: You now have a secure `private_document.pdf.cfx` file. The original `private_document.pdf` is safely deleted.

### Keeping the Original File
If you want to create a secure copy but still keep your original, unencrypted file:
```bash
cfx encrypt family_photo.jpg --keep
```
* **Result**: You will have both `family_photo.jpg` (unencrypted) and `family_photo.jpg.cfx` (securely encrypted).

### Saving to a Specific Location
If you want to encrypt a file directly onto a USB drive or a specific backup folder:
```bash
cfx encrypt tax_returns.zip --output /media/usb/tax_backup.cfx
```
* **Result**: The encrypted file is saved precisely where you requested as `tax_backup.cfx`. The original is deleted.

### Forcing an Overwrite
If a file named `report.cfx` already exists, CFX will normally stop and refuse to overwrite it to protect your data. If you are sure you want to replace it:
```bash
cfx encrypt report.docx --output report.cfx --force
```

---

## 2. Decrypting Files (`cfx decrypt`)

The `decrypt` command unlocks your `.cfx` files back into their original form.

### Basic Decryption
If you just want to unlock a file and no longer need the `.cfx` archive:
```bash
cfx decrypt private_document.pdf.cfx
```
* **Result**: CFX asks for your password, unlocks the file, and restores the original `private_document.pdf`. The `.cfx` file is then deleted.

### Keeping the Encrypted Archive
If you are just viewing the file temporarily but want to keep the locked archive safe on your hard drive:
```bash
cfx decrypt tax_backup.cfx --keep
```
* **Result**: CFX restores `tax_returns.zip` (which was its original name inside the archive) and leaves `tax_backup.cfx` perfectly intact.

### Extracting to a Specific Location
If you want to decrypt a file directly onto your Desktop, even if the file is stored elsewhere:
```bash
cfx decrypt /media/usb/tax_backup.cfx --output ~/Desktop/restored_taxes.zip
```
* **Result**: The file is unlocked and placed directly on your Desktop.

---

## 3. Checking File Metadata (`cfx info`)

The `info` command lets you check the technical details of a `.cfx` file. You do **not** need a password to run this command.

### Viewing Public Information
```bash
cfx info unknown_file.cfx
```
* **Result**: CFX prints out the version of CFX used to create the file and the algorithms used (e.g., Argon2id, ChaCha20Poly1305). 
* **Note**: It will *never* reveal the original filename or the file size to anyone without the password.

---

## 4. Verifying File Integrity (`cfx verify`)

The `verify` command allows you to test your password and make sure the file is not corrupted, *without* actually extracting the decrypted file to your hard drive.

### Checking a Backup
If you have an old backup and just want to make sure you remember the password and the file is safe:
```bash
cfx verify old_archive.cfx
```
* **Result**: CFX will prompt for your password and silently read the entire file. If the file is 100% pristine and the password is correct, it will print "Verification successful".

---

## 5. Cleaning Up Temp Files (`cfx recover`)

The `recover` command is used for cleaning up your hard drive if CFX was interrupted (like a power outage or a forced quit).

### Deleting an Orphaned Temp File
When an operation fails midway, CFX leaves a `.tmp` file behind (e.g., `huge_video.mp4.cfx.tmp`). To safely clean this up:
```bash
cfx recover huge_video.mp4.cfx.tmp
```
* **Result**: CFX confirms it is a temporary file and deletes it to free up disk space. (Note: CFX will explicitly refuse to delete healthy, complete `.cfx` files if you use this command).

---

## Tips for Automation (Scripts)

If you are using CFX in a background script, you might want to hide the interactive progress bar to keep your logs clean:

```bash
cfx encrypt backup.tar.gz --no-progress
```
