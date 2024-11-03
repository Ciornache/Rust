use serde_derive::Deserialize;
use std::{fs, io};

fn main() {
    solve_p1("file.txt");
    solve_p2();
    solve_p3("file_json.txt");
}

fn solve_p1(file_name: &str) -> Result<(), io::Error> {
    let s = fs::read_to_string(file_name).ok().unwrap();

    struct Student {
        name: String,
        phone_number: String,
        age: i32,
    }

    let (mut s1, mut s2) = (
        Student {
            name: String::from(" "),
            phone_number: String::from(" "),
            age: 0,
        },
        Student {
            name: String::from(" "),
            phone_number: String::from(" "),
            age: 100,
        },
    );

    for line in s.lines() {
        let mut it = line.split(',');
        if let (Some(name), Some(phone_number), Some(age)) = (it.next(), it.next(), it.next()) {
            let mut int_age: i32 = 0;
            for ch in age.chars() {
                int_age = int_age * 10 + (ch as u8 - '0' as u8) as i32;
            }
            if int_age > s1.age {
                s1 = Student {
                    name: String::from(name),
                    phone_number: String::from(phone_number),
                    age: int_age,
                };
            }
            if int_age < s2.age {
                s2 = Student {
                    name: String::from(name),
                    phone_number: String::from(phone_number),
                    age: int_age,
                };
            }
        }
    }

    println!(
        "The oldest student is: {} {} {}",
        s1.name, s1.phone_number, s1.age
    );
    println!(
        "The youngest student is: {} {} {}",
        s2.name, s2.phone_number, s2.age
    );
    Ok(())
}

struct Canvas {
    matrix: [[char; 100]; 55],
}

impl Canvas {
    fn set_pixel(&mut self, pixel: (usize, usize, char)) {
        if pixel.0 < 55 && pixel.1 < 100 {
            self.matrix[pixel.0][pixel.1] = pixel.2;
        }
    }
}

fn new_canvas() -> Canvas {
    let canvas = Canvas {
        matrix: [[' '; 100]; 55],
    };
    canvas
}

fn set_pixels(canvas: &mut Canvas, pixel_arr: &[(i32, i32, i32)]) -> () {
    for pixel in pixel_arr {
        canvas.set_pixel((pixel.0 as usize, pixel.1 as usize, pixel.2 as u8 as char));
    }
}

fn draw(c: &mut Canvas) -> () {
    set_pixels(c, &[(10, 62, 61), (27, 31, 61), (47, 75, 61)]);
    set_pixels(c, &[(45, 15, 64), (29, 67, 37), (25, 55, 61)]);
    set_pixels(c, &[(35, 39, 61), (24, 37, 61), (45, 52, 61)]);
    set_pixels(c, &[(25, 45, 61), (30, 65, 42), (37, 50, 61)]);
    set_pixels(c, &[(24, 24, 42), (25, 84, 61), (25, 86, 61)]);
    set_pixels(c, &[(48, 66, 43), (50, 84, 46), (37, 42, 35)]);
    set_pixels(c, &[(14, 53, 61), (15, 63, 64), (42, 77, 61)]);
    set_pixels(c, &[(47, 55, 61), (27, 76, 61), (38, 12, 64)]);
    set_pixels(c, &[(43, 83, 61), (38, 75, 61), (30, 87, 61)]);
}

fn print(canvas: Canvas) -> () {
    for row in canvas.matrix {
        for chr in row {
            print!("{}", chr)
        }
        println!("");
    }
}

fn solve_p2() {
    let mut canvas = new_canvas();
    draw(&mut canvas);
    print(canvas);
}

fn solve_p3(file_name: &str) -> () {
    let s = fs::read_to_string(file_name).ok().unwrap();

    #[derive(Debug, Deserialize)]
    struct Student {
        name: String,
        phone_number: String,
        age: i32,
    }

    let (mut s1, mut s2) = (
        Student {
            name: String::from(" "),
            phone_number: String::from(" "),
            age: 0,
        },
        Student {
            name: String::from(" "),
            phone_number: String::from(" "),
            age: 100,
        },
    );

    for line in s.lines() {
        let student: Student = serde_json::from_str(&line).unwrap();
        let (name, phone_number, age) = (student.name, student.phone_number, student.age);
      
            if age > s1.age {
                s1 = Student {
                    name: String::from(&name),
                    phone_number: String::from(&phone_number),
                    age: age,
                };
            }
            if age < s2.age {
                s2 = Student {
                    name: String::from(&name),
                    phone_number: String::from(&phone_number),
                    age: age,
                };
            }
    }

    println!(
        "The oldest student is: {} {} {}",
        s1.name, s1.phone_number, s1.age
    );
    println!(
        "The youngest student is: {} {} {}",
        s2.name, s2.phone_number, s2.age
    );
}
