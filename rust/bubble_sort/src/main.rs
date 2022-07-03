fn main() {
    let mut nums = vec![5,6,4,7,3,8,2,9,1,0];
    bubble_sort(&mut nums);
    println!("Bubble sorting numbers: {:?}", nums);
    
    let mut vehicles = vec!["car", "bus", "plane", "bike"];
    bubble_sort(&mut vehicles);
    println!("Bubble sorting &str: {:?}", vehicles);
}

fn bubble_sort<T: std::cmp::PartialOrd>(vec: &mut Vec<T>) {
    for i in 0..(vec.len() - 1) {
        for j in 0..(vec.len() - i - 1) {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1);
            }
        }
    } 
}
