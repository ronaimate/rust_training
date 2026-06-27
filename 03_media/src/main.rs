mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let autiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", autiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(autiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // match catalog.get_by_index(40) {
    //     Some(value) => {
    //         println!("{:#?}", value);
    //     }
    //     None => {
    //         println!("No value here!");
    //     }
    // }

    // if let Some(value) = catalog.get_by_index(9990) {
    //     println!("Item in pattern match: {:#?}", value);
    // } else {
    //     print!("No value!!!!");
    // }

    let item = catalog.get_by_index(40);

    // println!("{:#?}", item.unwrap());
    // println!("{:#?}", item.expect("expected there to be an item here!"));

    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));
}
