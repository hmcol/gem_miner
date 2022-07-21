use std::cmp::Ordering;

pub fn checked_add_signed(u: usize, i: isize) -> Option<usize> {
    let i_abs = i.checked_abs()?.try_into().ok()?;
    // let ix_abs = ix.checked_abs()? as usize; // i think this is the same but idk

    match i.cmp(&0) {
        Ordering::Less => u.checked_sub(i_abs),
        Ordering::Equal => Some(u),
        Ordering::Greater => u.checked_add(i_abs),
    }
}

#[test]
fn test_cast() {
    println!("  x  .   binary   . x as .  |x| . |x| as");

    for x in [-128, -127, -25, -2, -1, 0i8, 1, 2, 25, 126, 127] {
        let x_as = x as u8;
        let x_abs = x.checked_abs().unwrap_or(69);
        let x_abs_as = x_abs as u8;

        println!("{x:4} . ({x:0>8b}) . {x_as:4} . {x_abs:4} . {x_abs_as:4}");
    }
}
