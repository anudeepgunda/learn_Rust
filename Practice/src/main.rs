fn main() {
    //let nums: &[u32] = &[2, 7, 11, 15];
    let mut numbers: Vec<u32> = Vec::new();
    numbers.push(2);
    numbers.push(7);
    numbers.push(11);
    numbers.push(15);
    let target: u32 = 9;
    // if target == nums[0] + nums[1] {
    //     println!("positions are 0 and 1");
    // } else if target == nums[1] + nums[2] {
    //     println!("positions are 1 and 2");
    // } else if target == nums[2] + nums[3] {
    //     println!("positions are 2 and 3");
    // }
    // let mut counter: usize = 0;
    // loop {
    //     if nums.len() <= counter + 1 {
    //         break;
    //     }
    //     let sum_of_numbers = nums[counter] + nums[counter + 1];
    //     if target == sum_of_numbers {
    //         println!("positions are {} and {}", counter, counter + 1);
    //     }
    //     counter += 1;
    // }
    let index = numberindexessumoftarget(numbers, target);
    println!("sum of target indexes are:{:?}", index);
}

fn numberindexessumoftarget(numbers: Vec<u32>, target: u32) -> Vec<u32> {
    let mut counter: usize = 0;
    let mut arrayindex: usize;
    let mut indexes: Vec<u32> = Vec::new();
    loop {
        if numbers.len() <= counter + 1 {
            return indexes;
        }
        arrayindex = counter + 1;
        let sum_of_numbers = numbers[counter] + numbers[arrayindex];
        if sum_of_numbers == target {
            println!("positions are {} and {}", counter, counter + 1);
            indexes.push(counter as u32);
            indexes.push(arrayindex as u32);
        }
        counter += 1;
    }
}
