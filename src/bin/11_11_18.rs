/*
 * Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
 *
 * For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
 *
 * Bonus: Can you do this in one pass?
 */
extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
	const MIN: i32 = 1;
	const MAX: i32 = 26;
	const MAX_B: i32 = 51;
	println!("-=-=-=-=-");
	println!("Running sum_two_in_array");
	println!("-=-=-=-=-");

	let mut arr: [i32; 4] = [10, 15, 3, 7];
	let mut k: i32 = 17;
	let mut output: bool = sum_two_in_array(k, &arr);
	println!("{:?}", arr);
	println!("{}", k);
	println!("{}", output);
	println!("-=-=-=-=-");

	arr = [10, 15, 3, 6];
	k = 17;
	output = sum_two_in_array(k, &arr);
	println!("{:?}", arr);
	println!("{}", k);
	println!("{}", output);
	println!("-=-=-=-=-");
	let mut rng = thread_rng();
	let mut arr2: [i32; 10] = [0; 10];
	for _i in 1..101 {
		for j in 0..10 {
			arr2[j] = rng.gen_range(MIN, MAX);
		}
		k = rng.gen_range(MIN, MAX_B);
		output = sum_two_in_array(k, &arr2);
		println!("{:?}", arr2);
		println!("{}", k);
		println!("{}", output);
		println!("-=-=-=-=-");
	}
}

/*
 * Add all values to map pointing to frequency of each
 * run through keys and if the map contains k-key then return true
 * else false
 */

fn sum_two_in_array(k: i32, arr: &[i32]) -> bool {
	let mut frequency: HashMap<_, i32> = HashMap::new();
	let mut output: bool = false;
	for i in arr.iter() {
		let counter = frequency.entry(*i).or_insert(0);
		*counter += 1;
	}
	for key in frequency.keys() {
		if frequency.contains_key(&(k - key)) {
			if k - key == *key {
				if frequency.get(key).unwrap() > &1 {
					output = true
				}
			} else {
				output = true;
			}
		}
	}
	return output;
}
