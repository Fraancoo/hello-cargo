fn main() {
    let_variable();
    mut_let_variable();
    shadowing();
    chars();
    parsing();
    tuples();
    tuple_structur();
    classic_structur();
    arrays();
}

fn let_variable() {
    println!("- - - - - let_variable - - - - -");
    let text: &str = "Hello World!";
    println!("Text: {}", text);
    let num: i32 = 664;
    println!("Number: {}", num);
    let deci: f64 = 1.5;
    println!("Float: {}", deci);
    let bool: bool = true;
    println!("Boolean {}", bool);
    // bool = false;
    // No es posible cambiar el valor
    // de una variable no mutable
}

fn mut_let_variable() {
    println!("- - - - - mut_let_variable - - - - -");
    let mut text: &str = "mundo";
    println!("Hello {}", text);
    // Las variables mutables sí
    // pueden cambiar su valor
    text = "world";
    println!("Hello {}", text);
}

fn shadowing() {
    println!("- - - - - shadowing - - - - -");
    // Puedes volver a crear una variable con
    // el mismo nombre para reemplazarla con
    // un valor nuevo, incluso su tipo de
    // valor puede cambiar
    let count: i32 = 1;
    println!("Count: {}", count);
    let count: i32 = count + 1;
    println!("Count: {}", count);
    let count: &str = "3";
    println!("Count: {}", count);
}

fn chars() {
    println!("- - - - - chars - - - - -");
    // Un caracter solo puede ser un
    // único item, pero este puede ser
    // cualquier representación UTF-8
    let char: char = 'Z';
    println!("{}", char);
    let char: char = '9';
    println!("{}", char);
    let char: char = '🦀';
    println!("{}", char);
}

fn parsing() {
    println!("- - - - - Parsing - - - - -");
    let num: i8 = "127".parse().expect("Not a valid number");
    println!("{num}")
}

fn tuples() {
    println!("- - - - - Tuples - - - - -");
    let mut tuple: (&str, u32, f64, bool) = ("Tuple", 664, 3.14, true);
    println!("{}", tuple.0);
    tuple.0 = "Hola";
    println!("{} {} {} {}", tuple.0, tuple.1, tuple.2, tuple.3);
}

fn tuple_structur() {
    println!("- - - - - Tuple Structur - - - - -");
    struct Profesor(u8, String, String, bool);

    let prof: Profesor = Profesor(1, String::from("Mario"), String::from("Castañeda"), true);

    println!("Profesor {}", prof.0);
    println!("First name: {}", prof.1);
    println!("Last name: {}", prof.2);
    println!("Status: {}", prof.3);
}

fn classic_structur() {
    println!("- - - - - Classic Structures - - - - -");
    struct User {
        id: u8,
        fname: String,
        lname: String,
        status: bool,
    }

    let user: User = User {
        id: 1,
        fname: String::from("Juan"),
        lname: String::from("Pablo"),
        status: true,
    };

    println!("User {}", user.id);
    println!("First name: {}", user.fname);
    println!("Last name: {}", user.lname);
    println!("Status: {}", user.status);
}

fn arrays() {
    let array: [i8; 3] = [1, 2, 3];
    println!("{} {} {}", array[0], array[1], array[2]);
}
