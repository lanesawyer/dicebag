table! {
    campaigns (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    characters (id) {
        id -> Int4,
        campaign_id -> Nullable<Int4>,
        image -> Nullable<Varchar>,
        name -> Varchar,
        class -> Varchar,
        level -> Int4,
        background -> Nullable<Varchar>,
        race -> Varchar,
        alignment -> Nullable<Varchar>,
        experience_points -> Int4,
        strength -> Int4,
        dexterity -> Int4,
        constitution -> Int4,
        intelligence -> Int4,
        wisdom -> Int4,
        charisma -> Int4,
        proficiency_bonus -> Int4,
        has_inspiration -> Bool,
        personality_traits -> Nullable<Text>,
        ideals -> Nullable<Text>,
        bonds -> Nullable<Text>,
        flaws -> Nullable<Text>,
        features_and_traits -> Nullable<Text>,
        other_proficiencies_and_languages -> Nullable<Text>,
        armor_class -> Int4,
        speed -> Int4,
        hit_points -> Int4,
        current_hit_points -> Int4,
        temporary_hit_points -> Int4,
        hit_dice -> Int4,
        used_hit_dice -> Int4,
        saves -> Int4,
        failures -> Int4,
        equipment -> Nullable<Text>,
        copper -> Int4,
        silver -> Int4,
        electrum -> Int4,
        platinum -> Int4,
        gold -> Int4,
    }
}

joinable!(characters -> campaigns (campaign_id));

allow_tables_to_appear_in_same_query!(campaigns, characters,);
