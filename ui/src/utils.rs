pub fn calculate_modifier(stat: i64) -> i64 {
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
        _ => panic!("Ability score must be between 1 and 20"),
    }
}

pub fn calculate_modifier_display(stat: i64) -> String {
    format!("{:+}", calculate_modifier(stat))
}

pub fn level_xp(level: i64) -> i64 {
    match level {
        1 => 0,
        2 => 300,
        3 => 900,
        4 => 2700,
        5 => 6500,
        6 => 14000,
        7 => 23000,
        8 => 34000,
        9 => 48000,
        10 => 64000,
        11 => 85000,
        12 => 100000,
        13 => 120000,
        14 => 140000,
        15 => 165000,
        16 => 195000,
        17 => 225000,
        18 => 265000,
        19 => 305000,
        20 => 355000,
        _ => panic!("Level must be between 1 and 20"),
    }
}

pub fn level_xp_display(level: i64) -> String {
    format!("{}", level_xp(level))
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
