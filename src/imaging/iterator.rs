pub struct PixelPositionIterator {
    width: u16,
    height: u16,
    column: u16,
    row: u16,
    linear: usize
}

impl PixelPositionIterator {

    pub fn new(width: u16, height: u16) -> Self {
        Self{
            width,
            height,
            column: 0,
            row: 0,
            linear: 0
        }
    }

}

impl Iterator for PixelPositionIterator {

    type Item = (u16, u16, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.height {
            let result = Some((self.column, self.row, self.linear));
            self.linear += 1;
            self.column += 1;
            if self.column >= self.width {
                self.column = 0;
                self.row += 1;
            }
            result
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let s = (self.width as usize) * (self.height as usize);
        (s, Some(s))
    }

}
