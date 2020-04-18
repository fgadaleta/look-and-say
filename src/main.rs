use std::char;


pub struct Looknumber {
    current: Vec<i32>, 
    next: Vec<i32>,
}

fn consecutive_slices(data: &[i32]) -> Vec<&[i32]> {
    let mut slice_start = 0;
    let mut result = Vec::new();

    for i in 1..data.len() {
        if data[i - 1] != data[i] {
            result.push(&data[slice_start..i]);
            slice_start = i;
        }
    }
    if data.len() > 0 {
        result.push(&data[slice_start..]);
    }
    result
}


fn compute_numbers(input: &Vec<i32>) -> Vec<i32>{
    let consec = consecutive_slices(&input);
    println!("consec: {:?}", consec);
    
    let mut vec = Vec::new();

    for el in consec.iter(){
        if el.len() > 1 {
            vec.push(el.len() as i32);
            vec.push(el[0] as i32);
        }

        if el.len() == 1 {
            vec.push(1 as i32);
            vec.push(el[0] as i32);
        }
    }
    vec
}

pub fn look_and_say(input: Vec<i32>, iter: u32) -> Vec<i32> {
    let mut this_input: Vec<i32> = input.to_owned();
    let mut sums: Vec<i32> = Vec::new();

	if input.len() == 0 {
		panic!("{:?} empty!", input);
    } 

    for i in 0.. iter{
        println!("Iteration {}/{}", i, iter);
        let consec = consecutive_slices(&this_input);
        let mut vec = Vec::new();

        for el in consec.iter(){
            if el.len() > 1 {
                vec.push(el.len() as i32);
                vec.push(el[0] as i32);
            }
            if el.len() == 1 {
                vec.push(1 as i32);
                vec.push(el[0] as i32);
            }
        }
        let previous_sum: i32 = vec.iter().sum();
        sums.push(previous_sum);
        this_input = vec;

    }

    sums
}


fn main() {
    println!("Calculating look numbers");
    let series = look_and_say(vec![1, 1], 25);
    println!("{:?}", series);
}