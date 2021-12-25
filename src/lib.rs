pub mod draw_rates {
    pub fn deck_draw_rate(drawn_per_turn: f32, num_target: f32, num_deck: f32) -> f32 {
        let num_draws: i32 = (num_deck / num_target).ceil() as i32;
        (0..num_draws - 1)
            .map(|x| x as f32)
            .map(|x| single_turn_sum(drawn_per_turn, num_target - (x), num_deck - (x), false))
            .product::<f32>()
            * 100.0
    }

    pub fn single_turn_sum(
        drawn_per_turn: f32,
        num_target: f32,
        num_deck: f32,
        percent: bool,
    ) -> f32 {
        if (num_target) <= 0.0 {
            return 0.0;
        }
        let result = 1.0
            - (0..drawn_per_turn as i32)
                .map(|i| (num_deck - num_target - (i as f32)) / (num_deck - (i as f32)))
                .product::<f32>();
        if percent {
            return 100.0 * result;
        }
        result
    }

    pub fn armored_health_loss(shield_gains: &Vec<i32>, enemy_attacks: &Vec<i32>) -> i32 {
        shield_gains.iter().sum::<i32>() - enemy_attacks.iter().sum::<i32>()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use assert::close;

//     #[test]
//     #[should_panic]
//     fn single_turn() {
//         let x: f32 = 1.0 / 0.0;
//     }

//     #[test]
//     fn even_draw_rates() {
//         assert_eq!(0, 0)
//     }

//     #[test]
//     fn odd_draw_rates() {
//         assert_eq!(0, 0)
//     }
// }
