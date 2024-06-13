use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;
fn main() {
    let mut rounds=1;
    let mut playerpts=0;
    let mut noops=0;
    let mut enemypts=0;
    'gamel: loop{
        println!("Round {}", rounds);
        let player: Rps = loop{
            print!("Player: ");
            io::stdout().flush().expect("huh?");
            let mut buff=String::new();
            io::stdin().read_line(&mut buff).expect("huh?");
            match buff.trim().parse(){
                Ok(x)=>break x,
                Err(x)=>if x {
                    break 'gamel;
                }
                else{
                    println!("Wrong value, let's try again!");
                }
            }
        };
        
        let enemy=Rps::from_int(rand::thread_rng().gen_range(0..3));
        println!("Enemy: {}",enemy.to_string());
        let result = Rps::duel(player, enemy);
        result.print();
        let result=result.to_counters();
        playerpts+=result.0;
        enemypts+=result.1;
        noops+=result.2;
        rounds+=1;
        println!();
    }
    println!("\nGAME'S FINISHED!\n----------------\nRounds played: {}, Wins: {}, Losses: {}, Draws: {}",rounds-1,playerpts,enemypts,noops);
}
enum Rps{
    Rock,
    Paper,
    Scissors
}
impl Rps{
    fn from_int(x:u8)->Self{
        match x{
            0=>Ok(Self::Rock),
            1=>Ok(Self::Paper),
            2=>Ok(Self::Scissors),
            _=>Err(())
        }.expect("Wrong number provided")
    }
    fn to_string(&self)->String{
        match self {
            Self::Rock=>String::from("rock"),
            Self::Paper=>String::from("paper"),
            Self::Scissors=>String::from("scissors")
        }
    }
    fn duel(player: Self, enemy: Self)->RpsResult{
        match player{
            Self::Rock=> match enemy {
                Self::Rock=>RpsResult::Noop,
                Self::Paper=>RpsResult::Enemy,
                Self::Scissors=>RpsResult::Player
            },
            Self::Paper=>match enemy {
                Self::Rock=>RpsResult::Player,
                Self::Paper=>RpsResult::Noop,
                Self::Scissors=>RpsResult::Enemy
            },
            Self::Scissors=>match enemy {
                Self::Rock=>RpsResult::Enemy,
                Self::Paper=>RpsResult::Player,
                Self::Scissors=>RpsResult::Noop
            }
        }
    }
}
impl FromStr for Rps{
    type Err = bool;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().chars().nth(0){
            None=>Err(false),
            Some(x)=>match x {
                'r'|'k'=>Ok (Self::Rock),
                'p'    =>Ok (Self::Paper),
                's'|'n'=>Ok (Self::Scissors),
                'q'    =>Err(true),
                 _     =>Err(false)
            }
        }
    }
}
enum RpsResult{
    Player,
    Enemy,
    Noop
}
impl RpsResult{
    fn print(&self){
        match self{
            Self::Enemy=>println!("Enemy wins!"),
            Self::Player=>println!("Player wins!"),
            Self::Noop=>println!("Nobody wins!")
        }
    }
    fn to_counters(&self)->(i32,i32,i32){
        match self{
            Self::Player=>(1,0,0),
            Self::Enemy=>(0,1,0),
            Self::Noop=>(0,0,1)
        }
    }
}