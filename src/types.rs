use crate::common::get_unexp_err_msg;
use crate::common::UNEXPECTED_ERROR_MESSAGE;
use anyhow::Context;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::fs::{self, File};
use std::io::{BufWriter, Cursor, Read};
use std::path::PathBuf;

const MAX_FILE_LEN_BYTES: u64 = 100000000;
const RESIZE_RATIO_UPPER_THRESHOLD: f32 = 3.0;

pub(crate) struct ValidFile {
    bytes: Vec<u8>,
}

impl ValidFile {
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl TryFrom<&str> for ValidFile {
    type Error = anyhow::Error;

    fn try_from(path_str: &str) -> Result<Self, Self::Error> {
        let file_metadata = std::fs::metadata(path_str).context("couldn't fetch file metadata")?;
        let path_buf = PathBuf::from(path_str);

        if path_buf.file_stem().is_none() {
            return Err(anyhow::anyhow!("provided file has no name"));
        }

        let file_len = file_metadata.len();

        if file_len > MAX_FILE_LEN_BYTES {
            return Err(anyhow::anyhow!(
                "file's size ({} bytes) exceeds the maximum supported by squish, which is {} bytes",
                file_len,
                MAX_FILE_LEN_BYTES
            ));
        }
        let bytes = std::fs::read(path_str).context(format!(
            "couldn't read file contents; {}",
            UNEXPECTED_ERROR_MESSAGE
        ))?;

        Ok(Self { bytes })
    }
}

pub(crate) struct ValidImage {
    image: DynamicImage,
    format: SquishImageFormat,
}

#[derive(Clone, Copy)]
pub(crate) enum SquishImageFormat {
    Jpeg,
    Png,
}

impl ValidImage {
    pub(crate) fn width(&self) -> u32 {
        self.image.width()
    }

    pub(crate) fn height(&self) -> u32 {
        self.image.height()
    }

    pub(crate) fn format(&self) -> SquishImageFormat {
        self.format
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        self.image.as_bytes()
    }

    pub(crate) fn size(&self) -> usize {
        self.image.as_bytes().len()
    }

    pub(crate) fn validate_resize_request(&self, resize_width: u32) -> anyhow::Result<()> {
        if resize_width == self.width() {
            return Err(anyhow::anyhow!(
                "resize width is the same as the input image's width",
            ));
        }

        let ratio = resize_width as f32 / self.width() as f32;
        if ratio > RESIZE_RATIO_UPPER_THRESHOLD {
            Err(anyhow::anyhow!(
            "resize width cannot be more than {} times the width of the input image (which is {})",
            RESIZE_RATIO_UPPER_THRESHOLD,
            self.width()
        ))
        } else {
            Ok(())
        }
    }

    fn get_squish_format(format: &ImageFormat) -> anyhow::Result<SquishImageFormat> {
        match format {
            ImageFormat::Png => Ok(SquishImageFormat::Png),
            ImageFormat::Jpeg => Ok(SquishImageFormat::Jpeg),
            _ => {
                let allowed_types_str: Vec<&str> = Vec::from([ImageFormat::Jpeg, ImageFormat::Png])
                    .iter()
                    .map(|t| t.to_mime_type())
                    .collect();

                Err(anyhow::anyhow!(format!(
                    "file format not supported; allowed types: [{}]",
                    allowed_types_str.join(", ")
                )))
            }
        }
    }

    fn get_image_format(&self) -> ImageFormat {
        match self.format {
            SquishImageFormat::Jpeg => ImageFormat::Jpeg,
            SquishImageFormat::Png => ImageFormat::Png,
        }
    }

    pub(crate) fn get_image_format_repr(&self) -> String {
        match self.format {
            SquishImageFormat::Jpeg => "JPEG".to_string(),
            SquishImageFormat::Png => "PNG".to_string(),
        }
    }

    pub(crate) fn get_resized_version(&self, width: u32) -> Self {
        let old_h = self.image.height() as f32;
        let ratio = width as f32 / self.width() as f32;
        let new_h = (old_h * ratio).ceil() as u32;

        let resized = self
            .image
            .resize(width, new_h, image::imageops::FilterType::Lanczos3);

        Self {
            image: resized,
            format: self.format,
        }
    }

    pub(crate) fn write_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("couldn't create target directory")?;
        };

        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.image
            .write_to(&mut writer, self.get_image_format())
            .context(format!(
                "couldn't write to output file; {}",
                UNEXPECTED_ERROR_MESSAGE
            ))?;

        Ok(())
    }
}

impl TryFrom<&mut File> for ValidImage {
    type Error = anyhow::Error;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .context(get_unexp_err_msg("couldn't read from file"))?;
        let c: &[u8] = contents.as_ref();
        ValidImage::try_from(c)
    }
}

impl TryFrom<(Vec<u8>, u32, u32)> for ValidImage {
    type Error = anyhow::Error;

    fn try_from(data: (Vec<u8>, u32, u32)) -> Result<Self, Self::Error> {
        let (bytes, width, height) = data;
        let maybe_image_buffer = image::RgbaImage::from_raw(width, height, bytes);

        let image = match maybe_image_buffer {
            Some(ib) => DynamicImage::ImageRgba8(ib),
            None => {
                return Err(anyhow::anyhow!(
                    "couldn't construct RGBA image from clipboard bytes"
                ));
            }
        };

        Ok(Self {
            image,
            // TODO: this a bit wonky; we're forcing the format to be PNG without any validations
            format: SquishImageFormat::Png,
        })
    }
}

impl TryFrom<&[u8]> for ValidImage {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .context(format!(
                "something went wrong; {}",
                UNEXPECTED_ERROR_MESSAGE
            ))?;

        let format = reader
            .format()
            .ok_or(anyhow::anyhow!("couldn't determine the image format",))?;

        let image = reader
            .decode()
            .context("couldn't decode bytes to a known image format")?;

        let squish_format = Self::get_squish_format(&format)?;

        Ok(Self {
            image,
            format: squish_format,
        })
    }
}

impl TryFrom<&ValidFile> for ValidImage {
    type Error = anyhow::Error;

    fn try_from(file: &ValidFile) -> Result<Self, Self::Error> {
        ValidImage::try_from(file.bytes())
    }
}
