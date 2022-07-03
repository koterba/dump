use std::collections::HashMap;

fn main() {
    for target in 8..12 {
        two_sum(target); // Function is being run with a different target each time
    }
}

fn two_sum(target: i32) {
    let nums = vec![5,6,4,7,3,8,2,9,1];
    let mut used_nums: HashMap<i32, usize> = HashMap::new(); // i32: number, usize: index
    
    println!("Given vector: {:?}\nTarget: {}\nHere are the combinations found:\n", nums, target);
    
    for (ind, num) in nums.iter().enumerate() {
        let required = target - num;
        
        let hash_check = used_nums.get(&required);
        
        if hash_check == None {
            used_nums.insert(*num, ind);
        } else {
            let index = *hash_check.unwrap();
            println!("{} + {} = {}", num, nums[index], target)
        }

    }
    println!("\n----------------------------\n");
}
