use std::{
    cmp::{max, max_by_key, min_by_key},
    fmt::Debug,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    let min_list: &[T] = min_by_key(first_list, second_list, |list: &&[T]| list.len());
    let max_list: &[T] = max_by_key(first_list, second_list, |list: &&[T]| list.len());
    let mut longest_subsequence: usize = 0;

    for m_ind in 0..(max_list.len() - min_list.len() + 1) {
        let mut subsequence: usize = 0;

        for s_ind in 0..min_list.len() {
            if min_list[s_ind] != max_list[m_ind + s_ind] {
                break;
            }
            subsequence += 1;
        }

        longest_subsequence = max(longest_subsequence, subsequence);
    }

    get_comparison(first_list.len(), second_list.len(), longest_subsequence)
}

fn get_comparison(
    first_list_size: usize,
    second_list_size: usize,
    longest_subsequence: usize,
) -> Comparison {
    if longest_subsequence == first_list_size && first_list_size == second_list_size {
        Comparison::Equal
    } else if longest_subsequence == first_list_size && first_list_size < second_list_size {
        Comparison::Sublist
    } else if longest_subsequence == second_list_size && first_list_size > second_list_size {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
