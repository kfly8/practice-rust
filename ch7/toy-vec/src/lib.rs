pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>() // Vecのtoy実装作るために、一時的にVecを使っているが簡単のためやむなし
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {} // 実装省略
}
