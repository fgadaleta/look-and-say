
fn compute_numbers(input: &Vec<i32>) -> Vec<i32>{
    let consec = consecutive_slices(input);
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


pub fn consecutive_slices(data: &[i32]) -> Vec<&[i32]> {
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
