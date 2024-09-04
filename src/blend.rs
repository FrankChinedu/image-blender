use crate::{io::SourceData, operations::BlendOperation};
use image::{ImageBuffer, Rgb};

impl SourceData {
    pub fn blend_images(&self, operation: impl BlendOperation) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let SourceData {
            width,
            height,
            image1,
            image2,
        } = self;

        //  create a new buffer that has the same size as input images, which will serve as our output data
        let mut buffer = ImageBuffer::new(*width as u32, *height as u32);

        // iterate over ll pixles in the output buffer, along with their coordinates

        for (x, y, output_pixel) in buffer.enumerate_pixels_mut() {
            let index = (y * *width as u32 + x) as usize;

            let pixel1 = image1[index];
            let pixel2 = image2[index];
            *output_pixel = Rgb::from(operation.perform_operation(pixel1, pixel2))
        }
        buffer
    }
}
