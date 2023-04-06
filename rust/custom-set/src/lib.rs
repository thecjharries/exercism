#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(Vec<T>);

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut set = CustomSet(Vec::new());
        for element in _input {
            set.add(element.clone());
        }
        set
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.0.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.0.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.0.iter().all(|element| _other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
