extern crate time;

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
    SECOND, MINUTE, HOUR, DAY, WEEK, MONTH, YEAR,
    JAN, FEB, MAR, APR, MAY, JUN, JUL, AUG, SEP, OCT, NOV, DEC,
    UTC,
    PLUS, MINUS
}

fn parse_timespec(tokens: Vec<Token>, current_time: time::Tm) -> time::Tm {
    let mut new_time = time::now();

    // Reset to zero for midnight/noon/teatime
    new_time.tm_hour = 0;
    new_time.tm_min = 0;
    new_time.tm_sec = 0;

    for token in tokens {
        match token {
            Token::NOW => {
                new_time = current_time;
            }
            Token::MIDNIGHT => {
                new_time.tm_hour = 0;
            }
            Token::NOON => {
                new_time.tm_hour = 12;
            }
            Token::TEATIME => {
                new_time.tm_hour = 16;
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

// fn increment_time(tokens: Vec<Token>) -> time::Tm {
//     inc_dec_time(decrement_tm, tokens)
// }
// 
// fn decrement_time(tokens: Vec<Token>) -> time::Tm {
//     inc_dec_time(increment_tm, tokens)
// }
// 
// fn inc_dec_time(inc_dec: fn(time::Tm, time::Duration) -> time::Tm, tokens:Vec<Token>) -> time::Tm {
//     return time::now();
// }
// 
// fn increment_tm(a: time::Tm, b: time::Duration) -> time::Tm {
//     return a + b;
// }
// 
// fn decrement_tm(a: time::Tm, b: time::Duration) -> time::Tm {
//     return a - b;
// }

fn tokenize (timespec: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let exprs = timespec.split(' ');
    for expr in exprs {
        match expr {
            "now" | "n" => {
                tokens.push(Token::NOW);
            }
            "midnight" => {
                tokens.push(Token::MIDNIGHT);
            }
            "noon" => {
                tokens.push(Token::NOON);
            }
            "teatime" => {
                tokens.push(Token::TEATIME);
            }
            "sun" | "sunday" => {
                tokens.push(Token::SUN);
            }
            "mon" | "monday" => {
                tokens.push(Token::MON);
            }
            "tue" | "tuesday" => {
                tokens.push(Token::TUE);
            }
            "wed" | "wednesday" => {
                tokens.push(Token::WED);
            }
            "thu" | "thursday" => {
                tokens.push(Token::TUE);
            }
            "fri" | "friday" => {
                tokens.push(Token::FRI);
            }
            "sat" | "saturday" => {
                tokens.push(Token::SAT);
            }
            "jan" | "january" => {
                tokens.push(Token::JAN);
            }
            "feb" | "february" => {
                tokens.push(Token::FEB);
            }
            "mar" | "march" => {
                tokens.push(Token::MAR);
            }
            "apr" | "april" => {
                tokens.push(Token::APR);
            }
            "may" => {
                tokens.push(Token::MAY);
            }
            "jun" | "june" => {
                tokens.push(Token::JUN);
            }
            "jul" | "july" => {
                tokens.push(Token::JUL);
            }
            "aug" | "august" => {
                tokens.push(Token::AUG);
            }
            "sep" | "september" => {
                tokens.push(Token::SEP);
            }
            "oct" | "october" => {
                tokens.push(Token::OCT);
            }
            "nov" | "november" => {
                tokens.push(Token::NOV);
            }
            "dec" | "december" => {
                tokens.push(Token::DEC);
            }
            "am" => {
                tokens.push(Token::AM);
            }
            "pm" => {
                tokens.push(Token::PM);
            }
            "seconds" | "second" | "sec" | "s" => {
                tokens.push(Token::SECOND);
            }
            "minutes" | "minute" | "min" => {
                tokens.push(Token::SECOND);
            }
            "hours" | "hour" | "hr" | "h" => {
                tokens.push(Token::HOUR);
            }
            "days" | "day" | "d" => {
                tokens.push(Token::DAY);
            }
            "weeks" | "week" | "wk" | "w" => {
                tokens.push(Token::WEEK);
            }
            "months" | "month" => {
                tokens.push(Token::MONTH);
            }
            "years" | "year" | "yr" | "y" => {
                tokens.push(Token::YEAR);
            }
            "+" => {
                tokens.push(Token::PLUS);
            }
            "-" => {
                tokens.push(Token::MINUS);
            }
            _ => {
            }
        }
    }
    return tokens;
}

#[no_mangle]
pub extern fn parsetime_int (timespec: &str) -> u32 {
    let time = parsetime(timespec);
    let epoch_time = format!("{}", time.strftime("%s").ok().unwrap());
    return epoch_time.parse::<u32>().ok().unwrap();
}

#[no_mangle]
pub extern fn parsetime (timespec: &str) -> time::Tm {
    let timespec_string = timespec.to_string();
    let now = time::now();

    let tokens = tokenize(timespec_string);
    let time = parse_timespec(tokens, now);

    return time;
}
