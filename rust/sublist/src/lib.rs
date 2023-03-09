#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        for i in 0.._first_list.len() {
            if _first_list[i] != _second_list[i] {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else {
        let mut larger = _first_list;
        let mut smaller = _second_list;
        if _first_list.len() < _second_list.len() {
            larger = _second_list;
            smaller = _first_list;
        }
        if 0 == smaller.len() {
            if larger == _first_list {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
        let window_size = smaller.len();
        for window in larger.windows(window_size) {
            let mut equal = true;
            let mut index = 0;
            while equal && index < window_size {
                if window[index] != smaller[index] {
                    equal = false;
                }
                index += 1;
            }
            if equal {
                if larger == _first_list {
                    return Comparison::Superlist;
                } else {
                    return Comparison::Sublist;
                }
            }
        }
        Comparison::Unequal
    }
}
