pub trait BStrParse {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error>;
}

impl BStrParse for [u8] {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error> {
        lexical::parse(self)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    line_length: usize,
    line_count: usize,
}

impl<T, I2: ExactSizeIterator<Item = T>> std::iter::FromIterator<I2> for Grid<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = I2>>(iter: I) -> Self {
        let mut iter = iter.into_iter().peekable();

        let (low, high) = iter.size_hint();
        let mut data = Vec::with_capacity(high.map_or(low, |x| x));

        let line_length = iter.peek().unwrap().len();

        // This is significantly faster than data.extend(i.flatten()). Dunno why.
        for i in iter {
            data.extend(i);
        }

        debug_assert!(data.len() % line_length == 0);
        let line_count = data.len() / line_length;

        Self {
            data,
            line_length,
            line_count,
        }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T: Clone> Grid<T> {
    #[inline]
    pub fn from_value(value: T, line_length: usize, line_count: usize) -> Self {
        Self {
            line_length,
            line_count,
            data: vec![value; line_length * line_count],
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<&[T]> {
        if i < self.line_count {
            let start = i * self.line_length;
            unsafe { Some(self.data.get_unchecked(start..start + self.line_length)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut [T]> {
        if i < self.line_count {
            let start = i * self.line_length;
            unsafe { Some(self.data.get_unchecked_mut(start..start + self.line_length)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'_ [T]> {
        self.data.chunks_exact(self.line_length)
    }

    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = &'_ T> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut [T]> {
        self.data.chunks_exact_mut(self.line_length)
    }

    #[inline]
    pub fn flat_iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        self.data.iter_mut()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.line_count
    }

    #[inline]
    pub fn line_length(&self) -> usize {
        self.line_length
    }
}
