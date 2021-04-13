const VERSES: [&'static str; 4] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
    "x bottles of beer on the wall, x bottles of beer.\nTake one down and pass it around, y bottles of beer on the wall.\n"
];

pub fn verse(n: u32) -> String {
    match n {
        0 | 1 | 2 => VERSES[n as usize].to_string(),
        // Unfortunately, due to format! being a macro, we can't use a dynamic fmt string, so we
        // hack around that by just using x and y since they don't occur in the sentence.
        // Figured I might as well code golf and make this interesting.
        n => VERSES[3]
            .chars()
            .map(|c| match c {
                'x' => format!("{}", n),
                'y' => format!("{}", n - 1),
                c => format!("{}", c),
            })
            .collect(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
