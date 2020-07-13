use std::cmp;
use std::ops::AddAssign;
use card_core::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HandTotal {
    pub hard_total: u8,
    pub soft_total: u8,
}

impl Default for HandTotal {
    fn default() -> Self {
        Self {
            hard_total: 0,
            soft_total: 0,
        }
    }
}

impl AddAssign for HandTotal {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            hard_total: self.hard_total + other.hard_total,
            soft_total: self.soft_total + other.soft_total,
        }
    }
}

impl HandTotal {
    pub fn get_best_total(&self) -> u8 {
        if self.hard_total > 21 {
            return cmp::min(self.hard_total, self.soft_total)
        }
        return cmp::max(self.hard_total, self.soft_total)        
    }
}

pub enum HandResult {
    DealerWins(bool),
    PlayerWins(bool),
    Push,
}

pub struct BlackjackEvaluator;

impl BlackjackEvaluator {
    pub fn hand_value(hand: &[Card]) -> HandTotal {
        let mut result = HandTotal::default();

        for card in hand.iter() {
            let card_value = Self::get_card_value(card);
            result += card_value;
        }

        result
    }

    pub fn compare_hands(player: &[Card], dealer: &[Card]) -> HandResult {
        let player_total = Self::hand_value(player);
        let dealer_total = Self::hand_value(dealer);

        // check for blackjack
        if player.len() == 2 && player_total.get_best_total() == 21 {
            if dealer.len() == 2 && dealer_total.get_best_total() == 21 {
                return HandResult::Push;
            }
            return HandResult::PlayerWins(true);
        } else if dealer.len() == 2 && dealer_total.get_best_total() == 21 {
            if player.len() == 2 && player_total.get_best_total() == 21 {
                return HandResult::Push;
            }
            return HandResult::DealerWins(true);
        }

        // normal hand eval
        if player_total.get_best_total() > dealer_total.get_best_total() {
            return HandResult::PlayerWins(false);
        } else if player_total.get_best_total() < dealer_total.get_best_total() {
            return HandResult::DealerWins(false)
        } 
        HandResult::Push
    }

    fn get_card_value(card: &Card) -> HandTotal {
        match card.rank() {
            Rank::Ace => HandTotal { hard_total: 11, soft_total: 1 },
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => HandTotal { hard_total: 10, soft_total: 10 },
            Rank::Joker => panic!("Why is there a joker in the deck!??!"),
            _ => {
                let v = card.rank().to_ordinal()+1;
                HandTotal {
                    hard_total: v,
                    soft_total: v,
                }
            },
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    let mut discard = Deck::new_empty();

    // first we just generate a deck and draw from front of the deck until it is depleted.
    // each drawn card is added to a hand until the hand busts, the cards are then discarded to a discard pile
    // and the process continues until there are no cards left in the deck
    while deck.len() > 0 {
        let mut hand = Vec::new();

        loop {
            let total: HandTotal;

            hand.push(deck.draw_nth(0).unwrap());
            total = BlackjackEvaluator::hand_value(&hand);

            println!("Hand: {:?}", hand);
            println!("Total: {:?}", total);
            
            if total.hard_total > 21 && total.soft_total > 21 {
                break;
            }
        }
        
        // add the cards to the discard pile
        for card in hand {
            discard.add(card);
        }
        println!("Discard pile has {} card", discard.len());
    }

    println!("\n***********\n");

    fn print_hand_result(hand: &[Card], dealer: &[Card]) {
        let now = std::time::SystemTime::now();
        match BlackjackEvaluator::compare_hands(&hand, &dealer) {
            HandResult::DealerWins(is_blackjack) => println!(
                "Dealer wins {} v {} (blackjack = {})", 
                BlackjackEvaluator::hand_value(&dealer).get_best_total(), 
                BlackjackEvaluator::hand_value(&hand).get_best_total(),
                is_blackjack,
            ),
            HandResult::PlayerWins(is_blackjack) => println!(
                "Player wins {} v {} (blackjack = {})", 
                BlackjackEvaluator::hand_value(&hand).get_best_total(), 
                BlackjackEvaluator::hand_value(&dealer).get_best_total(),
                is_blackjack,
            ),
            HandResult::Push => println!(
                "Push! Dealer: {} v Player: {}", 
                BlackjackEvaluator::hand_value(&dealer).get_best_total(), 
                BlackjackEvaluator::hand_value(&hand).get_best_total()
            ),
        }
        println!("\t:: hand_result took {}ns", now.elapsed().unwrap().as_nanos());
    }

    let hand = vec![Card::from_suit_and_rank(Suit::Clubs, Rank::Ace), Card::from_suit_and_rank(Suit::Hearts, Rank::Seven)];
    let dealer = vec![Card::from_suit_and_rank(Suit::Diamonds, Rank::Jack), Card::from_suit_and_rank(Suit::Hearts, Rank::Seven)];
    print_hand_result(&hand, &dealer);

    let hand = vec![Card::from_suit_and_rank(Suit::Clubs, Rank::Ace), Card::from_suit_and_rank(Suit::Hearts, Rank::King)];
    let dealer = vec![Card::from_suit_and_rank(Suit::Diamonds, Rank::Nine), Card::from_suit_and_rank(Suit::Hearts, Rank::Eight)];
    print_hand_result(&hand, &dealer);

    let hand = vec![Card::from_suit_and_rank(Suit::Clubs, Rank::Ace), Card::from_suit_and_rank(Suit::Hearts, Rank::King)];
    let dealer = vec![Card::from_suit_and_rank(Suit::Diamonds, Rank::Jack), Card::from_suit_and_rank(Suit::Hearts, Rank::Ace)];
    print_hand_result(&hand, &dealer);

    let hand = vec![Card::from_suit_and_rank(Suit::Clubs, Rank::Ace), Card::from_suit_and_rank(Suit::Hearts, Rank::King)];
    let dealer = vec![Card::from_suit_and_rank(Suit::Diamonds, Rank::Nine), Card::from_suit_and_rank(Suit::Hearts, Rank::Seven), Card::from_suit_and_rank(Suit::Spades, Rank::Five)];
    print_hand_result(&hand, &dealer);
}
