<!--suppress HtmlDeprecatedAttribute, CheckImageSize -->
<h2 align="center">
  <a href="https://github.com/zakuciael/hfw-save-converter">
    <img alt="Horizon Forbidden West Save Converter" src="assets/logo.png" width="200px" />
  </a>
  <br />
  HFW Save Converter
</h2>

<h4 align="center">
    A cross-platform application to convert Horizon Forbidden West's PS4 saves to PC. <br />
</h4>
<br />

### Pre-requirements

- A decrypted PS4 save file (`checkpoint.dat` file).
- (Optional) A `sce_sys/` folder containing the following files:
    - A `param.sfo` file associated with the save file.
    - A `icon0.png` file containing the save file's image.

### Installation

Download the executable from the
repository's [release page](https://github.com/zakuciael/hfw-save-converter/releases/latest) or build locally following
the instructions in the [build section](#build).

### Usage

```shell
./hfw-save-converter.exe <PS4_SAVE_FILE> [-o <OUTPUT_DIR>]
```

The converter will automatically look up the save metadata from `sce_sys/param.sfo` file located in the same directory
as the PS4 save file.

For more options and configurations, check out the **help** command.

## Build

To build the application, install Rust toolchain using the instructions found under the
following [link](https://rustup.rs/).
After installation, run the following command in the terminal

```shell
cargo build -r
```

This will build the release version of the application for the running operating system.

## License

This project is distributed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for more information.