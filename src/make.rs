use {crate::source::Source, image::RgbaImage};

pub fn make<S>(source: S, (width, height): (u32, u32)) -> RgbaImage
where
    S: Source + Sync,
{
    use rayon::prelude::*;

    const N_CHANNELS: usize = 4;

    let mut buf = vec![0; width as usize * height as usize * N_CHANNELS];

    (0..)
        .zip(buf.chunks_mut(N_CHANNELS))
        .par_bridge()
        .for_each(|(n, chunk)| {
            let x = n % width;
            let y = n / height;
            let color = source.source((x as _, y as _)).into_byte_array();
            chunk.copy_from_slice(&color);
        });

    RgbaImage::from_raw(width, height, buf).expect("the buffer fits")
}
