pub fn calculate_modifier(stat: usize) -> i32 {
    match stat {
        1 => -5,
        2 | 3 => -4,
        4 | 5 => -3,
        6 | 7 => -2,
        8 | 9 => -1,
        10 | 11 => 0,
        12 | 13 => 1,
        14 | 15 => 2,
        16 | 17 => 3,
        18 | 19 => 4,
        20 => 5,
        _ => panic!("Ability scores must be between 1 and 20"),
    }
}

pub fn calculate_modifier_display(stat: usize) -> String {
    format!("{:+}", calculate_modifier(stat))
}

#[cfg(test)]
mod tests {
    use super::{calculate_modifier, calculate_modifier_display};

    #[test]
    fn calculate_modifier_works() {
        assert_eq!(calculate_modifier(1), -5);
        assert_eq!(calculate_modifier(2), -4);
        assert_eq!(calculate_modifier(3), -4);
        assert_eq!(calculate_modifier(4), -3);
        assert_eq!(calculate_modifier(5), -3);
        assert_eq!(calculate_modifier(6), -2);
        assert_eq!(calculate_modifier(7), -2);
        assert_eq!(calculate_modifier(8), -1);
        assert_eq!(calculate_modifier(9), -1);
        assert_eq!(calculate_modifier(10), 0);
        assert_eq!(calculate_modifier(11), 0);
        assert_eq!(calculate_modifier(12), 1);
        assert_eq!(calculate_modifier(13), 1);
        assert_eq!(calculate_modifier(14), 2);
        assert_eq!(calculate_modifier(15), 2);
        assert_eq!(calculate_modifier(16), 3);
        assert_eq!(calculate_modifier(17), 3);
        assert_eq!(calculate_modifier(18), 4);
        assert_eq!(calculate_modifier(19), 4);
        assert_eq!(calculate_modifier(20), 5);
    }

    #[test]
    fn calculate_modifier_display_works() {
        assert_eq!(calculate_modifier_display(1), "-5");
        assert_eq!(calculate_modifier_display(20), "+5");
    }
}
