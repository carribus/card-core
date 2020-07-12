use std::collections::VecDeque;
use crate::cards::Card;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Default for Deck {
    /// Generate a deck of 52 cards, ordered by Suit (Clubs, Diamonds, Hearts, Spades) and rank (Ace to King).
    /// This method will not add jokers to the deck. Those must be added separately.
    fn default() -> Self {
        let mut deck = VecDeque::with_capacity(52);

        for i in 0..52 {
            deck.push_back(Card::from_ordinals(i/13, i%13));
        }

        Deck {
            cards: deck,
        }
    }
}

impl Deck {
    pub fn new() -> Self {
        Deck::default()
    }

    pub fn new_empty() -> Self {
        Deck {
            cards: VecDeque::with_capacity(52),
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;
    use crate::cards::{Rank, Suit};

    #[test]
    fn deck_default() {
        let mut d = Deck::new();

        assert_eq!(d.len(), 52);
        for i in (0..52).rev() {
            match d.draw() {
                Some(card) => {
                    let suit = card.suit().to_ordinal();
                    let rank = card.rank().to_ordinal();

                    assert_eq!(i, suit*13 + rank);
                },
                None => panic!("Should not have exhausted the deck!")
            }
        }

        assert_eq!(d.len(), 0);
    }

    #[test]
    fn empty_deck() {
        let d = Deck::new_empty();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn draw() {
        let mut d = Deck::new();
        let c = d.draw().unwrap();

        assert_eq!(d.len(), 51);
        assert_eq!(*c.suit(), Suit::Spades);
        assert_eq!(*c.rank(), Rank::King);

    }
}