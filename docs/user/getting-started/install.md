---
title: Install PDFTara Pixel
description: Download PDFTara Pixel for Windows 10 or 11, macOS or Linux from getPDFTara pixel.com or GitHub, or install it with PDFTara pixel Launcher.
order: 10
verified: 0.8.2
---

The easiest way is the [PDFTara Pixel page on getPDFTara pixel.com](https\://www\.pdftara.com/apps/PDFTara Pixel). You can also install it with [PDFTara pixel Launcher](https\://www\.pdftara.com/docs/craft-launcher/guides/install-and-update-apps), which lists PDFTara Pixel. For a specific build or file format, take it from the [latest release](https://github.com/storytold/PDFTara Pixel/releases/latest) on GitHub. In the file names below, `<version>` stands for the version number, for example `0.8.2`.

## Windows

PDFTara Pixel needs Windows 10 or later. On Windows 7 and 8 the installer stops with "PDFTara Pixel needs Windows 10 or later."

1. Open the [latest release](https://github.com/storytold/PDFTara Pixel/releases/latest).
2. Download `PDFTara Pixel-<version>-windows-x64.msi`. It fits most PCs (64-bit Intel and AMD).
3. Run the file and follow the installer.

To run without installing, download `PDFTara Pixel-<version>-windows-x64-portable.zip` instead and unzip it.

| Your PC                            | Installer                                | Portable                                          |
|------------------------------------|------------------------------------------|---------------------------------------------------|
| 64-bit Intel or AMD                | `PDFTara Pixel-<version>-windows-x64.msi`   | `PDFTara Pixel-<version>-windows-x64-portable.zip`   |
| Arm (Snapdragon and other Arm PCs) | `PDFTara Pixel-<version>-windows-arm64.msi` | `PDFTara Pixel-<version>-windows-arm64-portable.zip` |
| 32-bit                             | `PDFTara Pixel-<version>-windows-x86.msi`   | `PDFTara Pixel-<version>-windows-x86-portable.zip`   |

The installers and executables are code-signed.

## macOS

1. Download `PDFTara Pixel-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs, and is signed and notarized by Apple.
2. Open it and drag PDFTara Pixel to Applications.

## Linux

Pick one file from the release. Use `x86_64` for Intel and AMD, or `aarch64` for Arm.

| Format              | File                                         | Notes                                                                              |
|---------------------|----------------------------------------------|------------------------------------------------------------------------------------|
| AppImage            | `PDFTara Pixel-<version>-linux-x86_64.AppImage` | Runs anywhere and needs no install.                                                |
| Debian and Ubuntu   | `PDFTara Pixel-<version>-linux-x86_64.deb`      |                                                                                    |
| Fedora and openSUSE | `PDFTara Pixel-<version>-linux-x86_64.rpm`      |                                                                                    |
| Flatpak             | `PDFTara Pixel-<version>-linux-x86_64.flatpak`  | Run `flatpak install --user <file>`, then `flatpak run ai.storyteller.PDFTara Pixel`. |
| Tarball             | `PDFTara Pixel-<version>-linux-x86_64.tar.gz`   | Unpack it anywhere.                                                                |

The first time you run the AppImage, it adds its icon and menu entry under `~/.local/share`. Set `PDFTara Pixel_NO_DESKTOP_INTEGRATION=1` to skip that. For RISC-V, take the `riscv64` file: `PDFTara Pixel-<version>-linux-riscv64.tar.gz`, `.deb` or `.rpm`. The RISC-V tarball needs glibc 2.39 or newer.

On a Wayland session, files dropped on the PDFTara Pixel window do not open yet. Use **File › Open…** instead, or copy the image in your file manager and paste it with Ctrl+V. To drag and drop, start PDFTara Pixel under XWayland with `WAYLAND_DISPLAY= PDFTara Pixel`. PDFTara Pixel shows the right command for your install in a notice until you dismiss it.

## FreeBSD and the web

FreeBSD 14 (x86_64) has a tarball, `PDFTara Pixel-<version>-freebsd-x86_64.tar.gz`, laid out like `/usr/local`. The web version comes as `PDFTara Pixel-web-<version>.zip`, a static site you host on any web server. It runs in a modern browser.

## Check your download

`SHA256SUMS.txt` on the release lists a checksum for every file.

## Next step

Open an image: [Open and save files](../guides/open-and-save-files.md).
