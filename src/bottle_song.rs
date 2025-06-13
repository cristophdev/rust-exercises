pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    for i in 0..take_down {
        let first_line = format!(
            "{} green {} hanging on the wall,\n",
            string_for_num(start_bottles - i),
            bottles_for_num(start_bottles - i)
        );
        let second_line = first_line.clone();
      
        let third_line = match start_bottles - i - 1 {
            0 => "There'll be no green bottles hanging on the wall.".to_string(),
            1 => "There'll be one green bottle hanging on the wall.".to_string(),
            n => format!("There'll be {} green {} hanging on the wall.", string_for_num(n).to_lowercase(), bottles_for_num(n)),
        };
        song.push_str(&first_line);
        song.push_str(&second_line);
        song.push_str("And if one green bottle should accidentally fall,\n");
        song.push_str(&third_line);
        song.push_str("\n\n");
    }
    song
}


fn string_for_num(n: u32) -> String {
    match n {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1 => "One",
        0 => "No",
        _ => "what?"
    }.to_string()
}

fn bottles_for_num(n: u32) -> String {
    match n {
        1 => "bottle",
        _ => "bottles"
    }.to_string()
}