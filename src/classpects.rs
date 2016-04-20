use rand::random;

const ASPECT: [&'static str; 12] = ["Space", "Time", "Void", "Light", "Mind", "Heart",
                          "Rage", "Hope", "Doom", "Life", "Blood", "Breath"]; 
const CLASS: [&'static str; 12] = ["Rogue", "Thief", "Heir", "Maid", "Page", "Knight",
                                "Seer", "Mage","Sylph", "Witch", "Bard", "Prince"]; 
const DEADCLASS: [&'static str; 2] = ["Muse", "Lord"];

// This function gives classpects for up to 12 players. No normalizing finds place,
// so many groups may end up not having the required Space player and/or no Time player.
pub fn n_classpects(players: u32) -> Vec<String> {
    let mut left_aspects: Vec<usize> = vec![0,1,2,3,4,5,6,7,8,9,10,11];
    let mut left_classes: Vec<usize> = vec![0,1,2,3,4,5,6,7,8,9,10,11];
    let mut player_classpects: Vec<String> = Vec::new();
    
    for i in 0..players {
        let cl: usize = left_classes.len();
        let al: usize = left_aspects.len();
        let pcn: usize = left_classes.remove(random::<usize>() % cl);
        let pan: usize = left_aspects.remove(random::<usize>() % al);
        let pclass = CLASS[pcn];
        let paspect = ASPECT[pan];
        let mut s: String = pclass.to_string();
        s.push_str(" of ");
        s.push_str(paspect);
        player_classpects.push(s);
    }
    player_classpects
}
