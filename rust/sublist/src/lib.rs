#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        let mut equal = true;
        for i in 0.._first_list.len() {
            if _first_list[i] != _second_list[i] {
                equal = false;
                break;
            }
        }
        if equal {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
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
            for i in 0..window_size {
                if window[i] != smaller[i] {
                    equal = false;
                    break;
                }
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
