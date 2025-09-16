/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(array: &mut [T]){
    if array.len() <= 1 {
        return;
    }
    match partition(array) {
        (left, right) => {
            sort(left);
            sort(&mut right[1..]);
        }
    }
}

fn partition<T: Ord + Clone>(array: &mut [T]) -> (&mut [T], &mut [T]) {
    let pivot = array[0].clone();
    let mut i = 1;
    for j in 1..array.len() {
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(0, i - 1);
    array.split_at_mut(i - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
