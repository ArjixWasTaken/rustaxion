use crate::{
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    types::{packet::Packet, response::Response, session::SessionData},
};

#[rustfmt::skip]
pub fn handle(
    session: &mut SessionData,
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Response> {
    assert_eq!(main_cmd, MainCmd::Game);
    let ParaCmd::CometScene(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometScene::NtfCharacterFullData => todo!(),
        CometScene::RequestBeginSong => todo!(),
        CometScene::RequestFinishSong => todo!(),
        CometScene::RequestSingleSongRank => todo!(),
        CometScene::RequestRankInfo => todo!(),
        CometScene::RequestSetFavorite => todo!(),
        CometScene::RequestBackstageGame => todo!(),
        CometScene::NtfUpdateInfo => todo!(),
        CometScene::RequestActivityInfo => todo!(),
        CometScene::RequestActivityBegin => todo!(),
        CometScene::RequestActivityFinish => todo!(),
        CometScene::NtfActivityChange => todo!(),
        CometScene::RequestMailList => todo!(),
        CometScene::RequestGetMailReward => todo!(),
        CometScene::RequestDeleteMail => todo!(),
        CometScene::NtfDeleteInfo => todo!(),
        CometScene::RequestGuide => todo!(),
        CometScene::RequestGuideClear => todo!(),
        CometScene::RequestChangeHeadIcon => todo!(),
        CometScene::RequestChangeCharacter => todo!(),
        CometScene::RequestChangeTheme => todo!(),
        CometScene::RequestShopInfo => todo!(),
        CometScene::RequestShopBuy => todo!(),
        CometScene::RequestPieceExchange => todo!(),
        CometScene::RequestBattleFieldInfo => todo!(),
        CometScene::RequestBattleFieldRankInfo => todo!(),
        CometScene::RequestBattleFieldBegin => todo!(),
        CometScene::RequestBattleFieldFinish => todo!(),
        CometScene::RequestSummonInfo => todo!(),
        CometScene::RequestSummon => todo!(),
        CometScene::RequestSummonWeekReward => todo!(),
        CometScene::RequestSummonShopBuy => todo!(),
        CometScene::RequestChangeLanguage => todo!(),
        CometScene::RequestSocialSearchPlayer => todo!(),
        CometScene::RequestSocialPlayerProfile => todo!(),
        CometScene::RequestSocialSendAddFriendRequest => todo!(),
        CometScene::NtfSocialAddFriendRequest => todo!(),
        CometScene::RequestSocialDeleteFriend => todo!(),
        CometScene::NtfSocialDeleteFriend => todo!(),
        CometScene::RequestSocialDisposeFriendRequest => todo!(),
        CometScene::NtfSocialDisposeFriendRequest => todo!(),
        CometScene::RequestSocialPublishDynamics => todo!(),
        CometScene::RequestSocialDeleteDynamics => todo!(),
        CometScene::RequestSocialFriendDynamics => todo!(),
        CometScene::NtfSocialFriendPublicDynamic => todo!(),
        CometScene::NtfSocialFriendStatus => todo!(),
        CometScene::RequestStoryInfo => todo!(),
        CometScene::RequestStoryFinish => todo!(),
        CometScene::RequestUseItem => todo!(),
        CometScene::RequestArcadeInfo => todo!(),
        CometScene::RequestArcadeFinish => todo!(),
        CometScene::RequestChangeTitle => todo!(),
        CometScene::RequestEventInfo => todo!(),
        CometScene::RequestEventLevelGift => todo!(),
        CometScene::RequestEventStamina => todo!(),
        CometScene::RequestEventNewPlayer => todo!(),
        CometScene::RequestEventWeekCheckin => todo!(),
        CometScene::RequestEventRecharge => todo!(),
        CometScene::NtfRechargeUpdate => todo!(),
        CometScene::RequestEventLogin => todo!(),
        CometScene::RequestEventNewCharLogin => todo!(),
        CometScene::RequestEventNewThemeLogin => todo!(),
        CometScene::RequestEventNewCharRelease => todo!(),
        CometScene::RequestEventNewThemeRelease => todo!(),
        CometScene::NtfEventNewReleaseUpdate => todo!(),
        CometScene::RequestEventFriend => todo!(),
        CometScene::RequestEventBili => todo!(),
        CometScene::RequestTeamCreate => todo!(),
        CometScene::RequestTeamSearch => todo!(),
        CometScene::RequestTeamList => todo!(),
        CometScene::RequestTeamApply => todo!(),
        CometScene::RequestTeamDeclaration => todo!(),
        CometScene::RequestTeamInfo => todo!(),
        CometScene::RequestTeamPosition => todo!(),
        CometScene::RequestTeamApplyList => todo!(),
        CometScene::RequestTeamDealApply => todo!(),
        CometScene::RequestTeamKick => todo!(),
        CometScene::RequestTeamExit => todo!(),
        CometScene::RequestTeamLogs => todo!(),
        CometScene::NtfTeamChange => todo!(),
        CometScene::NtfTeamInfoChange => todo!(),
        CometScene::NtfTeamApplyChange => todo!(),
        CometScene::RequestTeamUploadSong => todo!(),
        CometScene::RequestTeamConfirmUploadSong => todo!(),
        CometScene::RequestTeamBuyItem => todo!(),
        CometScene::NtfTeamBuffList => todo!(),
        CometScene::RequestPreRankInfo => todo!(),
        CometScene::RequestPreRankBegin => todo!(),
        CometScene::RequestPreRankEnd => todo!(),
        CometScene::RequestPreRankRankList => todo!(),
        CometScene::RequestPVPBeginMatching => todo!(),
        CometScene::RequestPVPEndMatching => todo!(),
        CometScene::NtfPVPMatchSuccess => todo!(),
        CometScene::RequestPVPMatchConfirm => todo!(),
        CometScene::NtfPVPMatchConfirm => todo!(),
        CometScene::NtfPVPStartLoading => todo!(),
        CometScene::RequestPVPFinishLoading => todo!(),
        CometScene::NtfPVPFinishLoading => todo!(),
        CometScene::NtfPVPStartGame => todo!(),
        CometScene::RequestPVPSyncScore => todo!(),
        CometScene::NtfPVPSyncScore => todo!(),
        CometScene::RequestPVPUseSkill => todo!(),
        CometScene::NtfPVPUseSkill => todo!(),
        CometScene::RequestPVPFinishGame => todo!(),
        CometScene::NtfPVPFinishGame => todo!(),
        CometScene::RequestPVPCurrentState => todo!(),
        CometScene::RequestBuyProduct => todo!(),
        CometScene::RequestVerifyIOSReceipt => todo!(),
        CometScene::RequestMissingOrder => todo!(),
        CometScene::RequestSendOrder => todo!(),
        CometScene::RequestVerifyGooglePay => todo!(),
        CometScene::RequestIOSAppReceipt => todo!(),
        CometScene::RequestTestVerify => todo!(),

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
