use std::io;
use std::process::exit;
use rand::Rng;


fn charset () -> &'static [u8] {
    println!("1 => ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!\n\
    2 => ABCDEFGHIJKLMNOPQRSTUVWXYZ\n\
    3 => abcdefghijklmnopqrstuvwxyz\n\
    4 => ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\n\
    5 => 0123456789\n\
    6 => )(*&^%$#@!\n\
    q => выход");
    println!("тип пароля:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input:&str = input.trim();
    let output:&'static[u8] = match input {
        "1" => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!",
        "2" => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "3" => b"abcdefghijklmnopqrstuvwxyz",
        "4" => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "5" => b"0123456789",
        "6" => b")(*&^%$#@!",
        "q" => exit(0x0000),
        _ => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!",
    };
    output
}

fn passwd_gen(col_passwd:u8, len_passwd:u8, charset_str:&[u8] ){
    let mut rng = rand::thread_rng();
    for _i in 0..col_passwd {
        let password: String = (0..len_passwd)

            .map(|_| {
                let idx = rng.gen_range(0..charset_str.len());

                charset_str[idx] as char
            })

            .collect();

        println!("Password: {}", password);
    }
}

fn main() {
    let mut len_passwd= String::new();
    println!("длина пароля:");
    io::stdin().read_line(&mut len_passwd).expect("ошибка чтения из консоли");
    let len_passwd: u8 = len_passwd.trim().parse().expect("нужно ввести целое число от 0 до 255");

    let charset_str = charset();

    println!("введите количество паролей");
    let mut col_passwd = String::new();
    io::stdin().read_line(&mut col_passwd).expect("ошибка чтения из консоли");
    let col_passwd: u8 = col_passwd.trim().parse().expect("нужно ввести целое число от 0 до 255");

    passwd_gen(col_passwd, len_passwd, charset_str);
}
