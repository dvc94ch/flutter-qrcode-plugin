use flutter_engine::texture_registry::Texture;
use flutter_plugins::prelude::*;
use image::{Bgr, ConvertBuffer, ImageBuffer};
use opencv::core::{Mat, Vec3b};
use opencv::objdetect::QRCodeDetector;
use opencv::videoio::{VideoCapture, CAP_ANY};

pub type Frame<'a> = ImageBuffer<Bgr<u8>, Vec<u8>>;

#[derive(Debug)]
pub enum Error {
    OpenCamera,
    OpenCv(opencv::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OpenCamera => write!(f, "unable to open default camera"),
            Self::OpenCv(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<opencv::Error> for Error {
    fn from(error: opencv::Error) -> Self {
        Self::OpenCv(error)
    }
}

impl From<Error> for MethodCallError {
    fn from(error: Error) -> Self {
        Self::from_error(error)
    }
}

pub struct QrCodeScanner {
    width: u32,
    height: u32,
    cam: VideoCapture,
    detector: QRCodeDetector,
    texture: Texture,
}

impl QrCodeScanner {
    pub fn new(texture: Texture) -> Result<Self, Error> {
        let mut cam = VideoCapture::new_with_backend(0, CAP_ANY)?;
        if !VideoCapture::is_opened(&cam)? {
            return Err(Error::OpenCamera);
        }
        let mut frame = Mat::default()?;
        cam.read(&mut frame)?;
        let size = frame.size()?;
        let detector = QRCodeDetector::default()?;
        Ok(Self {
            cam,
            detector,
            width: size.width as _,
            height: size.height as _,
            texture,
        })
    }

    pub fn frame(&mut self) -> Result<Option<String>, Error> {
        let mut frame = Mat::default()?;
        self.cam.read(&mut frame)?;

        let mut points = Mat::default()?;
        let mut qrcode = Mat::default()?;
        let result = self
            .detector
            .detect_and_decode(&frame, &mut points, &mut qrcode)
            .ok()
            .and_then(|code| {
                if !code.is_empty() {
                    Some(code.as_str().to_string())
                } else {
                    None
                }
            });

        let data = frame.data_typed::<Vec3b>()?;
        let data: &[u8] =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const _, data.len() * 3) };
        let frame = Frame::from_raw(self.width, self.height, data.to_vec()).unwrap();
        self.texture.post_frame_rgba(frame.convert());

        Ok(result)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
