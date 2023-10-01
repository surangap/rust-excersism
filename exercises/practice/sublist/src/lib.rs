#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Copy>(first_list: &[T], second_list: &[T]) -> Comparison {
    // Equal
    if first_list == second_list {
        return Comparison::Equal;
    }

    let mut result = Comparison::Unequal;
    if first_list.len() > second_list.len() {
        // empty list is a superlist
        if second_list.is_empty() {
            return Comparison::Superlist;
        }

        first_list.iter().enumerate().for_each(|(pos, &x)| {
            if x == second_list[0] && pos + second_list.len() <= first_list.len() {
                if &first_list[pos..pos + second_list.len()] == second_list {
                    result = Comparison::Superlist;
                }
            }
        });
    } else {
        // empty list is a sublist
        if first_list.is_empty() {
            return Comparison::Sublist;
        }

        second_list.iter().enumerate().for_each(|(pos, &x)| {
            if x == first_list[0] && pos + first_list.len() <= second_list.len() {
                if &second_list[pos..pos + first_list.len()] == first_list {
                    result = Comparison::Sublist;
                }
            }
        });
    }

    result
}
