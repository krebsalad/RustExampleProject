use rand::Rng;
use std::collections::HashMap;
use std::cell::RefCell;

// Interfaces
pub trait StateFormat
{
    fn get_string(&self) -> String;
}

pub trait Game
{
    fn setup(&mut self) -> bool;
    fn spin_once(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn calculate_score(&self, _deck : &Deck) -> i32;
}

pub trait CardShare
{
    fn take(&mut self) -> (bool, Card);
    fn take_specific(&mut self, _index : usize) -> (bool, Card);
    fn give(&mut self, _card : Card) -> bool;
    fn count(&self) -> usize;
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
    pub hand : Deck,
}


// test Game 
pub struct TestGame
{
    
    pub deck : Deck,
    pub table : Deck,
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
}

impl Player
{ 
    pub fn new(_name : String) -> Player
    {
        return Player {
            name: _name,
            hand: Deck::new(),
        }
    }

    pub fn get_score(&mut self,  _game : &dyn Game) -> i32
    {
        return _game.calculate_score(&self.hand);
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
            table : Deck::new(),
            players : HashMap::new(),
        }
    }
    pub fn get_input(&self) -> String
    {
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        return s;
    }
    
}


// cardshare traits impl
impl CardShare for Player
{
    fn take(&mut self) -> (bool, Card)
    {
        return self.hand.take();
    }

    fn take_specific(&mut self, _index : usize) -> (bool, Card)
    {
        return self.hand.take_specific(_index);
    }

    fn give(&mut self, _card : Card) -> bool
    {
        return self.hand.give(_card);
    }

    fn count(&self) -> usize
    {
        return self.hand.count();
    }
}

impl CardShare for Deck
{
    
    fn take(&mut self) -> (bool, Card)
    {
        let _card_i : usize =  self.cards.len() - 1;
        return self.take_specific(_card_i);
    }

    fn take_specific(&mut self, _index : usize) -> (bool, Card)
    {
        if self.cards.len() == 0
        {
            return (false, Card::new(0, 0));
        }
        
        let mut _card = self.cards[_index];
        self.cards.remove(_index);
        return (true, _card);
    }

    fn give(&mut self, _card : Card) -> bool
    {
        self.cards.push(_card);
        return true;
    }

    fn count(&self) -> usize
    {
        return self.cards.len();
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
                else
                {
                    return false;
                }
            }
        }
        let res = self.deck.take();
        if res.0
        {
            self.table.give(res.1);
        }
        else
        {
            return false;
        }
        return true;
    }
    fn calculate_score(&self, _deck : &Deck) -> i32
    {
        let mut _score = 0;
        for _c in &_deck.cards
        {
            _score += _c.card_type;
        }
        return _score;
    }
    fn spin_once(&mut self) -> bool
    {
        print!("{}", &self.get_string());
        for (_name, _player) in &self.players
        {
            print!("{}'s turn\n", _name);

            // check if can finish game
            let mut _in_str : String = String::from("");
            if _player.borrow_mut().get_score(self) < 5
            {
                // ask if want to finish
                print!("type y to finish game or n to continue\n");
                loop
                {
                    _in_str = self.get_input();
                    if _in_str.to_string() == "y" || _in_str.to_string() == "n"
                    {
                        break;
                    }
                    print!("wrong input!\n");  
                }
                if _in_str == "y"
                {
                    print!("game ended, {} won!!\n", _name); 
                    return false;
                }
            }

            // take chosen cards(alteast 1) from current player hand to table
            let mut _exit_succesfully = false;
            let mut _to_table_cards : Vec<Card> = Vec::new();
            loop
            {
                let _hand_size = _player.borrow_mut().count();
                if _hand_size == 0
                {
                    break;
                }

                // check if can and wants to place more cards after first time
                if _exit_succesfully
                {
                    loop
                    {
                        print!("{}", &self.get_string());
                        print!("{} choose another card, please press y or n?\n", _name);
                        _in_str = self.get_input();
                        if _in_str.to_string() == "y" || _in_str.to_string() == "n"
                        {
                            break;
                        } 
                    }
                    if _in_str == "n"
                    {
                        break;
                    }
                }

                print!("choose a card to place between 0 and {}\n", _hand_size - 1);
                _in_str = self.get_input();

                for i in 0.._hand_size
                {
                    if _in_str == format!("{}", i)
                    {
                        let res = _player.borrow_mut().take_specific(i);
                        if res.0
                        {
                            
                            // check if card is playable
                            if _to_table_cards.len() > 0
                            {
                                let mut _all_are_equal = true;
                                for _card in _to_table_cards.iter()
                                {
                                    if res.1.card_type != _card.card_type
                                    {
                                        _all_are_equal = false;
                                    }
                                }

                                if !_all_are_equal
                                {
                                    // stop if chosen card is not not equal to prior chosen cards
                                    _player.borrow_mut().give(res.1);
                                    break;
                                }
                            }

                            // add the card
                            _to_table_cards.push(res.1);
                            _exit_succesfully = true;
                            break;
                        }
                    }
                }
            }

            // take card from deck OR from table 
            print!("press d to choose from deck or t to choose from table\n");
            loop
            {
                _in_str = self.get_input();
                if _in_str.to_string() == "d" || _in_str.to_string() == "t"
                {
                    break;
                }
                print!("wrong input!\n");  
            }
            if _in_str == "t"
            {
                let res = self.table.take();
                if res.0
                {
                    _player.borrow_mut().give(res.1);
                }
            }
            else
            {
                let res = self.deck.take();
                if res.0
                {
                    _player.borrow_mut().give(res.1);
                }
            }

            // add the players cards to the table
            if _exit_succesfully
            {
                for _card in _to_table_cards
                {
                    self.table.give(_card);
                }
            }

            print!("{}", &self.get_string());
        }
        return true;
    }
    
    fn stop(&mut self) -> bool
    {
        self.deck.clear();
        self.table.clear();
        self.players.clear();
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
        for (_i, _c) in self.cards.iter().enumerate()
        {
            txt.push_str(&format!("{}: ", _i));
            txt.push_str(&_c.get_string());
            txt.push_str("\n");
        }
        return txt;
    }
}

impl StateFormat for TestGame
{
    fn get_string(&self) -> String {
        
        let mut txt = String::from("--------\ntable:\n");
        txt.push_str(&self.table.get_string());
        txt.push_str("--------\n\n");
        for (_name, _player) in &self.players
        {
            txt.push_str("--------\n");
            txt.push_str(&_player.borrow().get_string());
            txt.push_str(&format!("score: {}\n", _player.borrow_mut().get_score(self)));
            txt.push_str("--------\n\n");
        }
        return txt;
    }
}