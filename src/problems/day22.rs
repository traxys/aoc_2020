use crate::DayContext;
use std::{
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
};

type Input = (VecDeque<u64>, VecDeque<u64>);

pub fn part_1((mut deck1, mut deck2): Input) -> color_eyre::Result<String> {
    while !deck1.is_empty() && !deck2.is_empty() {
        play_round(&mut deck1, &mut deck2);
    }

    let (winner, score) = if deck1.is_empty() {
        ("player2", score(&deck2))
    } else {
        ("player1", score(&deck1))
    };

    Ok(format!("Winner is {} with: {}", winner, score))
}

fn play_round(player1: &mut VecDeque<u64>, player2: &mut VecDeque<u64>) {
    let card1 = player1.pop_front().unwrap();
    let card2 = player2.pop_front().unwrap();

    if card1 > card2 {
        player1.push_back(card1);
        player1.push_back(card2);
    } else {
        player2.push_back(card2);
        player2.push_back(card1);
    }
}

// If it returns true it means it already existed
fn register_configuration(
    player1: &VecDeque<u64>,
    player2: &VecDeque<u64>,
    seen: &mut HashSet<u64>,
) -> bool {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    player1.hash(&mut hasher);
    player2.hash(&mut hasher);

    !seen.insert(hasher.finish())
}

// true means player1 won
fn recursive_combat(
    mut player1: VecDeque<u64>,
    mut player2: VecDeque<u64>,
) -> (bool, VecDeque<u64>) {
    let mut seen = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if register_configuration(&player1, &player2, &mut seen) {
            return (true, player1);
        }

        let card1 = player1.pop_front().unwrap() as usize;
        let card2 = player2.pop_front().unwrap() as usize;

        let player1win;
        if card1 <= player1.len() && card2 <= player2.len() {
            player1win = recursive_combat(
                player1.iter().take(card1).copied().collect(),
                player2.iter().take(card2).copied().collect(),
            )
            .0;
        } else {
            player1win = card1 > card2;
        }

        if player1win {
            player1.push_back(card1 as u64);
            player1.push_back(card2 as u64);
        } else {
            player2.push_back(card2 as u64);
            player2.push_back(card1 as u64);
        }
    }

    // If the player2 deck is empty, player1 won
    let player1win = player2.is_empty();
    (player1win, if player1win { player1 } else { player2 })
}

fn score(deck: &VecDeque<u64>) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) as u64 * card)
        .sum()
}

pub fn part_2((player1, player2): Input) -> color_eyre::Result<String> {
    let (winner, deck) = recursive_combat(player1, player2);
    let winner_name = if winner { "player1" } else { "player2" };

    Ok(format!("Winner is {} with: {}", winner_name, score(&deck),))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    fn load_example() -> (VecDeque<u64>, VecDeque<u64>) {
        let mut player1 = VecDeque::new();
        player1.extend([9, 2, 6, 3, 1].iter());

        let mut player2 = VecDeque::new();
        player2.extend([5, 8, 4, 7, 10].iter());

        (player1, player2)
    }

    #[test]
    fn recurse() {
        let (player1, player2) = load_example();
        let (winner, deck) = super::recursive_combat(player1, player2);

        assert_eq!(winner, false);
        assert_eq!(super::score(&deck), 291);
        assert_eq!(deck, vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut player1 = true;
    let mut deck1 = VecDeque::new();
    let mut deck2 = VecDeque::new();

    context.accumulate_str_lines(|_, line| {
        if !line.starts_with("Player") {
            if line.is_empty() {
                player1 = false;
            } else {
                let card = line.parse()?;
                if player1 {
                    deck1.push_back(card);
                } else {
                    deck2.push_back(card);
                }
            }
        }
        Ok(())
    })?;

    Ok((deck1, deck2))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}
