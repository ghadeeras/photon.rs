use std::ops::Range;

pub struct PixelPositionIterator {
    new_line_delta: usize,
    columns: Range<usize>,
    rows: Range<usize>,
    position: PixelPosition,
}

#[derive(Clone)]
pub struct PixelPosition {
    pub column: usize,
    pub row: usize,
    pub linear: usize,
}

impl PixelPositionIterator {

    pub fn new(image_width: usize, columns: Range<usize>, rows: Range<usize>) -> Self {
        let position = PixelPosition {
            column: columns.start,
            row: rows.start,
            linear: rows.start * image_width + columns.start
        };
        Self {
            new_line_delta: image_width - (columns.end - columns.start),
            columns,
            rows,
            position
        }
    }

    fn width(&self) -> usize {
        self.columns.end - self.columns.start
    }

    fn height(&self) -> usize {
        self.rows.end - self.rows.start
    }

}

impl Iterator for PixelPositionIterator {

    type Item = PixelPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let p = &mut self.position;
        if p.row < self.rows.end {
            let result = Some(p.clone());
            p.column += 1;
            p.linear += 1;
            if p.column >= self.columns.end {
                p.column = self.columns.start;
                p.row += 1;
                p.linear += self.new_line_delta;
            }
            result
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = self.width() * self.height();
        (s, Some(s))
    }

}
