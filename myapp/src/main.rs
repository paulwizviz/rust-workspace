mod math;
use mylib::form_sentence;

fn main() {

    let sentence = form_sentence(&["Hello","World","and", "Earth"]);
    println!("The sentence is \"{}\"", sentence);

    let result = math::add(1,1);
    println!("1+1 = {}", result)
}
