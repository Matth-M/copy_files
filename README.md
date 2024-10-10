CLI tool to copy files content to clipboard.

Built for linux with a X11 desktop environment. Because of [how X11 implements clipboards](https://jameshunt.us/writings/x11-clipboard-management-foibles/), the cli is running an infinite loop to keep the clipboard content.

# Example
```bash
copy_content file1.txt file2.txt file3.ext
```
