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
    This project only supports the <a href="https://store.steampowered.com/app/2420110/Horizon_Forbidden_West_Complete_Edition/">Steam</a> version of the game.
</h4>

## Usage

### Pre-requirements

- A decrypted PS4 save file (`checkpoint.dat` file).
- (Optional) A `param.sfo` file associated with the save file.
- (Optional) A PNG image for the save file.

### Installation

> [!CAUTION]
> This project is currently under **heavy development**, which means that it may be subject to **breaking changes** or *
*corrupt** save files.
> Use it at your **own risk**.

Download the executable from the
repository's [release page](https://github.com/zakuciael/hfw-save-converter/releases/latest) or build locally following
the instructions in the [build section](#build).

### Execution

```shell
./hfw-save-converter.exe <checkpoint.dat> [-F <path/to/param.sfo>]
```

For more options and configurations, check the **help** command of the executable.

## Build

> TODO: Detail build steps for the project.

## License

This project is distributed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for more information.