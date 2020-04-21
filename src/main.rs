mod utils;


pub fn look_and_say(input: Vec<i32>, iter: u32) -> Vec<i32> {
    let mut this_input: Vec<i32> = input.to_owned();
    let mut sums: Vec<i32> = Vec::new();

	if input.len() == 0 {
		panic!("{:?} empty!", input);
    } 

    for i in 0.. iter{
        println!("Iteration {}/{}", i, iter);
        let consec = utils::consecutive_slices(&this_input);
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
    let series = look_and_say(vec![1, 1], 30);

    for i in 1..series.len() {
        println!("{} {}", i, series[i] as f32 / series[i-1] as f32);
    }

}