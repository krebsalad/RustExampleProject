use rand::Rng;
use std::collections::HashMap;
use std::cell::RefCell;

// Interfaces
trait StateFormat
{
    fn get_string(&self) -> String;
}

trait Game
{
    fn setup(&mut self) -> bool;
    fn spin_once(&mut self) -> bool;
    fn stop(&mut self) -> bool;
}

trait CardShare
{
    fn take(&mut self) -> (bool, Card);
    fn give(&mut self, _card : Card) -> bool;
}

// Core
#[derive(Copy, Clone)]
pub struct Card
{
    pub card_type : i32,
    pub card_num: i32,
}

#[derive(PartialEq)]
pub enum DeckType {
    DefaultDeck,
}

pub struct Deck
{
    pub cards : Vec<Card>,
}

pub struct Player
{
    pub name : String,
    pub score : i32,
    pub hand : Deck,
}


// test Game 
pub struct TestGame
{
    
    pub deck : Deck,
    pub players : HashMap<String, RefCell<Player>>,
}


// Core impl
impl Card
{ 
    pub fn new(_card_type : i32, _card_num :i32) -> Card
    {
        return Card {
            card_type : _card_type,
            card_num: _card_num,
        }
    }
    
    pub fn is_same_card_as(&self, other_card: &Card) -> bool
    {
        return other_card.get_string() == self.get_string();
    }
}

impl Player
{ 
    pub fn new(_name : String) -> Player
    {
        return Player {
            name: _name,
            score: 0,
            hand: Deck::new(),
        }
    }
}

impl Deck
{
    pub fn new() -> Deck
    {
        return Deck {
            cards : Vec::new(),
        }
    }
    
    pub fn clear(&mut self)
    {
        self.cards.clear();
    }
    
    pub fn setup(&mut self, _deck_type : DeckType)
    {
        if _deck_type == DeckType::DefaultDeck
        {
            for _t in 1..14
            {
                for _c in 1..5
                {
                    self.cards.push(Card::new(_t, _c));
                }
            }
            
        }
        
    }
    
    pub fn shuffle(&mut self, _iterations : u32)
    {
        for _i in 0.._iterations
        {
            let mut _rng = rand::thread_rng();
            for _current_card_i in 0..self.cards.len()
            {
                let _rand_i = _rng.gen_range(0, self.cards.len());
                let _temp_card = self.cards[_rand_i];
                self.cards[_rand_i] = self.cards[_current_card_i];
                self.cards[_current_card_i] = _temp_card;
            }
        }
            
    }
}

// Game impl
impl TestGame
{
    pub fn new() -> TestGame
    {
        return TestGame {
            deck : Deck::new(),
            players : HashMap::new(),
        }
    }
}


// cardshare traits impl
impl CardShare for Player
{
    fn take(&mut self) -> (bool, Card)
    {
        return self.hand.take();
    }

    fn give(&mut self, _card : Card) -> bool
    {
        return self.hand.give(_card);
    }
}

impl CardShare for Deck
{
    
    fn take(&mut self) -> (bool, Card)
    {
        if self.cards.len() == 0
        {
            return (false, Card::new(0, 0));
        }
        let mut _card = self.cards[0];
        self.cards.remove(0);
        return (true, _card);
    }

    fn give(&mut self, _card : Card) -> bool
    {
        self.cards.push(_card);
        return true;
    }

}


// Game trait impl
impl Game for TestGame
{
    fn setup(&mut self) -> bool
    {
        self.deck.setup(DeckType::DefaultDeck);     
        self.deck.shuffle(3);
        self.players.insert("player1".to_string(), RefCell::new(Player::new("player1".to_string())));
        self.players.insert("player2".to_string(), RefCell::new(Player::new("player2".to_string())));
        for _i in 0..5
        {
            for (_name, _player) in &self.players
            {
                let res = self.deck.take();
                if res.0
                {
                    let mut _player_ref = self.players.get(_name).unwrap().borrow_mut();
                    _player_ref.give(res.1);
                }
            }
        }
        return true;
    }
    fn spin_once(&mut self) -> bool
    {
        
        return true;
    }
    fn stop(&mut self) -> bool
    {
        return true;
    }
}


// stateformat trait impl
impl StateFormat for Card
{
    fn get_string(&self) -> String {
        return format!("c-{},{}", self.card_type, self.card_num);
    }
}

impl StateFormat for Player
{
    fn get_string(&self) -> String {
        return format!("{}'s deck: \n{}", self.name, self.hand.get_string());
    }
}

impl StateFormat for Deck
{
    fn get_string(&self) -> String {
        let mut txt = String::from("");
        for _c in self.cards.iter()
        {
            txt.push_str(&_c.get_string());
            txt.push_str("\n");
        }
        return txt;
    }
}

impl StateFormat for TestGame
{
    fn get_string(&self) -> String {
        
        let mut txt = String::from("");
        for (_name, _player) in &self.players
        {
            txt.push_str("--------\n");
            txt.push_str(&_player.borrow().get_string());
            txt.push_str("--------\n\n");
        }
        return txt;
    }
}

// main
fn main() {
   let mut game = TestGame::new();
   if !game.setup()
   {
       return;
   }

    if !game.spin_once()
    {
        return;
    }
    print!("{}", game.get_string());
    
}