use crate::{Color, Image};

pub trait Source {
    fn source(&self, pos: (i32, i32)) -> Color;

    fn borders(&self) -> Option<Borders> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct Borders {
    pub w: (i32, i32),
    pub h: (i32, i32),
}

impl Borders {
    pub fn contains(self, (x, y): (i32, i32)) -> bool {
        let Self {
            w: (x0, x1),
            h: (y0, y1),
        } = self;

        x >= x0 && x <= x1 && y >= y0 && y <= y1
    }
}

impl<S> Source for &S
where
    S: Source + ?Sized,
{
    fn source(&self, pos: (i32, i32)) -> Color {
        S::source(self, pos)
    }

    fn borders(&self) -> Option<Borders> {
        S::borders(self)
    }
}

impl<S> Source for Box<S>
where
    S: Source + ?Sized,
{
    fn source(&self, pos: (i32, i32)) -> Color {
        S::source(self, pos)
    }

    fn borders(&self) -> Option<Borders> {
        S::borders(self)
    }
}

impl<S> Source for [S]
where
    S: Source,
{
    fn source(&self, pos: (i32, i32)) -> Color {
        let mut res = Color::default();
        for source in self {
            let col = source.source(pos);
            res = res.overlay(col);
            if !res.is_transparent() {
                break;
            }
        }

        res
    }
}

impl<S> Source for Vec<S>
where
    S: Source,
{
    fn source(&self, pos: (i32, i32)) -> Color {
        self.as_slice().source(pos)
    }
}

impl Source for Color {
    fn source(&self, _: (i32, i32)) -> Self {
        *self
    }
}

impl Source for Image {
    fn source(&self, (x, y): (i32, i32)) -> Color {
        match (x.try_into(), y.try_into()) {
            (Ok(x), Ok(y)) => self.color((x, y)).unwrap_or_default(),
            _ => Color::default(),
        }
    }

    fn borders(&self) -> Option<Borders> {
        let (w, h) = self.size();
        Some(Borders {
            w: (0, w.saturating_sub(1) as _),
            h: (0, h.saturating_sub(1) as _),
        })
    }
}

pub struct Offset<S> {
    source: S,
    offset: (i32, i32),
}

impl<S> Offset<S> {
    pub fn new(source: S, offset: (i32, i32)) -> Self {
        Self { source, offset }
    }
}

impl<S> Source for Offset<S>
where
    S: Source,
{
    fn source(&self, (x, y): (i32, i32)) -> Color {
        self.source.source({
            let (dx, dy) = self.offset;
            (x.wrapping_sub(dx), y.wrapping_sub(dy))
        })
    }

    fn borders(&self) -> Option<Borders> {
        self.source.borders().map(
            |Borders {
                 w: (x0, x1),
                 h: (y0, y1),
             }| {
                let (dx, dy) = self.offset;
                Borders {
                    w: (x0.wrapping_sub(dx), x1.wrapping_sub(dx)),
                    h: (y0.wrapping_sub(dy), y1.wrapping_sub(dy)),
                }
            },
        )
    }
}

pub struct Scale<S> {
    source: S,
    factor: f32,
    filter: Filter,
}

impl<S> Scale<S> {
    /// The [`Scale`] constructor.
    ///
    /// # Panics
    /// Panics when a `factor` is less than or equal to zero.
    pub fn new(source: S, factor: f32, filter: Filter) -> Self {
        assert!(
            factor > f32::EPSILON,
            "factor cannot be less than or equal to zero"
        );

        Self {
            source,
            factor: 1. / factor,
            filter,
        }
    }
}

impl<S> Source for Scale<S>
where
    S: Source,
{
    fn source(&self, (x, y): (i32, i32)) -> Color {
        use std::cmp::Ordering;

        fn linear_points(v: f32) -> (i32, i32, f32) {
            let a = v as i32;
            let f = v.abs().fract();
            match f.total_cmp(&0.5) {
                Ordering::Less => (a, a.wrapping_sub(1), 0.5 - f),
                Ordering::Equal => (a, a, 0.),
                Ordering::Greater => (a, a.wrapping_add(1), f - 0.5),
            }
        }

        let x = self.factor * x as f32;
        let y = self.factor * y as f32;

        match self.filter {
            Filter::Near => self.source.source((x as _, y as _)),
            Filter::Linear => {
                let (x0, x1, xt) = linear_points(x);
                let (y0, y1, yt) = linear_points(y);

                match (x0 == x1, y0 == y1) {
                    (true, true) => self.source.source((x0, y0)),
                    (false, true) => {
                        let a = self.source.source((x0, y0));
                        let b = self.source.source((x1, y0));
                        a.lerp(b, xt)
                    }
                    (true, false) => {
                        let a = self.source.source((x0, y0));
                        let b = self.source.source((x0, y1));
                        a.lerp(b, yt)
                    }
                    (false, false) => {
                        let c0 = self
                            .source
                            .source((x0, y0))
                            .lerp(self.source.source((x1, y0)), xt);

                        let c1 = self
                            .source
                            .source((x0, y1))
                            .lerp(self.source.source((x1, y1)), xt);

                        c0.lerp(c1, yt)
                    }
                }
            }
        }
    }

    fn borders(&self) -> Option<Borders> {
        self.source.borders().map(
            |Borders {
                 w: (x0, x1),
                 h: (y0, y1),
             }| Borders {
                w: (
                    (x0 as f32 / self.factor) as _,
                    (x1 as f32 / self.factor) as _,
                ),
                h: (
                    (y0 as f32 / self.factor) as _,
                    (y1 as f32 / self.factor) as _,
                ),
            },
        )
    }
}

pub enum Filter {
    Near,
    Linear,
}

pub struct Blur<S> {
    source: S,
    radius: i32,
}

impl<S> Blur<S> {
    pub fn new(source: S, radius: u8) -> Self {
        Self {
            source,
            radius: radius as i32,
        }
    }
}

impl<S> Source for Blur<S>
where
    S: Source,
{
    fn source(&self, (x, y): (i32, i32)) -> Color {
        let mut col = Color::default();

        if let Some(borders) = self.borders() {
            if !borders.contains((x, y)) {
                return col;
            }
        }

        let radius = self.radius;
        let rsqr = radius * radius;
        let mut i = 0;
        for dy in -radius..radius {
            for dx in -radius..radius {
                if dx * dx + dy * dy < rsqr {
                    col += self.source.source((x + dx, y + dy));
                    i += 1;
                }
            }
        }

        if i > 0 {
            col *= 1. / i as f32;
        }

        col
    }

    fn borders(&self) -> Option<Borders> {
        self.source.borders().map(
            |Borders {
                 w: (x0, x1),
                 h: (y0, y1),
             }| Borders {
                w: (x0.wrapping_sub(self.radius), x1.wrapping_add(self.radius)),
                h: (y0.wrapping_sub(self.radius), y1.wrapping_add(self.radius)),
            },
        )
    }
}
