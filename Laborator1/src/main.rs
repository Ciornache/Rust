fn main() {
    // task1();
    // task2();
    // task3();
    // print!("{}", generate_nth_fibonnacci_number(10));
    // print!("{}", fahrenheit_to_celsius(100f64));
    sing_the_twelve_days_of_christmas();
}

fn task1() -> () {
    fn isprime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }

        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let mut i: i64 = 3;
        let mut ok: bool = true;

        while i * i < n {
            if n % i == 0 {
                ok = false;
            }
            i += 2;
        }

        return ok;
    }

    let mut i: i64 = 0;
    while i <= 100 {
        if isprime(i) == true {
            println!("The number {} is prime", i);
        }
        i += 1;
    }
}

fn task2() -> () {
    let (mut p1, mut p2) = (0, 0);
    while p1 <= 100 {
        while p2 <= 100 {
            fn cmmdc(x: i32, y: i32) -> i32 {
                if y == 0 {
                    return x;
                }
                return cmmdc(y, x % y);
            }
            if cmmdc(p1, p2) == 1 {
                println!("The numbers {} {} are coprime", p1, p2);
            }
            p2 += 1;
        }
        p1 += 1;
        p2 = 0;
    }
}

fn task3() -> () {
    let mut beers: i32 = 99;
    loop {
        println!(
            r#"{beers} bottles of beer on the wall
    {beers} bottles of beer
    Take one down, pass it around,"#
        );
        beers -= 1;
        if beers == 0 {
            break;
        }
        println!("{beers} bottles of beer on the wall.");
    }
    print!("No bottles of beer on the wall.");
}

fn generate_nth_fibonnacci_number(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }
    return generate_nth_fibonnacci_number(n - 1) + generate_nth_fibonnacci_number(n - 2);
}

fn fahrenheit_to_celsius(degree: f64) -> f64 {
    return (degree - 32f64) * 5f64 / 9f64;
}

fn sing_the_twelve_days_of_christmas() -> () {
    let items = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        let mut j: usize = i;
        while j > 0 {
            println!("{}", items[j]);
            j -= 1;
        }
        println!("{}\n", items[0]);
    }
}
