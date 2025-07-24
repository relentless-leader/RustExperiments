fn main() {
    let sorted_items: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("sortedItems: {:?}", sorted_items);
    let find_value = 2;
    let mut left = 0;
    let mut right = sorted_items.len()-1;
    loop {
        let mid = (left + right)/2;
        if sorted_items[mid] > find_value {
            right = mid - 1;
        } else if sorted_items[mid] == find_value {
            println!("Found the value at {}", mid+1);
            break;
        } else {
            left = mid + 1;
        }
    }
}
