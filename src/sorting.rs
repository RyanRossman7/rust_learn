fn main() {

    let mut X: Vec<i32> = vec!(2, 11, 98, 23, 48, 33, 97, 61, 3);

    insertion_sort(&mut X); 
    println!("{:?}", X); 

}

pub(crate) fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let mut j = i; 
        while j > 0 && arr [j] < arr[j-1] {
            arr.swap(j, j-1); 
            j = j-1;
        }
        println!("{:?}", arr)
    }

}