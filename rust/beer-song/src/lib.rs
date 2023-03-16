pub fn verse(n: u32) -> String {
    if 0 == n {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    if 1 == n {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n",
        n, n, n - 1, if n - 1 == 1 { "" } else { "s" })
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {start} to {end}, inclusive")
}
