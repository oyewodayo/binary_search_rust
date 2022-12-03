use std::cmp::Ordering;

pub fn binary_search<T: Ord>(array: &[T], item: &T) -> Option<usize> {
    //Note: The array must be sorted first to be able to apply binary search.
    let mut is_asc = true;
    if array.len() >1 {
        is_asc = array[0] < array[(array.len() - 1)];
    }
    /*
        - Get the first item position on the array to search 
        - Get the position of the last item on the array to search
    */
    let mut left = 0;
    let mut right = array.len();

    /*
        - Split the array into two and get the mid point of the array
        - Identify the position of the midpoint value of the array
    */
    while left < right {
        let mid = (left + (right -left))/2;
        // let mid = (left + right)/2; Same as above

        if is_asc {
            match item.cmp(&array[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),//This checks if the middle element is equal to the target
                Ordering::Greater => left = mid+1,
            }
        }
        else {
            match item.cmp(&array[mid]) {
                Ordering::Less => left = mid +1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right= mid
            }
        }
    }
    None
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&[], &"a");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn one_item() {
        let index = binary_search(&["a","b"], &"a");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&["a", "b", "c", "d", "google", "zoo"],&"a" );
        assert_eq!(index, Some(0));

        let index = binary_search(&[1, 2, 4, 5, 7, 9],&6 );
        assert_eq!(index, Some(4));
    }

}
fn main() {

    println!("Hello, world!");
}
