enum ListElement {
    Text(String),
    Number(i32),
}
fn bencoding_string(input_string: &str) {
    print!("{}:{}", input_string.len(),input_string);
}
fn bencoding_integer(input_integer: i32) {
    print!("i{}e", input_integer);
}
fn bencoding_list(input_list: &Vec<ListElement>) {
    print!("l");
    for element in input_list {
        match element {
            ListElement::Number(num) => {
                bencoding_integer(*num);
            }
            ListElement::Text(text) => {
                bencoding_string(text);
            }
        }
    }
    print!("e");
}
fn main() {
    let my_string = "Ben";
    let my_integer = 10;
    let my_list: Vec<ListElement> = vec![
        ListElement::Text(String::from("A")),
        ListElement::Text(String::from("B")),
        ListElement::Number(1),
    ];
    bencoding_string(my_string);
    bencoding_integer(my_integer);
    bencoding_list(&my_list);
}
//3:Beni10el1:A1:Bi1ee
