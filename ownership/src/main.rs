fn main() {
        let mut s1 = String::from("hello");

        tuples();
        //refprint();
        //change(&mut s1);
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
}

fn tuples() {
    struct Color(i32, i32, i32);
    let black = Color(0,0,0);
    println!("{} {} {}", black.0, black.1, black.2);
}
/*
fn refprint() {
    let mut s = String::from("hello");
    let t = String::from("hello");

    let r1 = s;
    let r2 = &t;
    let mut r3 = &t;
    r3.push_str(", world");
    //let r4 = s;
    //println!("This is r1: {} and this is r2: {} and this is r3: {} and this is r4: {}", r1, r2, r3, r4);
    println!("This is r1: {} and this is r2: {} this is t: {}, r3: {}", r1, r2, t, r3);
}
*/

fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
        s.len() // len() returns the length of a String
}

