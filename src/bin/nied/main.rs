fn main() {
    use nied::{
        source::{Blur, Filter, Offset, Scale, Source},
        Image,
    };

    let source = {
        let im = image::open("./unknown.png").expect("open");
        Image::from_dynamic(im).expect("image")
    };

    let source = Scale::new(source, 0.1, Filter::Near);
    let blur = Blur::new(&source, 8);

    let source: &[Offset<&(dyn Source + Sync)>] = &[
        Offset::new(&blur, (200, 200)),
        Offset::new(&blur, (100, 100)),
        Offset::new(&blur, (400, 200)),
        Offset::new(&blur, (200, 400)),
    ];

    nied::make(source, (600, 600))
        .save("out.png")
        .expect("save image");
}
