pub struct CharactersQuery;
pub mod characters_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CharactersQuery";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub characters: Vec<CharactersQueryCharacters>,
    }
    #[derive(Deserialize)]
    pub struct CharactersQueryCharacters {
        pub id: Int,
        pub image: Option<String>,
        pub name: String,
        pub class: String,
        pub level: Int,
        pub background: Option<String>,
        pub race: String,
        pub alignment: Option<String>,
        #[serde(rename = "experiencePoints")]
        pub experience_points: Int,
        pub strength: Int,
        pub dexterity: Int,
        pub constitution: Int,
        pub intelligence: Int,
        pub wisdom: Int,
        pub charisma: Int,
        #[serde(rename = "proficiencyBonus")]
        pub proficiency_bonus: Int,
        #[serde(rename = "hasInspiration")]
        pub has_inspiration: Boolean,
        #[serde(rename = "personalityTraits")]
        pub personality_traits: Option<String>,
        pub ideals: Option<String>,
        pub bonds: Option<String>,
        pub flaws: Option<String>,
        #[serde(rename = "featuresAndTraits")]
        pub features_and_traits: Option<String>,
        #[serde(rename = "otherProficienciesAndLanguages")]
        pub other_proficiencies_and_languages: Option<String>,
        #[serde(rename = "armorClass")]
        pub armor_class: Int,
        pub speed: Int,
        #[serde(rename = "hitPoints")]
        pub hit_points: Int,
        #[serde(rename = "currentHitPoints")]
        pub current_hit_points: Int,
        #[serde(rename = "temporaryHitPoints")]
        pub temporary_hit_points: Int,
        #[serde(rename = "hitDice")]
        pub hit_dice: Int,
        #[serde(rename = "usedHitDice")]
        pub used_hit_dice: Int,
        pub saves: Int,
        pub failures: Int,
        pub equipment: Option<String>,
        pub copper: Int,
        pub silver: Int,
        pub electrum: Int,
        pub platinum: Int,
        pub gold: Int,
    }
}
impl graphql_client::GraphQLQuery for CharactersQuery {
    type Variables = characters_query::Variables;
    type ResponseData = characters_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: characters_query::QUERY,
            operation_name: characters_query::OPERATION_NAME,
        }
    }
}
pub struct CampaignsQuery;
pub mod campaigns_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CampaignsQuery";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub campaigns: Vec<CampaignsQueryCampaigns>,
    }
    #[derive(Deserialize)]
    pub struct CampaignsQueryCampaigns {
        pub id: Int,
        pub name: String,
        pub description: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for CampaignsQuery {
    type Variables = campaigns_query::Variables;
    type ResponseData = campaigns_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: campaigns_query::QUERY,
            operation_name: campaigns_query::OPERATION_NAME,
        }
    }
}
pub struct NewCharacterMutation;
pub mod new_character_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "NewCharacterMutation";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct NewCharacter {
        pub image: Option<String>,
        pub name: String,
        pub class: String,
        pub race: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "newCharacter")]
        pub new_character: NewCharacter,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "newCharacter")]
        pub new_character: Boolean,
    }
}
impl graphql_client::GraphQLQuery for NewCharacterMutation {
    type Variables = new_character_mutation::Variables;
    type ResponseData = new_character_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: new_character_mutation::QUERY,
            operation_name: new_character_mutation::OPERATION_NAME,
        }
    }
}
pub struct NewCampaignMutation;
pub mod new_campaign_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "NewCampaignMutation";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct NewCampaign {
        pub name: String,
        pub description: Option<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "newCampaign")]
        pub new_campaign: NewCampaign,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "newCampaign")]
        pub new_campaign: Boolean,
    }
}
impl graphql_client::GraphQLQuery for NewCampaignMutation {
    type Variables = new_campaign_mutation::Variables;
    type ResponseData = new_campaign_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: new_campaign_mutation::QUERY,
            operation_name: new_campaign_mutation::OPERATION_NAME,
        }
    }
}
pub struct DeleteCharacterMutation;
pub mod delete_character_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteCharacterMutation";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "deleteId")]
        pub delete_id: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "deleteCharacter")]
        pub delete_character: Boolean,
    }
}
impl graphql_client::GraphQLQuery for DeleteCharacterMutation {
    type Variables = delete_character_mutation::Variables;
    type ResponseData = delete_character_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_character_mutation::QUERY,
            operation_name: delete_character_mutation::OPERATION_NAME,
        }
    }
}
pub struct DeleteCampaignMutation;
pub mod delete_campaign_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteCampaignMutation";
    pub const QUERY : & str = "query CharactersQuery {\n    characters {\n        id\n        image\n        name\n        class\n        level\n        background\n        race\n        alignment\n        experiencePoints\n        strength\n        dexterity\n        constitution\n        intelligence\n        wisdom\n        charisma\n        proficiencyBonus\n        hasInspiration\n        personalityTraits\n        ideals\n        bonds\n        flaws\n        featuresAndTraits\n        otherProficienciesAndLanguages\n        armorClass\n        speed\n        hitPoints\n        currentHitPoints\n        temporaryHitPoints\n        hitDice\n        usedHitDice\n        saves\n        failures\n        equipment\n        copper\n        silver\n        electrum\n        platinum\n        gold\n    }\n}\n\nquery CampaignsQuery {\n    campaigns {\n        id\n        name\n        description\n    }\n}\n\nmutation NewCharacterMutation($newCharacter: NewCharacter!) {\n    newCharacter(newCharacter: $newCharacter)\n}\n\nmutation NewCampaignMutation($newCampaign: NewCampaign!) {\n    newCampaign(newCampaign: $newCampaign)\n}\n\nmutation DeleteCharacterMutation($deleteId: Int!) {\n    deleteCharacter(deleteId: $deleteId)\n}\n\nmutation DeleteCampaignMutation($deleteId: Int!) {\n    deleteCampaign(deleteId: $deleteId)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "deleteId")]
        pub delete_id: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "deleteCampaign")]
        pub delete_campaign: Boolean,
    }
}
impl graphql_client::GraphQLQuery for DeleteCampaignMutation {
    type Variables = delete_campaign_mutation::Variables;
    type ResponseData = delete_campaign_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_campaign_mutation::QUERY,
            operation_name: delete_campaign_mutation::OPERATION_NAME,
        }
    }
}
