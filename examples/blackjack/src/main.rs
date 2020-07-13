use std::cmp;
use std::ops::AddAssign;
use rand::prelude::*;
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
        let player_total = player_total.get_best_total();
        let dealer_total = dealer_total.get_best_total();

        if player_total > 21 {
            return HandResult::DealerWins(false)
        }

        if player_total > dealer_total {
            return HandResult::PlayerWins(false);
        } else if player_total < dealer_total {
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

#[derive(Debug, Copy, Clone)]
struct BlackjackTableConfig {
    num_boxes: usize,
    max_splits_per_box: usize,
    split_aces: bool,
    decks_per_shoe: usize,
    blackjack_payout_factor: f32,
}

impl Default for BlackjackTableConfig {
    fn default() -> Self {
        Self {
            num_boxes: 5,
            max_splits_per_box: 4,
            split_aces: true,
            decks_per_shoe: 6,
            blackjack_payout_factor: 1.5,
        }
    }
}

#[derive(Debug, Clone)]
struct BlackjackBox {
    child_box: Vec<Option<BlackjackBox>>,
    cards: Vec<Card>,
}

impl Default for BlackjackBox {
    fn default() -> Self {
        Self {
            child_box: Vec::new(),
            cards: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlackjackTable {
    config: BlackjackTableConfig,
    deck: Deck,
    boxes: Vec<BlackjackBox>,
    dealer: Vec<Card>
}

impl BlackjackTable {
    pub fn new() -> Self {
        let config = BlackjackTableConfig::default();
        let mut deck = Deck::new_empty();
        let mut boxes = Vec::new();

        for _ in 0..config.decks_per_shoe {
            let mut d = Deck::new();
            deck.add_deck(&mut d);
        }

        for _ in 0..config.num_boxes {
            boxes.push(BlackjackBox::default());
        }

        Self {
            config,
            deck,
            boxes,
            dealer: Vec::new(),
        }
    }

    pub fn deal_cards(&mut self) {
        let num_boxes = self.boxes.len()+1; // dealer's box counts as one of the boxes

        // deal 2 cards to each box and 1 card to dealer
        for i in 0..(num_boxes*2-1) {
            let card = self.draw_card().unwrap();
            let box_num = i % num_boxes;

            if box_num < num_boxes-1 {
                self.boxes[box_num].cards.push(card);
            } else {
                self.dealer.push(card)
            }
        }
    }

    pub fn num_boxes(&self) -> usize {
        self.config.num_boxes
    }

    pub fn box_total(&self, index: usize) -> HandTotal {
        BlackjackEvaluator::hand_value(&self.boxes[index].cards)
    }

    pub fn dealer_total(&self) -> HandTotal {
        BlackjackEvaluator::hand_value(&self.dealer)
    }

    pub fn draw_card_for_box(&mut self, index: usize) {
        let card = self.draw_card().unwrap();
        self.boxes[index].cards.push(card);
    }

    pub fn draw_card_for_dealer(&mut self) {
        let card = self.draw_card().unwrap();
        self.dealer.push(card);
    }

    fn draw_card(&mut self) -> Option<Card> {
        let index = rand::random::<usize>() & self.deck.len()-1;
        // println!("deck length: {}, index = {}", self.deck.len(), index);

        self.deck.draw_nth(index)
    }
}

fn generate_and_deplete_deck() {
    let mut deck = Deck::new();
    let mut discard = Deck::new_empty();

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
}

fn evaluate_some_hands() {
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

fn run_table_games() {
    let now = std::time::SystemTime::now();
    let mut table = BlackjackTable::new();

    println!("Dealing hands...");
    table.deal_cards();

    println!("\nPlayers drawing while below 17");
    for i in 0..table.num_boxes() {
        while table.box_total(i).get_best_total() < 17 {
            table.draw_card_for_box(i);
        }

        println!("box {}, total {:?}:\n\t{:?}", i, table.box_total(i), table.boxes[i]);
    }

    println!("\nDealer drawing while below 17");
    while table.dealer_total().hard_total < 17 {
        table.draw_card_for_dealer();
    }
    println!("dealer total {:?}:\n\t{:?}", table.dealer_total(), table.dealer);

    println!("\nEvaluating hands");
    for i in 0..table.num_boxes() {
        match BlackjackEvaluator::compare_hands(&table.boxes[i].cards, &table.dealer) {
            HandResult::PlayerWins(is_bj) => println!(
                "Box {} wins with {} against {} (blackjack = {})", 
                i, 
                table.box_total(i).get_best_total(), 
                table.dealer_total().get_best_total(), 
                is_bj
            ),
            HandResult::DealerWins(is_bj) => println!(
                "Box {} loses with {} against {}", 
                i, 
                table.box_total(i).get_best_total(), 
                table.dealer_total().get_best_total()
            ),
            HandResult::Push => println!(
                "Box {} pushes with dealer ({} vs {})", 
                i, 
                table.box_total(i).get_best_total(), 
                table.dealer_total().get_best_total()
            ),
        }
    }
    
    println!("\nHand took {}µs to execute and evaluate", now.elapsed().unwrap().as_micros());
}

fn main() {
    // first we just generate a deck and draw from front of the deck until it is depleted.
    // each drawn card is added to a hand until the hand busts, the cards are then discarded to a discard pile
    // and the process continues until there are no cards left in the deck
    generate_and_deplete_deck();
    println!("\n***********\n");

    // test some hand evaluations
    evaluate_some_hands();
    println!("\n***********\n");

    // actual table based gameplay
    for i in 0..10 {
        run_table_games();
    }
}
