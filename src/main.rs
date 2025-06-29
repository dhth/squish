mod common;
mod types;

use crate::types::{SquishImageFormat, ValidFile, ValidImage};
use anyhow::Context;
use arboard::Clipboard;
use clap::Parser;
use colored::Colorize;
use std::borrow::Cow;
use std::path::PathBuf;

const DEFAULT_RESIZE_WIDTH: u32 = 800;
const RESIZE_WIDTH_LOWER_THRESHOLD: u32 = 20;

/// squish lets you resize images via the command line
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// Local file path, or "cb" for system clipboard
    #[arg(value_name = "INPUT")]
    source: String,
    /// Width of resized image
    #[arg(short = 'w', long = "width", value_name = "INTEGER")]
    #[clap(default_value_t = DEFAULT_RESIZE_WIDTH)]
    width: u32,
    /// Destination of resized output file
    #[arg(short = 'o', long = "output-file", value_name = "FILE")]
    destination: Option<String>,
    /// Whether to copy resized image to clipboard (only supported for PNG images)
    #[arg(short = 'c', long = "copy-to-clipboard", value_name = "BOOLEAN")]
    copy_to_clipboard: bool,
    /// Blur strength
    #[arg(
        short = 'b',
        long = "blur-strength",
        value_name = "INTEGER",
        default_value = "0"
    )]
    blur_strength: u8,
    /// Whether to print updates
    #[arg(short = 'v', long = "verbose", default_value = "false")]
    verbose: bool,
    /// Whether to print address of output file in markdown format
    #[arg(short = 'm', long = "print-markdown-address", default_value = "false")]
    print_output_address_in_md: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // argument validations
    if args.destination.is_none() && !args.copy_to_clipboard {
        return Err(anyhow::anyhow!(
            "at least one destination (a local output file or the system clipboard) needs to be provided"
        ));
    }

    if args.width < RESIZE_WIDTH_LOWER_THRESHOLD {
        return Err(anyhow::anyhow!(
            "width must be greater than or equal to {}",
            RESIZE_WIDTH_LOWER_THRESHOLD
        ));
    }

    // image parsing and post-parsing validations
    let image = match args.source.as_str() {
        "cb" => {
            let mut clipboard = Clipboard::new().context("couldn't access system clipboard")?;
            let img_from_cboard = clipboard
                .get_image()
                .context("couldn't get image from clipboard")?;

            ValidImage::try_from((
                img_from_cboard.bytes.to_vec(),
                img_from_cboard.width as u32,
                img_from_cboard.height as u32,
            ))?
        }
        input_path => {
            let file = ValidFile::try_from(input_path)?;
            ValidImage::try_from(&file)?
        }
    };

    image.validate_resize_request(args.width)?;

    match (image.format(), args.copy_to_clipboard) {
        (SquishImageFormat::Png, _) => (),
        (_, true) => {
            return Err(anyhow::anyhow!(
                "copy to clipboard is only supported for PNG files"
            ));
        }
        _ => (),
    }

    if args.verbose {
        let msg = format!(
            "input image is {} px wide and {} px tall, and is of the format {}",
            image.width(),
            image.height(),
            image.get_image_format_repr()
        );
        print_message(msg.as_str());
    }

    // resizing
    let resized_image = image.get_resized_version(args.width, args.blur_strength);

    // outputs
    if args.verbose {
        let size_message = if args.width < image.width() {
            let size_reduction = (image.size() - resized_image.size()) as f32 / image.size() as f32;
            format!("size reduced by {:.2}%", size_reduction * 100.0)
        } else {
            let size_increase = (resized_image.size() - image.size()) as f32 / image.size() as f32;
            format!("size increased by {:.2}%", size_increase * 100.0)
        };

        let msg = format!(
            "resized image is {} px wide and {} px tall; {}",
            resized_image.width(),
            resized_image.height(),
            size_message,
        );
        print_message(msg.as_str());
    }

    if let Some(destination) = &args.destination {
        let d = PathBuf::from(&destination);
        resized_image.write_to_file(&d)?;
        if args.verbose {
            print_message(format!("written to \"{}\"", d.display()).as_str())
        }
    };

    if args.copy_to_clipboard {
        let resized_image_data = arboard::ImageData {
            width: resized_image.width() as usize,
            height: resized_image.height() as usize,
            bytes: Cow::from(resized_image.bytes()),
        };

        let mut clipboard = Clipboard::new().context("couldn't access system clipboard")?;
        clipboard
            .set_image(resized_image_data)
            .context("couldn't write resized image bytes to system clipboard")?;
        if args.verbose {
            print_message("resized image written to clipboard");
        }
    }

    if args.print_output_address_in_md {
        if let Some(dest) = &args.destination {
            println!("![image]({dest})");
        }
    }

    Ok(())
}

fn print_message(msg: &str) {
    println!("{}", msg.yellow());
}
