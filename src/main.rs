extern crate time;

use std::env;

#[derive(Clone, Debug)]
enum Token {
    DOTTEDDATE, HYPHENDATE,
    HOURMIN,
    INT1DIGIT, INT2DIGIT, INT4DIGIT, INT58DIGIT,
    INT, FLOAT,
    NOW,
    AM, PM,
    NOON, MIDNIGHT, TEATIME,
    SUN, MON, TUE, WED, THU, FRI, SAT,
    TODAY, TOMORROW,
    NEXT,
    MINUTE, HOUR, DAY, WEEK, MONTH, YEAR,
    JAN, FEB, MAR, APR, MAY, JUN, JUL, AUG, SEP, OCT, NOV, DEC,
    UTC,
    PLUS, MINUS
}

fn usage(name: String) {
    println!("Usage: {} [timespec]", name)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let result = parsetime(args[1].clone());
        println!("{}", result.asctime());
    } else {
        usage(args[0].clone());
    }
}

fn parse_timespec(tokens: Vec<Token>, current_time: time::Tm) -> time::Tm {
    let mut new_time = time::empty_tm();
    for token in tokens {
        match token {
            Token::NOW => {
                new_time = current_time;
            }
            Token::MIDNIGHT => {
                new_time.tm_hour = 0;
                new_time.tm_min = 0;
            }
            Token::NOON => {
                new_time.tm_hour = 12;
                new_time.tm_min = 0;
            }
            Token::TEATIME => {
                new_time.tm_hour = 16;
                new_time.tm_min = 0;
            }
            Token::SUN => {
                new_time.tm_wday = 0;
            }
            Token::MON => {
                new_time.tm_wday = 1;
            }
            Token::TUE => {
                new_time.tm_wday = 2;
            }
            Token::WED => {
                new_time.tm_wday = 3;
            }
            Token::THU => {
                new_time.tm_wday = 4;
            }
            Token::FRI => {
                new_time.tm_wday = 5;
            }
            Token::SAT => {
                new_time.tm_wday = 6;
            }
            Token::JAN => {
                new_time.tm_mon = 0;
            }
            Token::FEB => {
                new_time.tm_mon = 1;
            }
            Token::MAR => {
                new_time.tm_mon = 2;
            }
            Token::APR => {
                new_time.tm_mon = 3;
            }
            Token::MAY => {
                new_time.tm_mon = 4;
            }
            Token::JUN => {
                new_time.tm_mon = 5;
            }
            Token::JUL => {
                new_time.tm_mon = 6;
            }
            Token::AUG => {
                new_time.tm_mon = 7;
            }
            Token::SEP => {
                new_time.tm_mon = 8;
            }
            Token::OCT => {
                new_time.tm_mon = 9;
            }
            Token::NOV => {
                new_time.tm_mon = 10;
            }
            Token::DEC => {
                new_time.tm_mon = 11;
            }
            Token::PLUS => {
                new_time = current_time;
            }
            _ => println!("Unknown token: {:?}", token)
        }
    }
    return new_time;
}

fn increment_time(tokens: Vec<Token>) -> time::Tm {
    inc_dec_time(decrement_tm, tokens)
}

fn decrement_time(tokens: Vec<Token>) -> time::Tm {
    inc_dec_time(increment_tm, tokens)
}

fn inc_dec_time(inc_dec: fn(time::Tm, time::Duration) -> time::Tm, tokens:Vec<Token>) -> time::Tm {
    return time::now();
}

fn increment_tm(a: time::Tm, b: time::Duration) -> time::Tm {
    return a + b;
}

fn decrement_tm(a: time::Tm, b: time::Duration) -> time::Tm {
    return a - b;
}

fn tokenize (timespec: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let exprs = timespec.split(' ');
    for expr in exprs {
        match expr {
            "now" => {
                tokens.push(Token::NOW.clone());
            }
            "midnight" => {
                tokens.push(Token::MIDNIGHT.clone());
            }
            "noon" => {
                tokens.push(Token::NOON.clone());
            }
            "teatime" => {
                tokens.push(Token::TEATIME.clone());
            }
            "sun" | "sunday" => {
                tokens.push(Token::SUN.clone());
            }
            "mon" | "monday" => {
                tokens.push(Token::MON.clone());
            }
            "tue" | "tuesday" => {
                tokens.push(Token::TUE.clone());
            }
            "wed" | "wednesday" => {
                tokens.push(Token::WED.clone());
            }
            "thu" | "thursday" => {
                tokens.push(Token::TUE.clone());
            }
            "fri" | "friday" => {
                tokens.push(Token::FRI.clone());
            }
            "sat" | "saturday" => {
                tokens.push(Token::SAT.clone());
            }
            "jan" | "january" => {
                tokens.push(Token::JAN.clone());
            }
            "feb" | "february" => {
                tokens.push(Token::FEB.clone());
            }
            "mar" | "march" => {
                tokens.push(Token::MAR.clone());
            }
            "apr" | "april" => {
                tokens.push(Token::APR.clone());
            }
            "may" => {
                tokens.push(Token::MAY.clone());
            }
            "jun" | "june" => {
                tokens.push(Token::JUN.clone());
            }
            "jul" | "july" => {
                tokens.push(Token::JUL.clone());
            }
            "aug" | "august" => {
                tokens.push(Token::AUG.clone());
            }
            "sep" | "september" => {
                tokens.push(Token::SEP.clone());
            }
            "oct" | "october" => {
                tokens.push(Token::OCT.clone());
            }
            "nov" | "november" => {
                tokens.push(Token::NOV.clone());
            }
            "dec" | "december" => {
                tokens.push(Token::DEC.clone());
            }
            "+" => {
                tokens.push(Token::PLUS.clone());
            }
            _ => {
            }
        }
    }
    return tokens;
}

fn parsetime (timespec: String) -> time::Tm {
    let now = time::now();

    let tokens = tokenize(timespec);
    let time = parse_timespec(tokens, now);

    return time;
}
