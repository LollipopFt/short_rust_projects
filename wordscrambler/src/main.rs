fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("error.");
    let a = scrambler(input);
    println!("{}", a);
}

fn scrambler(stri: String) -> String {
    let mut capital: bool = true;
    let list: Vec<char> = stri.to_lowercase().trim().chars().collect();
    println!("{:?}", list);
    let mut scrambledstr: String = "".to_string();
    for i in list {
        let istr: String = i.to_string();
        match i {
            'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z' => {
                if capital {
                    scrambledstr+=&istr.to_uppercase();
                    capital = false;
                } else {
                    scrambledstr+=&istr;
                    capital = true;
                }
            },
            _ => scrambledstr+=&istr,
        }
    }
    scrambledstr
}
