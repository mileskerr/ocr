use std::fs::File;
use super::err::*;
use crate::err;

pub struct Image {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
    info: png::OutputInfo,
}

pub fn load(filename: &'static str) -> Result<Image,BoxErr> {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open(filename)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
   
    let ct = info.color_type.clone();
    let mut image = Image {
        r: vec![],
        g: vec![],
        b: vec![],
        info: info
    };
    use png::ColorType;
    match ct {
        ColorType::Rgb => {
            for i in 0..buf.len() {
                match i % 3 {
                    0 => image.r.push(buf[i]),
                    1 => image.g.push(buf[i]),
                    2 => image.b.push(buf[i]),
                    _ => unreachable!()
                }
            }
            Ok(image)
        }
        ColorType::Rgba => {
            for i in 0..buf.len() {
                match i % 4 {
                    0 => image.r.push(buf[i]),
                    1 => image.g.push(buf[i]),
                    2 => image.b.push(buf[i]),
                    3 => (),
                    _ => unreachable!()
                }
            }
            Ok(image)
        }
        ColorType::Grayscale => {
            for i in 0..buf.len() {
                image.r.push(buf[i]);
                image.g.push(buf[i]);
                image.b.push(buf[i]);
            }
            Ok(image)
        }
        _=> {
            err!("unsupported color type")
        }
    }
}

pub fn write(image: Image, filename: &'static str) -> Result<(),BoxErr> {
    use std::path::Path;
    use std::io::BufWriter;

    let path = Path::new(filename);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.info.width, image.info.height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header()?;

    let mut data = vec![];
    for i in 0..image.r.len() {
        data.push(image.r[i]);
        data.push(image.g[i]);
        data.push(image.b[i]);
    }
    writer.write_image_data(&data)?; // Save
    Ok(())
}
