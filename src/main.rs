mod blend;
mod io;
mod operations;

use io::*;

use operations::{
    AdditionBlend, AverageBlend, DarkenBlend, LightenBlend, MultiplyBlend, ScreenBlend,
    SubtrationBlend,
};

fn main() {
    let source_data = read_pixel_data("image1.jpg".to_string(), "image2.jpg".to_string());

    let output_buffer = source_data.blend_images(AdditionBlend);
    output_buffer.save("addition.jpg").unwrap();

    let output_buffer = source_data.blend_images(AverageBlend);
    output_buffer.save("average.jpg").unwrap();

    let output_buffer = source_data.blend_images(DarkenBlend);
    output_buffer.save("darken.jpg").unwrap();

    let output_buffer = source_data.blend_images(LightenBlend);
    output_buffer.save("lighten.jpg").unwrap();

    let output_buffer = source_data.blend_images(MultiplyBlend);
    output_buffer.save("multiply.jpg").unwrap();

    let output_buffer = source_data.blend_images(ScreenBlend);
    output_buffer.save("screen.jpg").unwrap();

    let output_buffer = source_data.blend_images(SubtrationBlend);
    output_buffer.save("subtraction.jpg").unwrap();
}
