use sea_orm::{ entity::*, error::*, query::*, DatabaseConnection, DbConn, FromQueryResult };
use crate::database::entities::{ player, prelude::* };

use crate::proto::comet_scene::{
    AnnouncementData,
    AnnouncementOneData,
    ArcadeData,
    ArcadeDiffList,
    CharacterFullData,
    CharacterList,
    DifficultyList,
    NotifyCharacterFullData,
    PlayerBaseInfo,
    PlayerCurrencyInfo,
    PlayerVipInfo,
    ScoreList,
    SocialData,
    SongData,
    SongList,
    TeamData,
    ThemeData,
    ThemeList,
    TitleData,
};

#[rustfmt::skip]
pub async fn get_player_base_info(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<PlayerBaseInfo> {
    let player = Player::find_by_id(account_id).one(db).await?;

    let player::Model {
        name,
        character_id,
        head_id,
        level,
        current_exp,
        maximum_exp,
        selected_character_id,
        selected_theme_id,
        pre_rank,
        country,
        pre_rank4k,
        pre_rank6k,
        title_id,
        ..
    } = player.unwrap();

    Ok(PlayerBaseInfo {
        acc_id: account_id,
        char_id: character_id,
        char_name: name,
        head_id,
        level,
        cur_exp: current_exp,
        max_exp: maximum_exp,
        guide_step: 7,
        cur_character_id: selected_character_id,
        cur_theme_id: selected_theme_id,
        online_time: 0,
        need_req_app_receipt: 0,
        active_point: 0,
        pre_rank_id: pre_rank,

        // TODO(arjix): Figure out what this is.
        guide_list: vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        
        country,
        pre_rank_id4_k: pre_rank4k,
        pre_rank_id6_k: pre_rank6k,
        title_id,
    })
}

#[rustfmt::skip]
pub async fn get_player_currency_info(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<PlayerCurrencyInfo> {
    // TODO: Populate this using data from the database.
    Ok(PlayerCurrencyInfo {
        gold: 0,
        diamond: 0,
        cur_stamina: 10,
        max_stamina: 10,
        honour_point: 0,
    })
}

pub async fn get_announcements(db: &DatabaseConnection) -> anyhow::Result<AnnouncementData> {
    // TODO: Populate this using data from the database.
    Ok(AnnouncementData {
        list: vec![AnnouncementOneData {
            title: "Operation Announcement".to_string(),
            content: "<b><color=#ffa500ff>《音灵INVAXION》Closing notice</color></b>\n\t\t  \n\n　　It's been a long wait, guardians of the sound.\n\t\t  \n　　Welcome to the<color=#ffa500ff>《音灵INVAXION》</color> world.".to_string(),
            pic_id: 0,
            tag: 1,
        }],
        pic_list: vec![],
    })
}

#[rustfmt::skip]
pub async fn get_player_score_list(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<ScoreList> {
    // TODO: Populate this using data from the database.

    let diff = DifficultyList {
        easy_list: Vec::with_capacity(0),
        normal_list: Vec::with_capacity(0),
        hard_list: Vec::with_capacity(0)
    };

    Ok(ScoreList {
        key4_list: diff.clone(),
        key6_list: diff.clone(),
        key8_list: diff.clone(),
    })
}

#[rustfmt::skip]
pub async fn get_player_song_list(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<SongList> {
    // TODO: Populate this using data from the database.

    let list = vec![
        80031, 80008, 80011, 80012,
        80010, 80034, 80007, 80015,
        80013, 80009, 80014, 80019,
        80020, 80018, 63122, 63123,
        63204, 62005, 62006, 63103,
        69008, 68008, 68108, 80002,
        64005, 69018, 68002, 68001,
        82005, 82006, 82007, 82011,
        65102, 68106, 64003, 62021,
        65036
    ].iter()
     .map(|id| SongData { song_id: *id })
     .collect::<Vec<_>>();

    Ok(SongList {
        list,
        favorite_list: Vec::with_capacity(0),
    })
}

#[rustfmt::skip]
pub async fn get_player_char_list(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<CharacterList> {
    // TODO: Populate this using data from the database.

    Ok(CharacterList {
        list: Vec::with_capacity(0),
    })
}

#[rustfmt::skip]
pub async fn get_player_social_data(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<SocialData> {
    // TODO: Populate this using data from the database.

    Ok(SocialData {
        friend_list: Vec::with_capacity(0),
        request_list: Vec::with_capacity(0),
    })
}

#[rustfmt::skip]
pub async fn get_player_theme_list(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<ThemeList> {
    // TODO: Populate this using data from the database.

    Ok(ThemeList { list: vec![ThemeData { theme_id: 1 }] })
}

#[rustfmt::skip]
pub async fn get_player_vip_info(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<PlayerVipInfo> {
    // TODO: Populate this using data from the database.

    Ok(PlayerVipInfo {
        level: 0,
        exp: 0,
        level_up_exp: 0,
        in_subscription: 0,
    })
}

#[rustfmt::skip]
pub async fn get_player_arcade_data(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<ArcadeData> {
    // TODO: Populate this using data from the database.

    let diff = ArcadeDiffList {
        easy_list: Vec::with_capacity(0),
        normal_list: Vec::with_capacity(0),
        hard_list: Vec::with_capacity(0)
    };
    Ok(ArcadeData {
        key4_list: diff.clone(),
        key6_list: diff.clone(),
        key8_list: diff.clone(),
    })
}

#[rustfmt::skip]
pub async fn get_player_title_data(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<TitleData> {
    // TODO: Populate this using data from the database.

    Ok(TitleData {
        list: Vec::with_capacity(0),
    })
}

#[rustfmt::skip]
pub async fn get_player_team(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<TeamData> {
    // TODO: Populate this using data from the database.

    Ok(TeamData {
        team_id: 0,
        team_name: "N/A".to_string(),
        upload_song_count: 0,
        can_upload_song: 0,
        buff_list: Vec::with_capacity(0),
    })
}

#[rustfmt::skip]
pub async fn get_character_full_data(
    account_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<CharacterFullData> {
    let announcement = get_announcements(db).await?;
    let base_info = get_player_base_info(account_id, db).await?;
    let currency_info = get_player_currency_info(account_id, db).await?;
    let score_list = get_player_score_list(account_id, db).await?;
    let song_list = get_player_song_list(account_id, db).await?;
    let char_list = get_player_char_list(account_id, db).await?;
    let social_data = get_player_social_data(account_id, db).await?;
    let item_list = Vec::with_capacity(0);
    let theme_list = get_player_theme_list(account_id, db).await?;
    let vip_info = get_player_vip_info(account_id, db).await?;
    let experience_list = Vec::with_capacity(0);
    let arcade_data = get_player_arcade_data(account_id, db).await?;
    let title_list = get_player_title_data(account_id, db).await?;
    let team = get_player_team(account_id, db).await?;

    Ok(CharacterFullData {
        base_info,
        currency_info,
        score_list,
        song_list,
        char_list,
        social_data,
        announcement,
        item_list,
        theme_list,
        vip_info,
        experience_list,
        arcade_data,
        title_list,
        team,
    })
}