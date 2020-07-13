#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    None,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
            Suit::Hearts => "h",
            Suit::Spades => "s",
            Suit::None => "-",
        })
    }
}

impl Suit {
    /// Convert a Suit variant to its ordinal value
    pub fn to_ordinal(&self) -> u8 {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
            _ => 4,
        }
    }

    /// Convert an ordinal value to a ```Suit``` variant
    pub fn from_ordinal(suit: u8) -> Self {
        match suit {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => Suit::None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Joker => write!(f, "JOKER"),
            _ => write!(f, "{}", self.to_ordinal()),
        }
    }
}

impl Rank {
    /// Convert a Rank variant to its ordinal value
    pub fn to_ordinal(&self) -> u8 {
        match self {
            Rank::Ace => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7, 
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
            _ => 13,
        }
    }

    /// Convert an ordinal value to a ```Rank``` variant
    pub fn from_ordinal(rank: u8) -> Self {
        match rank {
            0 => Rank::Ace,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            8 => Rank::Nine,
            9 => Rank::Ten,
            10 => Rank::Jack,
            11 => Rank::Queen,
            12 => Rank::King,
            _ => Rank::Joker,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl Card {
    /// Construct a default card (Suit = ```Suit::Clubs```, Rank = ```Rank::Ace```)
    pub fn new() -> Self {
        Card::default()
    }

    /// Construct a card using ordinal values (0..3 for suit) and (0..12) for rank
    pub fn from_ordinals(suit: u8, rank: u8) -> Self {
        Card {
            suit: Suit::from_ordinal(suit),
            rank: Rank::from_ordinal(rank),
        }
    }

    /// Construct a card using rank and suit values
    pub fn from_suit_and_rank(suit: Suit, rank: Rank) -> Self {
        Card {
            suit,
            rank,
        }
    }

    pub fn set_rank(&mut self, rank: Rank) {
        self.rank = rank;
    }

    pub fn set_suit(&mut self, suit: Suit) {
        self.suit = suit;
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Suit, Rank};

    #[test]
    fn card_default() {
        let c = Card::new();
        assert_eq!(*c.rank(), Rank::Ace);
        assert_eq!(*c.suit(), Suit::Clubs);
    }

    #[test]
    fn card_from_ordinals() {
        assert_eq!(Card::from_ordinals(0, 12), Card::from_suit_and_rank(Suit::Clubs, Rank::King));
        assert_eq!(Card::from_ordinals(1, 11), Card::from_suit_and_rank(Suit::Diamonds, Rank::Queen));
        assert_eq!(Card::from_ordinals(2, 10), Card::from_suit_and_rank(Suit::Hearts, Rank::Jack));
        assert_eq!(Card::from_ordinals(3, 9), Card::from_suit_and_rank(Suit::Spades, Rank::Ten));
    }

    #[test]
    fn joker_from_ordinals() {
        assert_eq!(Card::from_ordinals(4, 13), Card::from_suit_and_rank(Suit::None, Rank::Joker));
        // ... in fact any invalid ordinals will generate a joker
        assert_eq!(Card::from_ordinals(12, 113), Card::from_suit_and_rank(Suit::None, Rank::Joker));
    }

    #[test]
    fn card_mutators_and_getters() {
        let mut c = Card::new();
        assert_eq!(c, Card::from_suit_and_rank(Suit::Clubs, Rank::Ace));
        
        c.set_suit(Suit::Hearts);
        assert_eq!(c, Card::from_suit_and_rank(Suit::Hearts, Rank::Ace));

        c.set_rank(Rank::Five);
        assert_eq!(c, Card::from_suit_and_rank(Suit::Hearts, Rank::Five));

        c.set_rank(Rank::Ten);
        assert_eq!(*c.rank(), Rank::Ten);

        c.set_suit(Suit::Clubs);
        assert_eq!(*c.suit(), Suit::Clubs);
    }

    #[test]
    fn suit_from_ordinal() {
        assert_eq!(Suit::from_ordinal(0), Suit::Clubs);
        assert_eq!(Suit::from_ordinal(1), Suit::Diamonds);
        assert_eq!(Suit::from_ordinal(2), Suit::Hearts);
        assert_eq!(Suit::from_ordinal(3), Suit::Spades);
        assert_eq!(Suit::from_ordinal(4), Suit::None);
    }

    #[test]
    fn suit_to_ordinal() {
        assert_eq!(Suit::Clubs.to_ordinal(), 0);
        assert_eq!(Suit::Diamonds.to_ordinal(), 1);
        assert_eq!(Suit::Hearts.to_ordinal(), 2);
        assert_eq!(Suit::Spades.to_ordinal(), 3);
        assert_eq!(Suit::None.to_ordinal(), 4);
    }

    #[test]
    fn rank_from_ordinal() {
        assert_eq!(Rank::from_ordinal(0), Rank::Ace);
        assert_eq!(Rank::from_ordinal(1), Rank::Two);
        assert_eq!(Rank::from_ordinal(2), Rank::Three);
        assert_eq!(Rank::from_ordinal(3), Rank::Four);
        assert_eq!(Rank::from_ordinal(4), Rank::Five);
        assert_eq!(Rank::from_ordinal(5), Rank::Six);
        assert_eq!(Rank::from_ordinal(6), Rank::Seven);
        assert_eq!(Rank::from_ordinal(7), Rank::Eight);
        assert_eq!(Rank::from_ordinal(8), Rank::Nine);
        assert_eq!(Rank::from_ordinal(9), Rank::Ten);
        assert_eq!(Rank::from_ordinal(10), Rank::Jack);
        assert_eq!(Rank::from_ordinal(11), Rank::Queen);
        assert_eq!(Rank::from_ordinal(12), Rank::King);
        assert_eq!(Rank::from_ordinal(13), Rank::Joker);
    }

    #[test]
    fn rank_to_ordinal() {
        assert_eq!(Rank::Ace.to_ordinal(), 0);
        assert_eq!(Rank::Two.to_ordinal(), 1);
        assert_eq!(Rank::Three.to_ordinal(), 2);
        assert_eq!(Rank::Four.to_ordinal(), 3);
        assert_eq!(Rank::Five.to_ordinal(), 4);
        assert_eq!(Rank::Six.to_ordinal(), 5);
        assert_eq!(Rank::Seven.to_ordinal(), 6);
        assert_eq!(Rank::Eight.to_ordinal(), 7);
        assert_eq!(Rank::Nine.to_ordinal(), 8);
        assert_eq!(Rank::Ten.to_ordinal(), 9);
        assert_eq!(Rank::Jack.to_ordinal(), 10);
        assert_eq!(Rank::Queen.to_ordinal(), 11);
        assert_eq!(Rank::King.to_ordinal(), 12);
        assert_eq!(Rank::Joker.to_ordinal(), 13);
    }
}
