
struct Image {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
    info: png::OutputInfo,
}

fn load_image(filename: &'static str) -> Result<Image,BoxErr> {
    use std::fs::File;
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
