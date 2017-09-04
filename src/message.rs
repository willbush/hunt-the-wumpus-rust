pub struct Logo;
pub struct Prompt;
pub struct Message;
#[allow(dead_code)]
pub struct Warning;

impl Logo {
    pub const HUNT_ASCII: &'static str = "
 ██░ ██  █    ██  ███▄    █ ▄▄▄█████▓
▓██░ ██▒ ██  ▓██▒ ██ ▀█   █ ▓  ██▒ ▓▒
▒██▀▀██░▓██  ▒██░▓██  ▀█ ██▒▒ ▓██░ ▒░
░▓█ ░██ ▓▓█  ░██░▓██▒  ▐▌██▒░ ▓██▓ ░
░▓█▒░██▓▒▒█████▓ ▒██░   ▓██░  ▒██▒ ░
 ▒ ░░▒░▒░▒▓▒ ▒ ▒ ░ ▒░   ▒ ▒   ▒ ░░
 ▒ ░▒░ ░░░▒░ ░ ░ ░ ░░   ░ ▒░    ░
 ░  ░░ ░ ░░░ ░ ░    ░   ░ ░   ░
 ░  ░  ░   ░              ░
";
    pub const THE_ASCII: &'static str = "
▄▄▄█████▓ ██░ ██ ▓█████
▓  ██▒ ▓▒▓██░ ██▒▓█   ▀
▒ ▓██░ ▒░▒██▀▀██░▒███
░ ▓██▓ ░ ░▓█ ░██ ▒▓█  ▄
  ▒██▒ ░ ░▓█▒░██▓░▒████▒
  ▒ ░░    ▒ ░░▒░▒░░ ▒░ ░
    ░     ▒ ░▒░ ░ ░ ░  ░
  ░       ░  ░░ ░   ░
          ░  ░  ░   ░  ░
";
    pub const WUMPUS_ASCII: &'static str = "
 █     █░█    ██  ███▄ ▄███▓ ██▓███   █    ██   ██████
▓█░ █ ░█░██  ▓██▒▓██▒▀█▀ ██▒▓██░  ██▒ ██  ▓██▒▒██    ▒
▒█░ █ ░█▓██  ▒██░▓██    ▓██░▓██░ ██▓▒▓██  ▒██░░ ▓██▄
░█░ █ ░█▓▓█  ░██░▒██    ▒██ ▒██▄█▓▒ ▒▓▓█  ░██░  ▒   ██▒
░░██▒██▓▒▒█████▓ ▒██▒   ░██▒▒██▒ ░  ░▒▒█████▓ ▒██████▒▒
░ ▓░▒ ▒ ░▒▓▒ ▒ ▒ ░ ▒░   ░  ░▒▓▒░ ░  ░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░
  ▒ ░ ░ ░░▒░ ░ ░ ░  ░      ░░▒ ░     ░░▒░ ░ ░ ░ ░▒  ░ ░
  ░   ░  ░░░ ░ ░ ░      ░   ░░        ░░░ ░ ░ ░  ░  ░
    ░      ░            ░               ░           ░
";
}

#[allow(dead_code)]
impl Prompt {
    pub const ACTION: &'static str = "Shoot, Move or Quit(S - M - Q)? ";
    pub const PLAY: &'static str = "Play again? (Y-N)";
    pub const SETUP: &'static str = "Same Setup? (Y-N)";
    pub const NUM_ROOMS_TO_SHOOT: &'static str = "No. or rooms (0-5)?";
    pub const ROOM_NUM: &'static str = "Room #?";
}

#[allow(dead_code)]
impl Message {
    pub const BAT_SNATCH: &'static str = "Zap--Super Bat snatch! Elsewhereville for you!";
    pub const WUMPUS_BUMP: &'static str = "...Oops! Bumped a wumpus!";

    pub const OUT_OF_ARROWS: &'static str = "You've run out of arrows!";
    pub const ARROW_GOT_YOU: &'static str = "Ouch! Arrow got you!";
    pub const MISSED: &'static str = "Missed!";
    pub const TOO_CROOKED: &'static str = "Arrows aren't that crooked - try another room!";

    pub const FELL_IN_PIT: &'static str = "YYYIIIIEEEE... fell in a pit!";
    pub const WUMPUS_GOT_YOU: &'static str = "Tsk tsk tsk - wumpus got you!";
    pub const LOSE: &'static str = "Ha ha ha - you lose!";
    pub const WIN: &'static str = "Aha! You got the Wumpus!\nHee hee hee - the Wumpus'll getcha next time!!";
}

#[allow(dead_code)]
impl Warning {
    pub const PIT: &'static str = "I feel a draft!";
    pub const WUMPUS: &'static str = "I Smell a Wumpus.";
    pub const BAT: &'static str = "Bats nearby!";
}
