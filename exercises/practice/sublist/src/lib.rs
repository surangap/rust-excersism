#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Copy>(first_list: &[T], second_list: &[T]) -> Comparison {
    let find_sublist = |first: &[T], second: &[T]| {
      first.windows(second.len()).any(| item| item == second)
    };

    match (first_list.len(), second_list.len()) {
        (len1, len2) if len1 == len2 && first_list == second_list => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (len1, len2) if len1 > len2 => match find_sublist(first_list, second_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        (len1, len2) if len2 > len1 => match find_sublist(second_list, first_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        (_, _) => Comparison::Unequal,
    }
}
