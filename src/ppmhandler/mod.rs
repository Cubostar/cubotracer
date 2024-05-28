use std::io::{BufReader, BufWriter, BufRead, Write};
use std::fs::File;
use nalgebra::Vector3;

pub struct PPMImage {
    dim: (usize, usize),
    data: Vec<Vec<Vector3<u32>>>, // vec of columns, data[x][y]
    maxval: u32,
}

impl PPMImage {
    pub fn new(width: usize, height: usize, maxval: u32) -> Self {
        PPMImage {
            dim: (width, height),
            data: vec![vec![Vector3::new(0, 0, 0); height]; width],
            maxval,
        }
    }

    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    pub fn darken(&self) -> Self {
        PPMImage {
            dim: self.dim,
            data: self.data.iter().map(|row| row.iter().map(|pix| pix / 2).collect()).collect(),
            maxval: self.maxval,
        }

    }

    pub fn lighten(&self) -> Self {
        PPMImage {
            dim: self.dim,
            data: self.data.iter().map(|row| row.iter().map(|pix| (pix * 2).map(|val| val.min(self.maxval))).collect()).collect(),
            maxval: self.maxval,
        }
    }

    pub fn fill(&self, color: Vector3<u32>) -> Self {
        PPMImage {
            dim: self.dim,
            data: self.data.iter().map(|row| row.iter().map(|_| color).collect()).collect(),
            maxval: self.maxval
        }
    }

    pub fn change_pixel(&mut self, x: usize, y: usize, color: Vector3<u32>) -> () {
        if x >= self.dim.0 || y >= self.dim.1 {
            panic!("Pixel ({}, {}) does not exist!", x, y);
        } else if color.x > self.maxval || color.y > self.maxval || color.z > self.maxval {
            panic!("Color value is too high!");
        }
        self.data[x][y] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Vector3<u32> {
        if x >= self.dim.0 || y >= self.dim.1 {
            panic!("Pixel ({}, {}) does not exist!", x, y);
        }
        self.data[x][y]
    }

    pub fn maxval(&self) -> u32 {
        self.maxval
    }

    pub fn gamma_correct(&mut self) -> () {
        self.data = self.data.iter().map(|row| row.iter().map(|pix| Vector3::<u32>::new(
            (pix.x as f32).sqrt() as u32, 
            (pix.y as f32).sqrt() as u32, 
            (pix.z as f32).sqrt() as u32)).collect()).collect();
    }
}

pub struct PPMHandler();

pub enum PPMType {
    P3,
    P6,
}


impl PPMHandler {
    pub fn read(&self, path: &str) -> std::io::Result<PPMImage> {
        let reader = BufReader::new(File::open(path)?);
        let lines = reader.lines();

        let mut ppm_type: Option<PPMType> = None;
        let mut width: Option<usize> = None;
        let mut height: Option<usize> = None;
        let mut maxval: Option<u32> = None;
        let mut image: PPMImage = PPMImage::new(0, 0, 0);
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut count: usize = 0;
        let mut r: Option<u32> = None;
        let mut g: Option<u32> = None;
        let mut b: Option<u32> = None;

        for line_result in lines {
            if let Ok(line) = line_result {
                if line.starts_with("#") {
                    continue;
                } else {
                    self.read_line(
                        &line, 
                        &mut ppm_type, 
                        &mut width, 
                        &mut height, 
                        &mut maxval, 
                        &mut image,
                        &mut x,
                        &mut y,
                        &mut count,
                        &mut r,
                        &mut g,
                        &mut b);
                }
            }
        }

        Ok(image)
    }

    fn read_line(&self, 
        line: &str, 
        ppm_type: &mut Option<PPMType>,
        width: &mut Option<usize> , 
        height: &mut Option<usize> ,
        maxval: &mut Option<u32> ,
        image: &mut PPMImage,
        x: &mut usize,
        y: &mut usize,
        count: &mut usize,
        r: &mut Option<u32>,
        g: &mut Option<u32>,
        b: &mut Option<u32>) -> () {
        for token in line.split_whitespace() {
            if ppm_type.is_none() {
                if token == "P3" {
                    *ppm_type = Some(PPMType::P3);
                } else if token == "P6" {
                    *ppm_type = Some(PPMType::P6);
                } else {
                    panic!("Error when reading .ppm file: ppm type can not be read");
                }
            } else if width.is_none() {
                if token.parse::<usize>().map_or(false, |_w| true) {
                    *width = Some(token.parse::<usize>().unwrap());
                } else {
                    panic!("Error when reading .ppm file: width can not be read");
                }
            } else if height.is_none() {
                if token.parse::<usize>().map_or(false, |_h| true) {
                    *height = Some(token.parse::<usize>().unwrap());
                } else {
                    panic!("Error when reading .ppm file: height can not be read");
                }
            } else if maxval.is_none() {
                if token.parse::<u16>().map_or(false, |_m| true) {
                    *maxval = Some(token.parse::<u32>().unwrap());
                    *image = PPMImage::new(width.unwrap(), height.unwrap(), maxval.unwrap());
                } else {
                    panic!("Error when reading .ppm file: maxval can not be read");
                }
            } else {
                if r.is_none() {
                    if token.parse::<u16>().map_or(false, |_r| true) {
                        *r = Some(token.parse::<u32>().unwrap());
                    } else {
                        panic!("Error when reading .ppm file: r value can not be read");
                    }
                } else if g.is_none() {
                    if token.parse::<u16>().map_or(false, |_g| true) {
                        *g = Some(token.parse::<u32>().unwrap());
                    } else {
                        panic!("Error when reading .ppm file: g value can not be read");
                    }
                } else if b.is_none() {
                    if token.parse::<u16>().map_or(false, |_g| true) {
                        *b = Some(token.parse::<u32>().unwrap());
                        // println!("{}, ({}, {})", *count, *x, *y);
                        image.change_pixel(
                            *x, 
                            *y, 
                            Vector3::<u32>::new(r.unwrap(), g.unwrap(), b.unwrap()));
                        *r = None;
                        *g = None;
                        *b = None;
                        if *count % width.unwrap() == width.unwrap() - 1 {
                            *x = 0;
                            *y += 1;
                        } else {
                            *x += 1;
                        }
                        *count += 1;
                    } else {
                        panic!("Error when reading .ppm file: b value can not be read");
                    }
                }
            }
        }
    }

    pub fn write(&self, 
        image: &PPMImage, 
        ppm_type: PPMType,
        path: String) -> std::io::Result<String> {
        let mut writer = BufWriter::new(File::create(path.to_owned() + ".ppm")?);
        match ppm_type {
            PPMType::P3 => {
                writer.write(b"P3 \n").expect("Error when writing .ppm file: writing magic number failed");
                writer.write((
                    image.dim().0.to_string() + 
                    " " + 
                    &image.dim().1.to_string() + 
                    " \n")
                    .as_bytes()).expect("Error when writing .ppm file: writing dimensions failed");
                writer.write((image.maxval().to_string() + " \n").as_bytes())
                    .expect("Error when writing .ppm file: writing max value failed");

                for y in 0..image.dim().1 {
                    for x in 0..image.dim().0 {
                        writer.write((" ".to_owned() + 
                            &image.pixel_at(x, y).x.to_string() + 
                            " \n").as_bytes())
                            .expect("Error when writing .ppm file: writing color value failed");
                        writer.write((" ".to_owned() + 
                            &image.pixel_at(x, y).y.to_string() + 
                            " \n").as_bytes())
                            .expect("Error when writing .ppm file: writing color value failed");
                        writer.write((" ".to_owned() + 
                            &image.pixel_at(x, y).z.to_string() + 
                            " \n").as_bytes())
                            .expect("Error when writing .ppm file: writing color value failed");
                    }
                }
            },
            PPMType::P6 => {
                writer.write(("P6 ".to_owned() + 
                    &image.dim().0.to_string() + " " +
                    &image.dim().1.to_string() + " " +
                    &image.maxval().to_string() + "\n").as_bytes())
                    .expect("Error when writing .ppm file: writing header failed");
                if image.maxval() < 256 {
                    for y in 0..image.dim().1 {
                        for x in 0..image.dim().0 {
                            writer.write_all(&(image.pixel_at(x, y).x as u8).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                            writer.write_all(&(image.pixel_at(x, y).y as u8).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                            writer.write_all(&(image.pixel_at(x, y).z as u8).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                        }
                    }
                } else {
                    for y in 0..image.dim().1 {
                        for x in 0..image.dim().0 {
                            writer.write_all(&(image.pixel_at(x, y).x as u16).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                            writer.write_all(&(image.pixel_at(x, y).y as u16).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                            writer.write_all(&(image.pixel_at(x, y).z as u16).to_be_bytes())
                                .expect("Error when writing .ppm file: failed to write pixel");
                        }
                    }

                }
            },
        }
        Ok(path)
    }
} 
