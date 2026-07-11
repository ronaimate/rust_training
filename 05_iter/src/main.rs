fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element)
    // }

    // for_each() is an iterator consumer, addig hívja next()-et a háttérben amíg nem kap vissza None-t
    elements
        .iter()
        // map() is an iterator adapter
        .map(|it| format!("{} {}", it, it))
        .for_each(|it| println!("{}", it))
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|it| it.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|it| it.to_uppercase()).collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|it| vec_b.push(it));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|it| it.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|it| it.contains(search))
        .map_or(String::from(fallback), |it| it.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors);

    // shorten_strings(&mut colors);
    // println!("{:#?}", colors)

    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased)

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded)

    let found_color = find_color_or(&colors, "asd", "Orange");
    println!("{}", found_color);

    // turbofish example: ha pontos típusmegadás collectnél amikor nem írjuk le pl a változó típusát / nem ezzel térünk vissza,
    // helyette: collect::<Vec<i32>>(); vagy collect::<Vec<_>>(); ilyenkor kitalálja a compiler
    // let balances = accounts
    //     .iter()
    //     .map(|account| account.balance)
    //     .collect::<Vec<i32>>();
    //
    // println!("Balances: {:#?}", balances);

    // itt is van iter-nek filter(), flat_map() stb...
    // https://doc.rust-lang.org/std/iter/struct.Filter.html
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
    // ...
}
