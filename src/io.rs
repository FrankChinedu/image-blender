use image::GenericImageView;

pub struct SourceData {
    pub width: usize,
    pub height: usize,
    pub image1: Vec<[u8; 3]>,
    pub image2: Vec<[u8; 3]>,
}

pub fn read_pixel_data(image1_path: String, image2_path: String) -> SourceData {
    // open the images
    let image1 = image::open(image1_path).unwrap();
    let image2 = image::open(image2_path).unwrap();

    // Compute image dimensions
    let (width, height) = image1.dimensions();
    let (width, height) = (width as usize, height as usize);

    // create arrays to hold input pixel data
    let mut image1_data: Vec<[u8; 3]> = vec![[0, 0, 0]; width * height];
    let mut image2_data: Vec<[u8; 3]> = vec![[0, 0, 0]; width * height];

    // Iterate over all pixels in the input image, along with their positions in x & y coordinates
    for (x, y, pixel) in image1.to_rgb8().enumerate_pixels() {
        // Compute the raw values for each channel in the RGB pixel.
        let [r, g, b] = pixel.0;

        // Compute linear index based on 2D index. This is basically computing index in
        // 1D array based on the row and column index of the pixel in the 2D image.
        let index = (y * (width as u32) + x) as usize;

        image1_data[index] = [r, g, b];
    }

    // Iterate over all pixels in the input image, along with their positions in x & y coordinates
    for (x, y, pixel) in image2.to_rgb8().enumerate_pixels() {
        // Compute the raw values for each channel in the RGB pixel.
        let [r, g, b] = pixel.0;

        // Compute linear index based on 2D index. This is basically computing index in
        // 1D array based on the row and column index of the pixel in the 2D image.
        let index = (y * (width as u32) + x) as usize;

        image2_data[index] = [r, g, b];
    }

    SourceData {
        width,
        height,
        image1: image1_data,
        image2: image2_data,
    }
}
