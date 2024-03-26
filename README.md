# Adaptemoji

Convert your regular Telegram emojis into adaptive monochrome versions

## Examples

| Original                                         | Adaptive                                         | Adaptive negative                                                  |
| ------------------------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------ |
| ![Original image](assets/examples/original.webp) | ![Adaptive image](assets/examples/adaptive.webp) | ![Adaptive negative image](assets/examples/adaptive-negative.webp) |

As you can see, one of the adaptive versions looks wrong compared to original. But, if you switch your color theme from light to dark (or dark to light) the other one will be wrong. So, you need to use the one, that fits your background more.

## Requirements

1. CC linker (Windows - Microsoft Visual Studio with C++ Support) (Linux - gcc)

2. [Rust](https://www.rust-lang.org/tools/install)

## Installation

```bash
cargo install adaptemoji
```

## How to use

### Regular

```bash
adaptemoji -i your-image.png -o output-image.png
```

### Negative

```bash
adaptemoji -i your-image.png -o output-image.png -n
```

Also Telegram requires your emoji to be 100px x 100px in size. If you want adaptemoji automatically to resize image, add `-r` flag

### Regular resized

```bash
adaptemoji -i your-image.png -o output-image.png -r
```

### Negative resized

```bash
adaptemoji -i your-image.png -o output-image.png -nr
```

## Using as library

### Installation

```bash
cargo add adaptemoji
```

### Examples

```rust
use adaptemoji::AdaptiveEmojiConvert;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let img = image::open("./assets/examples/original.webp")?;
    let mut resized_img = img
        .resize_to_fill(100, 100, image::imageops::FilterType::Triangle) // Resize image to 100px x 100px
        .to_luma_alpha8();

    resized_img.convert_adaptive(false).save("./target/adaptive.png")?;

    Ok(())
}
```

```rust
use adaptemoji::AdaptiveEmojiConvert;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let img = image::open("./assets/examples/original.webp")?;
    let mut resized_img = img
        .resize_to_fill(100, 100, image::imageops::FilterType::Triangle) // Resize image to 100px x 100px
        .to_luma_alpha8();

    adaptemoji::convert_adaptive(&mut resized_img, true);
    resized_img.save("./target/adaptive_negative.png")?;

    Ok(())
}
```
