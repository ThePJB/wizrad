use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


pub struct ImageBuffer {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<(u8,u8,u8)>,
}

// should actually just use a vec u8 internally so no need to convert back and forth

impl ImageBuffer {
    pub fn new(w: usize, h: usize) -> ImageBuffer {
        ImageBuffer {
            w,
            h,
            pixels: vec![(0,0,0); w*h],
        }
    }
    pub fn set_px(&mut self, x: usize, y: usize, val: (u8, u8, u8)) {
        self.pixels[y*self.w + x] = val;
    }
    pub fn get_px(&self, x: usize, y: usize) -> (u8, u8, u8) {
        self.pixels[y*self.w + x]
    }
    
    pub fn new_from_file(path_str: &str) -> ImageBuffer {
        let decoder = png::Decoder::new(File::open(path_str).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        let mut bytes_idx = 0;
        // extra copy whatever idgaf
        let mut image_buffer = ImageBuffer::new(info.width as usize, info.height as usize);
        for j in 0..image_buffer.h {
            for i in 0..image_buffer.w {
                image_buffer.set_px(i, j, (bytes[bytes_idx], bytes[bytes_idx + 1], bytes[bytes_idx + 2]));
                bytes_idx += 3;
            }
        }
        image_buffer
    }
    pub fn dump_to_file(&self, path_str: &str) {
        let path = Path::new(path_str);
        let file = File::create(path).unwrap();
        let ref mut buf_writer = BufWriter::new(file);
    
        let mut data = vec![0u8; (3*self.w*self.h)];
        let mut data_index = 0;
        for px in self.pixels.iter() {
            data[data_index] = px.0;
            data_index += 1;
            data[data_index] = px.1;
            data_index += 1;
            data[data_index] = px.2;
            data_index += 1;
        }
    
        let mut encoder = png::Encoder::new(buf_writer, self.w as u32, self.h as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8));
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap(); // Save
    }
}

pub struct ImageBufferA {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<(u8,u8,u8,u8)>,
}

// should actually just use a vec u8 internally so no need to convert back and forth

impl ImageBufferA {
    pub fn new(w: usize, h: usize) -> ImageBufferA {
        ImageBufferA {
            w,
            h,
            pixels: vec![(0,0,0,0); w*h],
        }
    }
    pub fn set_px(&mut self, x: usize, y: usize, val: (u8, u8, u8, u8)) {
        self.pixels[y*self.w + x] = val;
    }
    pub fn get_px(&self, x: usize, y: usize) -> (u8, u8, u8, u8) {
        self.pixels[y*self.w + x]
    }
    
    pub fn new_from_file(path_str: &str) -> Option<ImageBufferA> {
        let result_file = File::open(path_str);
        if result_file.is_err() {return None};
        let file = result_file.unwrap();
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        let mut bytes_idx = 0;
        // extra copy whatever idgaf
        let mut image_buffer = ImageBufferA::new(info.width as usize, info.height as usize);
        for j in 0..image_buffer.h {
            for i in 0..image_buffer.w {
                image_buffer.set_px(i, j, (bytes[bytes_idx], bytes[bytes_idx + 1], bytes[bytes_idx + 2], bytes[bytes_idx + 3]));
                bytes_idx += 4;
            }
        }
        Some(image_buffer)
    }
    // just copy lmao
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes_vec = Vec::new();
        for px in self.pixels.iter() {
            bytes_vec.push(px.0);
            bytes_vec.push(px.1);
            bytes_vec.push(px.2);
            bytes_vec.push(px.3);
        }
        bytes_vec
    }
    pub fn bytes_transpose(&self) -> Vec<u8> {
        let mut bytes_vec = Vec::new();
        for i in 0..self.w {
            for j in 0..self.h {
                let px = self.pixels[j*self.w + i];
                bytes_vec.push(px.0);
                bytes_vec.push(px.1);
                bytes_vec.push(px.2);
                bytes_vec.push(px.3);
            }
        }
        bytes_vec
    }
    pub fn dump_to_file(&self, path_str: &str) {
        let path = Path::new(path_str);
        let file = File::create(path).unwrap();
        let ref mut buf_writer = BufWriter::new(file);
    
        let mut data = vec![0u8; (4*self.w*self.h)];
        let mut data_index = 0;
        for px in self.pixels.iter() {
            data[data_index] = px.0;
            data_index += 1;
            data[data_index] = px.1;
            data_index += 1;
            data[data_index] = px.2;
            data_index += 1;
            data[data_index] = px.3;
            data_index += 1;
        }
    
        let mut encoder = png::Encoder::new(buf_writer, self.w as u32, self.h as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8)); // maybe dont need lol
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap(); // Save
    }
}