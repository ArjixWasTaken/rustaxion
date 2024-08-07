use prost::Message;
use sea_orm::EntityTrait;

use crate::{
    database::entities::prelude::*,
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::{ RetShopInfo, ShopRecommend },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let shop_items = ShopItem::find().all(&db).await?;

    // TODO(arjix): Once a web-ui is made, make this more customizable.

    let ret = RetShopInfo {
        character_list: shop_items.iter().filter_map(|item| {
            match item.item_type.as_ref().map(|x| x.as_str()) {
                Some("Character") => Some(item.into_proto()),
                _ => None
            }
        }).collect::<Vec<_>>(),
        song_list: shop_items.iter().filter_map(|item| {
            match item.item_type.as_ref().map(|x| x.as_str()) {
                Some("Song") => Some(item.into_proto()),
                _ => None
            }
        }).collect::<Vec<_>>(),
        theme_list: shop_items.iter().filter_map(|item| {
            match item.item_type.as_ref().map(|x| x.as_str()) {
                Some("Theme") => Some(item.into_proto()),
                _ => None
            }
        }).collect::<Vec<_>>(),
        pay_list: vec![],
        piece_list: vec![],
        member_list: vec![],
        shop_recommend: ShopRecommend { hot_sell_list: vec![] },
        summon_shop_list: vec![],
        vip_reward_list: vec![],
    };
    
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseShopInfo),
        body: ret.encode_to_vec()
    }])
}
