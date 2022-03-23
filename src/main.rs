use text_io::read;
use ncurses::*;
use std::{thread, time};

fn main() {
    println!("Bubble Sort Visualizer");
    println!("Enter -1 to exit");
    let mut num_vec: Vec<i32> = Vec::new();
    let mut num_to_add: i32 = read!();
    loop {
    	if num_to_add == -1 {
    		break;
    	}
    	num_vec.push(num_to_add);
    	num_to_add = read!();
    }
    initscr();
    for i in 0..num_vec.len() - 1 {
    	for j in 0..num_vec.len() - i - 1 {
    		if num_vec[j] > num_vec[j + 1] {
    			let mut temp: i32 = num_vec[j];
    			num_vec[j] = num_vec[j + 1];
    			num_vec[j + 1] = temp;
    			let mut vec_str: String = String::new();
    			for k in 0..num_vec.len() {
    				vec_str = vec_str + &num_vec[k].to_string() + " ";
    			}
    			addstr(&vec_str);
    			refresh();
    			thread::sleep(time::Duration::from_millis(1000));
    			clear();
    		}
    	}
    }
    endwin();
    println!("The end");
}
