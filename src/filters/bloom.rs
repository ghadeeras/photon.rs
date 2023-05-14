use crate::basic::colors::Color;
use crate::filters::{AtomicImageFilter, FilteringPipeLine, Gaussian, ImageFilter};
use crate::imaging::Image;

pub struct Bloom {
    pub half_size: u8,
    pub depth: u8,
}

impl AtomicImageFilter for Bloom {

    fn filter_into(&self, output: &mut Image, input: &Image) {
        let &Bloom { half_size, depth } = self;
        if depth == 0 {
            input.copy_to(output, input.pixel_position_iterator());
        }
        let gaussian = Gaussian { half_size, skirt_value: 1.0 / 256.0 };
        let filter = (gaussian.pipeline().to_filter(), depth).to_filter();
        let dimmed = input.map(|c, _, _| dim(c));
        let filtered = filter.filter(&dimmed);
        input.blend_into(output, &filtered, |c1, c2| sat(&c1) + c2)
    }

}

fn dim(color: &Color) -> Color {
    let l = color.luminance();
    if l > 1.0 {
        color * ((l - 1.0) / l)
    } else {
        Color::BLACK
    }
}

fn sat(color: &Color) -> Color {
    let l = color.luminance();
    if l > 1.0 {
        color / l
    } else {
        *color
    }
}