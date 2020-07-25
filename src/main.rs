mod card_game;
use card_game::Game;
use card_game::StateFormat;



// main
fn main() {
   let mut game = card_game::TestGame::new();
   
   if !game.setup()
   {
       return;
   }
   print!("{}", game.get_string());

    while game.spin_once()
    {
        print!("{}", game.get_string());
    }

    
    return;
}