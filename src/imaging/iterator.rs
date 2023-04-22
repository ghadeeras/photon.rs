pub struct PixelPositionIterator {
    new_line_delta: u16,
    columns: [u16; 2],
    rows: [u16; 2],
    column: u16,
    row: u16,
    linear: usize
}

impl PixelPositionIterator {

    pub fn new(image_width: u16, columns: [u16; 2], rows: [u16; 2]) -> Self {
        Self {
            new_line_delta: image_width - (columns[1] - columns[0]),
            columns,
            rows,
            column: columns[0],
            row: rows[0],
            linear: (rows[0] as usize) * (image_width as usize) + (columns[0] as usize)
        }
    }

    fn width(&self) -> u16 {
        self.columns[1] - self.columns[0]
    }

    fn height(&self) -> u16 {
        self.rows[1] - self.rows[0]
    }

}

impl Iterator for PixelPositionIterator {

    type Item = (u16, u16, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.rows[1] {
            let result = Some((self.column, self.row, self.linear));
            self.column += 1;
            self.linear += 1;
            if self.column >= self.columns[1] {
                self.column = self.columns[0];
                self.row += 1;
                self.linear += self.new_line_delta as usize;
            }
            result
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = (self.width() as usize) * (self.height() as usize);
        (s, Some(s))
    }

}
