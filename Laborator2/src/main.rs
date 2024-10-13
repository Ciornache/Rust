fn main() {
    test_problem_1();
    test_problem_2();
    solve_problem_3();
}

fn add_chars_n(mut s: String, ch: char, nr: i32) -> String {
    for _ in 0..nr {
        s.push(ch);
    }
    s
}

fn test_problem_1() -> () {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
}

fn add_chars_n_v2(s: &mut String, ch: char, nr: i32) -> () {
    for _ in 0..nr {
        s.push(ch);
    }
}

fn test_problem_2() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_v2(&mut s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
}

fn solve_problem_3() {
    let mut s: String = String::from("");

    fn add_space(s: &mut String, nr: i32) -> () {
        add_chars_n_v2(s, ' ', nr);
    }

    fn add_str(s: &mut String, s2: &str) -> () {
        *s += s2;
    }

    fn add_integer(s: &mut String, mut nr: i32) -> () {

        if nr == 0 {
            add_chars_n_v2(s, '0', 1);
            return ();
        }

        let (copie, mut p) = (nr, 1);
        while nr > 0 {
            p *= 10;
            nr /= 10;
        }
        let mut sep: i32 = 0;
        nr = copie;
        while p > 1 {
            sep += 1;
            p /= 10;
            if sep > 1 && (sep - 1) % 3 == 0 {
                add_chars_n_v2(s, '_', 1);
            }
            add_chars_n_v2(s, (((nr / p) % 10) as u8 + '0' as u8) as char, 1);
        }
    }

    fn add_float(s: &mut String, fnr: f32) -> () {

        let intreg = fnr as i32;
        add_integer(s, intreg);
        add_chars_n_v2(s, '.', 1);
        let mut copie:f32 = fnr;
        while (copie - (copie as i32) as f32) > 0.01 {
            copie *= 10f32;
            add_integer(s, (copie as i32) % 10);
        }
    }

    add_space(&mut s, 46);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 46);
    add_str(&mut s, "RUST.\n");
    add_space(&mut s, "    ".len() as i32);
    add_str(&mut s, "Most");
    add_space(&mut s, "            ".len() as i32);
    add_str(&mut s, "crate");
    add_space(&mut s, "      ".len() as i32);
    add_integer(&mut s, 306_437_968);
    add_space(&mut s, "           ".len() as i32);
    add_str(&mut s, "and");
    add_space(&mut s, "     ".len() as i32);
    add_str(&mut s, "lastest");
    add_space(&mut s, "         ".len() as i32);
    add_str(&mut s, "is\n");
    add_space(&mut s, "         ".len() as i32);
    add_str(&mut s, "downloaded");
    add_space(&mut s, "        ".len() as i32);
    add_str(&mut s, "has");
    add_space(&mut s, "             ".len() as i32);
    add_str(&mut s, "downloads");
    add_space(&mut s, "     ".len() as i32);
    add_str(&mut s, "the");
    add_space(&mut s, "         ".len() as i32);
    add_str(&mut s, "version");
    add_space(&mut s, "    ".len() as i32);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");
    print!("{}", s);

}
