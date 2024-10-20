fn main() {

    for i in 65500..65535 {
        match next_prime(i) {
            Some(prime) => println!("The next prime number after {} is {}", i, prime),
            None => println!("No prime number exists after {}", i),
        }
    }

    match checked_addition_with_result(7, 10) {
        Ok(result) => println!("Checked addition result: {}", result),
        Err(_) => println!("Overflow occurred in addition"),
    }

    match checked_multiplication_with_result(1000000, 425235) {
        Ok(result) => println!("Checked multiplication result: {}", result),
        Err(_) => println!("Overflow occurred in multiplication"),
    }

    let uppercase_result = to_uppercase('a');
    match uppercase_result {
        Ok(ch) => println!("Uppercase of 'a' is '{}'", ch),
        Err(e) => print_error(e),
    }

    let lowercase_result = to_lowercase('A');
    match lowercase_result {
        Ok(ch) => println!("Lowercase of 'A' is '{}'", ch),
        Err(e) => print_error(e),
    }

    match print_char('B') {
        Ok(ch) => println!("Printed character: {}", ch),
        Err(e) => print_error(e),
    }

    match char_to_number('3') {
        Ok(num) => println!("Character '3' to number: {}", num),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(num) => println!("Character 'F' to hex number: {}", num),
        Err(e) => print_error(e),
    }

    let ops = vec![('+', 10, 5), ('-', 10, 5), ('*', 10, 5), ('/', 10, 5), ('%', 10, 5)];
    for (op, a, b) in ops {
        match calculator(op, a, b) {
            Ok(result) => println!("Result of {} {} {} = {}", a, op, b, result),
            Err(e) => print_calculator_error(e),
        }
    }

    match calculator('/', 10, 0) {
        Ok(result) => println!("Result of division: {}", result),
        Err(e) => print_calculator_error(e),
    }

    match calculator('!', 10, 5) {
        Ok(result) => println!("Result of invalid operation: {}", result),
        Err(e) => print_calculator_error(e),
    }
    
    match checked_addition(52352, 500) {
        Some(result) => println!("Result of addition: {}", result),
        None => panic!("Overflow occured during addition"),
    }

    match checked_multiplication(34754242, 4351) {
        Some(result) => println!("Result of multiplication: {}", result), 
        None => panic!("Overflow occured during multiplication"),
    }

}

fn next_prime(x: u16) -> Option<u16>
{
    let mut y:u16 = x;
    let max_value:u16 = 65535;

    let result = loop {
        y = y + 1;
        if y == max_value {
            break None;
        }
        let mut ok:bool = true;
        if y % 2 == 0 {
            ok = false;
        }

        let mut d:u16 = 3;
        while d <= ((y as f64).sqrt()) as u16 {
            if y % d == 0 {
                ok = false;
                break;
            }
            d = d + 2;
        }

        if ok == true {
            break Some(y);
        }

    };

    result 
}

fn checked_addition(a:u32, b:u32) -> Option<u32>
{
    if a < u32::MAX - b {
        Some(a + b)
    }
    else {
        None 
        
    }       
}

fn checked_multiplication(a:u32, b:u32) -> Option<u32> {
    if a < u32::MAX / b {
        Some(a * b)
    }
    else {
        None 
    }
}

enum MyError {
    Overflow,
}

fn checked_multiplication_with_result(a:u32, b:u32) -> Result<u32, MyError> {
    if a < u32::MAX / b {
        Ok(a * b)
    }
    else {
        Err(MyError::Overflow)
    }
}


fn checked_addition_with_result(a:u32, b:u32) -> Result<u32, MyError>
{
    if a < u32::MAX - b {
        Ok(a + b)
    }
    else {
        Err(MyError::Overflow)
    }       
}

enum CharacterError {
    NotAscii(char),
    NotDigit(char),
    NotBase16Digit(char),
    NotLetter(char),
    NotPrintable(char),
}

fn to_uppercase(ch:char) -> Result<char, CharacterError> {
    if ch.is_alphabetic() {
        Ok(ch.to_ascii_uppercase())
    }
    else {
        Err(CharacterError::NotLetter(ch))
    }
}

fn to_lowercase(ch:char) -> Result<char, CharacterError> {
    if ch.is_alphabetic() {
        Ok(ch.to_ascii_lowercase())
    }
    else {
        Err(CharacterError::NotLetter(ch))
    }
}

fn print_char(ch:char) -> Result<char, CharacterError> {
    if ch.is_ascii_graphic() {
        {
            println!("{}", ch); 
            Ok(ch)
        }
    }
    else {
        Err(CharacterError::NotPrintable(ch))
    }
}

fn char_to_number(ch:char) -> Result<u32, CharacterError> {
    if ! ch.is_ascii() {
        Err(CharacterError::NotAscii(ch))
    }
    else if ! ch.is_digit(10) {
        Err(CharacterError::NotDigit(ch))
    }
    else {
        Ok(ch.to_digit(10).unwrap())
    }
}

fn char_to_number_hex(ch:char) -> Result<u32, CharacterError> {
     if ! ch.is_ascii() {
        Err(CharacterError::NotAscii(ch))
    }
    else if ! ch.is_digit(16) {
        Err(CharacterError::NotBase16Digit(ch))
    }
    else {
        Ok(ch.to_digit(16).unwrap())
    }
}

fn print_error(error : CharacterError) -> () {
    match error {
        CharacterError::NotAscii(ch) => {
            println!("The given character {} is not an ascii type character", ch)
        }, 
        CharacterError::NotLetter(ch) => {
            println!("The given character {} is not a letter type character", ch)
        }, 
        CharacterError::NotBase16Digit(ch) => {
            println!("The given character {} is not a hex type character", ch)
        }, 
        CharacterError::NotPrintable(ch) => {
            println!("The given character {} is not a printable type character", ch)
        }, 
        CharacterError::NotDigit(ch) => {
            println!("The given character {} is not a digit type character", ch)
        }, 
    }
}

enum CalcError {
    DivByZero(i64),
    Overflow(i64, i64, char), 
    InvalidOperation(char),
    NotDivisibile(i64, i64)
}

fn calculator(operation:char, operand1: i64, operand2:i64) -> Result<i64, CalcError> {
    match operation {
        '+' => {
            if (operand1 < i64::MAX - operand2) && (operand1 > i64::MIN + operand2) {
                Ok(operand1 + operand2)
            }
            else {
                Err(CalcError::Overflow(operand1, operand2, operation))
            }
        },
        '-' => {
            Ok(operand1 - operand2)  
        }, 
        '*' => {
            if operand1 > 0 && operand2 > 0 || operand1 < 0 && operand2 < 0 {
                if operand1 < i64::MAX / operand2 {
                    Ok(operand1 * operand2)
                }
                else {
                    Err(CalcError::Overflow(operand1, operand2, operation))
                }
            }
            else {
                if operand1 > i64::MIN / operand2 {
                    Ok(operand1 * operand2)
                }
                else {
                    Err(CalcError::Overflow(operand1, operand2, operation))
                }
            }
        },
        '/' => {
            if operand2 == 0 {
                Err(CalcError::DivByZero(operand1))
            }
            else {
                if operand1 % operand2 != 0 {
                    Err(CalcError::NotDivisibile(operand1, operand2))
                }
                else {
                    Ok(operand1 / operand2)
                }
            }
        }, 
        '%' => {
           if operand2 == 0 {
            Err(CalcError::DivByZero(operand1))
           } 
           else {
                Ok(operand1 % operand1)
           }
        } ,
        _ => {
            Err(CalcError::InvalidOperation(operation))
        }
    }
}

fn print_calculator_error(error:CalcError) {
    match error {
        CalcError::InvalidOperation(operation) => {
            println!("{} is not a valid mathematical operation!", operation);
        }, 
        CalcError::DivByZero(operand1) => {
            println!("Cannot divide {} by 0", operand1)
        },
        CalcError::NotDivisibile(operand1,operand2 ) => {
            println!("{} not divisible by {}", operand1, operand2)
        },
        CalcError::Overflow(operand1, operand2, operation) => {
            println!("{} between {} and {} causes overflow", operation, operand1, operand2)
        }
    }
}