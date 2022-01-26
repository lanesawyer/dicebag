CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    campaign_id INT REFERENCES campaigns,
    image VARCHAR,
    name VARCHAR NOT NULL,
    class VARCHAR NOT NULL,
    level INT NOT NULL DEFAULT 1,
    background VARCHAR,
    race VARCHAR NOT NULL,
    alignment VARCHAR,
    experience_points INT NOT NULL DEFAULT 0,
    strength INT NOT NULL DEFAULT 1,
    dexterity INT NOT NULL DEFAULT 1,
    constitution INT NOT NULL DEFAULT 1,
    intelligence INT NOT NULL DEFAULT 1,
    wisdom INT NOT NULL DEFAULT 1,
    charisma INT NOT NULL DEFAULT 1,
    proficiency_bonus INT NOT NULL DEFAULT 2,
    has_inspiration BOOLEAN NOT NULL DEFAULT 'f',
    personality_traits TEXT,
    ideals TEXT,
    bonds TEXT,
    flaws TEXT,
    features_and_traits TEXT,
    other_proficiencies_and_languages TEXT,
    armor_class INT NOT NULL DEFAULT 10,
    speed INT NOT NULL DEFAULT 30,
    hit_points INT NOT NULL DEFAULT 1,
    current_hit_points INT NOT NULL DEFAULT 0,
    temporary_hit_points INT NOT NULL DEFAULT 0,
    hit_dice INT NOT NULL DEFAULT 0,
    used_hit_dice INT NOT NULL DEFAULT 0,
    saves INT NOT NULL DEFAULT 0,
    failures INT NOT NULL DEFAULT 0,
    equipment TEXT,
    copper INT NOT NULL DEFAULT 0,
    silver INT NOT NULL DEFAULT 0,
    electrum INT NOT NULL DEFAULT 0,
    platinum INT NOT NULL DEFAULT 0,
    gold INT NOT NULL DEFAULT 0
);