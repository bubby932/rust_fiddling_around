use std::fs;

fn main() {
    println!("get trolled bobert");
    let mut i = 0;
    let count = 5000000;
    let mut contents : String =  fs::read_to_string("./test.txt").unwrap();
    while i < count {
        if i % 50 != 0 {
            contents = format!("{} {}", contents, i);
        } else {
            contents = format!("{}\n{}", contents, i);
        }
        i = i+1;
    }
    let _unused = fs::write("./test.txt", contents);
    println!("Wrote every absurd number between 0 and {} to './test.txt'.", count);
}