use rand::{rng, seq::SliceRandom};

// ez egy attribute itt hozzáadjuk a derive attributet (azt mondjuk a compiplernek hogy adjunk hozá pár további funkciót
// A Debug pedig egy trait. A traitek a functionok halmaza.
// tehát lényegében itt azt mondjuk hogy adjunk hozzá pár funkciót a debudgból pl kb olyat amitől lesztostringje a funkciónak odagenerálja
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// Inherent Implementations - funkciót adunk a structhoz, belül a típusra tudunk Self-el hivatkozni
impl Deck {
    // new egy associated function (class function) igazodik a struct definiciohoz
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck: Deck = Deck { cards: vec![] }; // nem kell kiírni köteletően a típust automatikusan felismeri
        // let deck = Deck { cards: Vec::new() }; // így is lehet újat létrehozni Vectorból de van rá mactro is
        Deck { cards } // Implicit return: return Deck { cards }; rövid formája: utolsó sor egy functionon belül pontosvessző nélkül
    }

    // Method, művelet egy class instancen (az első argumentuma a &self)
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    // deck.shuffle();
    // Probably need to add error handling!!!
    let cards = deck.deal(3);
    print!("Heres your hand: {:#?}", cards);

    // println!("Heres your deck: {deck}"); így is lehet
    println!("Heres your deck: {:#?}", deck); // :? egy formatter testreszabja hogyan printeljük az értéket ez egy debug formatter

}
