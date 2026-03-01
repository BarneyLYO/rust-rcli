use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

pub fn process_gen_pass(
    len: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    const ALL_STR_UPPER: &[u8; 24] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
    const ALL_STR_LOWER: &[u8; 24] = b"abcdefghijkmnpqrstuvwxyz";
    const DIGIT: &[u8; 9] = b"123456789";
    const SYMBOL: &[u8; 10] = b"!@#$%^&*_+";

    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(ALL_STR_UPPER);
        password.push(*ALL_STR_UPPER.choose(&mut rng).expect("chars wont be empty"));
    }
    if lower {
        chars.extend_from_slice(ALL_STR_LOWER);
        password.push(*ALL_STR_LOWER.choose(&mut rng).expect("chars wont be empty"));
    }
    if number {
        chars.extend_from_slice(DIGIT);
        password.push(*DIGIT.choose(&mut rng).expect("chars wont be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("chars wont be empty"));
    }

    if len > 1 {
        for _ in 0..(len - password.len() as u8) {
            let c = chars.choose(&mut rng).expect("chars wont be empty");
            password.push(*c); // u8 support copy, *c will do the copy?
        }
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);
    let result = zxcvbn(&password, &[]);
    println!("strength:{}", result.score());
    // TODO: make sure the password has at least one of each type
    Ok(())
}
