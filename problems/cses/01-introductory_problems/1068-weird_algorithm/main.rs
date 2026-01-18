use std::io;

fn read() -> u64 { 
    io::stdin()
        .read_line(String::new()).expect("Failed to read")
        .trim().parse::<u64>().expect("Not a number")
} 

fn weird_algo(n: u64) {
    print!("{n} ");
    if n == 1 {
        return 
    }

    let next = if n % 2 == 0 { 3 * n + 1 } else { n / 2 }; 
    weird_algo(next);
}

fn main() {
    weird_algo(read());   
}