use std::mem::swap;
use std::sync::Arc;

pub use bloom::*;
pub use gaussian::*;
pub use kernel1d::*;
pub use kernel2d::*;

use crate::basic::colors::Color;
use crate::imaging::Image;

mod kernel1d;
mod kernel2d;
mod bloom;
mod gaussian;

pub type SubFilters<'a> = Box<dyn Iterator<Item=&'a dyn AtomicImageFilter> + 'a>;

pub struct Composite<F: FilteringPipeLine>(pub F);

pub trait ImageFilter {

    fn filter(&self, input: &Image) -> Image;

    fn decompose(&self) -> SubFilters<'_>;

}

pub trait FilteringPipeLine where Self: Sized {

    fn components(&self) -> SubFilters<'_>;

    fn to_filter(self) -> Composite<Self> {
        Composite(self)
    }

}

pub trait AtomicImageFilter {

    fn filter_into(&self, output: &mut Image, input: &Image);

}

pub trait Kernel {

    fn apply_at(&self, x: usize, y: usize, input: &Image) -> Color;

}

impl<F: FilteringPipeLine> ImageFilter for Composite<F> {

    fn filter(&self, input: &Image) -> Image {
        let mut iter = self.decompose();
        match iter.next() {
            None => input.clone(),
            Some(first) => {
                let mut temp1 = first.filter(input);
                match iter.next() {
                    None => temp1,
                    Some(second) => {
                        let mut temp2 = second.filter(&temp1);
                        let t1 = &mut temp1;
                        let t2 = &mut temp2;
                        let i = ping_pong(&mut iter, t1, t2);
                        if i == 0 { temp2 } else { temp1 }
                    }
                }
            }
        }
    }

    fn decompose(&self) -> SubFilters<'_> {
        let Composite(ref f) = self;
        f.components()
    }

}

fn ping_pong<'a>(iter: &'a mut SubFilters, mut t1: &'a mut Image, mut t2: &'a mut Image) -> i32 {
    let mut i = 0;
    for filter in iter {
        filter.filter_into(t1, t2);
        swap(&mut t1, &mut t2);
        i += 1;
    }
    i % 2
}

impl<F: AtomicImageFilter> ImageFilter for F {

    fn filter(&self, input: &Image) -> Image {
        let mut output = input.new_with_same_size();
        self.filter_into(&mut output, input);
        output
    }

    fn decompose(&self) -> SubFilters<'_> {
        Box::new([self as &dyn AtomicImageFilter].into_iter())
    }

}

impl<'a> ImageFilter for &'a dyn AtomicImageFilter {

    fn filter(&self, input: &Image) -> Image {
        let mut output = input.new_with_same_size();
        self.filter_into(&mut output, input);
        output
    }

    fn decompose(&self) -> SubFilters<'_> {
        Box::new([*self].into_iter())
    }

}

impl<K: Kernel> AtomicImageFilter for K {

    fn filter_into(&self, output: &mut Image, input: &Image) {
        for ref p in input.pixel_position_iterator() {
            output[p] = self.apply_at(p.column, p.row, input)
        }
    }

}

impl<F: ImageFilter + ?Sized> FilteringPipeLine for Box<F> {

    fn components(&self) -> SubFilters<'_> {
        self.as_ref().decompose()
    }

}

impl<F: ImageFilter + ?Sized> FilteringPipeLine for Arc<F> {

    fn components(&self) -> SubFilters<'_> {
        self.as_ref().decompose()
    }

}

impl<F: ImageFilter> FilteringPipeLine for Vec<F> {

    fn components(&self) -> SubFilters<'_> {
        Box::new(self.iter().flat_map(|f| f.decompose()))
    }

}

impl<F: ImageFilter> FilteringPipeLine for (F, u8) {

    fn components(&self) -> SubFilters<'_> {
        let &(ref filter, count) = self;
        Box::new((0..count).flat_map(|_| filter.decompose()))
    }

}
