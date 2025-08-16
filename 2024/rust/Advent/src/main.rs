#![feature(lock_value_accessors)]
#![feature(core_float_math)]
#![feature(string_remove_matches)]
#![feature(iter_next_chunk)]
#![feature(random)]
#![feature(iter_map_windows)]

mod d1;
mod d6;
mod d9;
mod d10;
mod d11;
mod d12;
mod d15;
mod d16;
mod d23;
mod d24;
mod d25;
mod matrix;


fn run() -> &'static str  {
    println!("Hello, world!");
    "Hello World"
    
}

fn main()  {
    println!("Hello, world!");
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn says_hello() {
        let r = run();
        assert_eq!(r, "Hello World")

    }

}


