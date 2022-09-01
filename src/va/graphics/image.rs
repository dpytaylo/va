use std::{path::Path, io::BufWriter, fs::File};
use png::{BitDepth, ColorType, SourceChromaticities};

pub fn save_image<T>(path: T, data: &[u8], color_type: ColorType, width: u32, height: u32) -> anyhow::Result<()>
    where T: AsRef<Path>,
{
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        w, 
        width, 
        height,
    );

    encoder.set_color(color_type);
    encoder.set_depth(BitDepth::Eight);
    //encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
    //encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    //encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;
    writer.finish()?;

    Ok(())
}