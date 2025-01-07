# squish

✨ Overview
---

`squish` lets you resize images via the command line.

![demo](https://github.com/user-attachments/assets/cecbffd7-cdcc-4b04-8084-00fb8c8298a0)

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/squish.git
```

⚡️ Usage
---

### Help

```text
Usage: squish [OPTIONS] <STRING>

Arguments:
  <STRING>  Local file path, or "cb" for system clipboard

Options:
  -w, --width <INTEGER>         Width of resized image [default: 800]
  -o, --output-file <STRING>    Destination of resized output file
  -c, --copy-to-clipboard       Whether to copy resized image to clipboard (only supported for PNG images)
  -v, --verbose                 Whether to print updates
  -m, --print-markdown-address  Whether to print address of output file in markdown format
  -h, --help                    Print help
```

### Basic Usage

```bash
# resize clipboard contents, and write them back to clipboard
squish -c cb 

# resize clipboard contents, and write them to a file
squish -o path/to/output/image.png cb

# resize a local file, and write resized contents to a file
squish -o path/to/output/image.png path/to/input/image.png

# resize a local file, and write resized contents to a file and the clipboard
squish -o path/to/output/image.png -c path/to/input/image.png
```

ℹ️ Disclaimer
---

`squish` has only been tested on macOS so far. Feedback on any bugs on
Linux/Windows via [issues](https://github.com/dhth/squish/issues) is very much
appreciated.
