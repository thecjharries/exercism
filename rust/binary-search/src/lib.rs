pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len();
    while low < high {
        let mid = (low + high) / 2;
        if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if low < array.len() && array[low] == key {
        Some(low)
    } else {
        None
    }
}
