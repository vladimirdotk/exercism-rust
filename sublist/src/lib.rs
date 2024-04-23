#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(list_a: &[T], list_b: &[T]) -> Comparison {
    if list_a == list_b {
        Comparison::Equal
    } else if is_sublist(list_a, list_b) {
        Comparison::Sublist
    } else if is_sublist(list_b, list_a) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    if small.is_empty() {
        true
    } else {
        large.windows(small.len()).any(|window| window == small)
    }
}
