use crate::parsing::entities::Entity;
use crate::parsing::stringtables::UserInfo;
use crate::parsing::variants::PropColumn;
use crate::parsing::variants::VarVec;
use crate::Demo;
use ahash::RandomState;
use phf::phf_map;
use std::collections::HashMap;
use std::collections::HashSet;

#[inline(always)]
fn create_default(col_type: i32, playback_frames: usize) -> PropColumn {
    let v = match col_type {
        0 => VarVec::I32(Vec::with_capacity(playback_frames)),
        1 => VarVec::F32(Vec::with_capacity(playback_frames)),
        2 => VarVec::F32(Vec::with_capacity(playback_frames)),
        4 => VarVec::String(Vec::with_capacity(playback_frames)),
        5 => VarVec::U64(Vec::with_capacity(playback_frames)),
        _ => panic!("INCORRECT COL TYPE"),
    };
    PropColumn { data: v }
}
#[inline(always)]
fn insert_propcolumn(
    ticks_props: &mut HashMap<String, PropColumn, RandomState>,
    ent: &Entity,
    prop_name: &String,
    playback_frames: usize,
    col_type: i32,
) {
    
    match ent.props.get(prop_name) {
        None => ticks_props
            .entry(prop_name.to_string())
            .or_insert_with(|| create_default(col_type, playback_frames))
            .data
            .push_none(),
        Some(p) => ticks_props
            .entry(prop_name.to_string())
            .or_insert_with(|| create_default(col_type, playback_frames))
            .data
            .push_propdata(p.data.clone()),
    }
}


fn insert_manager_prop( 
    ticks_props: &mut HashMap<String, PropColumn, RandomState>,
    ent: &Entity,
    prop_name: &String,
    playback_frames: usize,
    col_type: i32,
    manager: Option<&Entity>
){
    match manager {
        Some(m) => {
            let key = if ent.entity_id < 10{
                prop_name.to_owned() + "00" + &ent.entity_id.to_string()
            }else if ent.entity_id < 100{
                prop_name.to_owned() + "0" + &ent.entity_id.to_string()
            }else{
                panic!("Entity id 100 ????: id:{}", ent.entity_id);
            };
            match m.props.get(&key){
                Some(p) => {
                    ticks_props
                    .entry(prop_name.to_string())
                    .or_insert_with(|| create_default(col_type, playback_frames))
                    .data
                    .push_propdata(p.data.clone())
                }
                None => ticks_props
                        .entry(prop_name.to_string())
                        .or_insert_with(|| create_default(col_type, playback_frames))
                        .data
                        .push_none(),
            }
        }
        None => ticks_props
            .entry(prop_name.to_string())
            .or_insert_with(|| create_default(col_type, playback_frames))
            .data
            .push_none(),
    }   
}


impl Demo {
    #[inline(always)]
    pub fn collect_player_data(
        players: &HashMap<u64, UserInfo, RandomState>,
        tick: &i32,
        wanted_ticks: &HashSet<i32, RandomState>,
        wanted_players: &Vec<u64>,
        entities: &mut Vec<(u32, Entity)>,
        props_names: &Vec<String>,
        ticks_props: &mut HashMap<String, PropColumn, RandomState>,
        playback_frames: usize,
        manager_id: &Option<u32>,
    ) {
        // Collect wanted props from players
        for player in players.values() {
            if player.xuid == 0 || player.name == "GOTV" {
                continue;
            };

            // Check that we want the tick
            if wanted_ticks.contains(tick) || wanted_ticks.is_empty() {
                // Check that we want the player
                if wanted_players.contains(&player.xuid) || wanted_players.is_empty() {
                    let pl = &mut entities[player.entity_id as usize];
                    if pl.0 != 1111111 {
                        let ent = &entities[player.entity_id as usize];
                        let manager = if manager_id.is_some(){
                            Some(&entities[manager_id.unwrap() as usize].1)
                        }else{
                            None
                        };
                        // Insert all wanted non-md props
                        for prop_name in props_names {
                            let prop_type = TYPEHM[prop_name];
                            if prop_type == 10{
                                insert_manager_prop(ticks_props, &ent.1, prop_name, playback_frames, 0, manager);
                            }else{
                                insert_propcolumn(
                                    ticks_props,
                                    &ent.1,
                                    prop_name,
                                    playback_frames,
                                    prop_type,
                                );
                            }
                            
                        }
                        // Insert tick, steamid, name
                        insert_metadata(
                            player.name.clone(),
                            *tick,
                            player.xuid,
                            ticks_props,
                            playback_frames,
                        )
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn insert_metadata(
    name: String,
    tick: i32,
    xuid: u64,
    ticks_props: &mut HashMap<String, PropColumn, RandomState>,
    playback_frames: usize,
) {
    ticks_props
        .entry("tick".to_string())
        .or_insert_with(|| create_default(0, playback_frames))
        .data
        .push_i32(tick);

    ticks_props
        .entry("name".to_string())
        .or_insert_with(|| create_default(4, playback_frames))
        .data
        .push_string(name.to_string());

    ticks_props
        .entry("steamid".to_string())
        .or_insert_with(|| create_default(5, playback_frames))
        .data
        .push_u64(xuid);
}

pub static TYPEHM: phf::Map<&'static str, i32> = phf_map! {
    "m_flNextAttack" => 1,
    "m_bDuckOverride" => 0,
    "m_flStamina" => 1,
    "m_flVelocityModifier" => 1,
    "m_iShotsFired" => 0,
    "m_nQuestProgressReason" => 0,
    "m_vecOrigin" => 2,
    "m_vecOrigin_X" => 1,
    "m_vecOrigin_Y" => 1,
    "m_vecOrigin[2]" => 1,
    "m_aimPunchAngle" => 2,
    "m_aimPunchAngle_X" => 1,
    "m_aimPunchAngle_Y" => 1,
    "m_aimPunchAngleVel" => 2,
    "m_aimPunchAngleVel_X" => 1,
    "m_aimPunchAngleVel_Y" => 1,
    "m_audio.soundscapeIndex" => 0,
    "m_bDucked" => 0,
    "m_bDucking" => 0,
    "m_bWearingSuit" => 0,
    "m_chAreaBits.000" => 0,
    "m_chAreaBits.001" => 0,
    "m_chAreaPortalBits.002" => 0,
    "m_flFOVRate" => 1,
    "m_flFallVelocity" => 1,
    "m_flLastDuckTime" => 1,
    "m_viewPunchAngle" => 2,
    "m_viewPunchAngle_X" => 1,
    "m_viewPunchAngle_Y" => 1,
    "m_flDeathTime" => 1,
    "m_flNextDecalTime" => 1,
    "m_hLastWeapon" => 0,
    "m_hTonemapController" => 0,
    "m_nNextThinkTick" => 0,
    "m_nTickBase" => 0,
    "m_vecBaseVelocity" => 2,
    "m_vecBaseVelocity_X" => 1,
    "m_vecBaseVelocity_Y" => 1,
    "m_vecVelocity[0]" => 1,
    "m_vecVelocity[1]" => 1,
    "m_vecVelocity[2]" => 1,
    "m_vecViewOffset[2]" => 1,
    "m_ArmorValue" => 0,
    "m_usSolidFlags" => 0,
    "m_vecMaxs" => 2,
    "m_vecMaxs_X" => 1,
    "m_vecMaxs_Y" => 1,
    "m_vecMins" => 2,
    "m_vecMins_X" => 1,
    "m_vecMins_Y" => 1,
    "m_LastHitGroup" => 0,
    "m_afPhysicsFlags" => 0,
    "m_angEyeAngles[0]" => 1,
    "m_angEyeAngles[1]" => 1,
    "m_bAnimatedEveryTick" => 0,
    "m_bClientSideRagdoll" => 0,
    "m_bHasDefuser" => 0,
    "m_bHasHelmet" => 0,
    "m_bHasMovedSinceSpawn" => 0,
    "m_bInBombZone" => 0,
    "m_bInBuyZone" => 0,
    "m_bIsDefusing" => 0,
    "m_bIsHoldingLookAtWeapon" => 0,
    "m_bIsLookingAtWeapon" => 0,
    "m_bIsScoped" => 0,
    "m_bIsWalking" => 0,
    "m_bResumeZoom" => 0,
    "m_bSpotted" => 0,
    "m_bSpottedByMask.000" => 0,
    "m_bStrafing" => 0,
    "m_bWaitForNoAttack" => 0,
    "m_fEffects" => 0,
    "m_fFlags" => 0,
    "m_fMolotovDamageTime" => 1,
    "m_fMolotovUseTime" => 1,
    "m_flDuckAmount" => 1,
    "m_flDuckSpeed" => 1,
    "m_flFOVTime" => 1,
    "m_flFlashDuration" => 1,
    "m_flFlashMaxAlpha" => 1,
    "m_flGroundAccelLinearFracLastTime" => 1,
    "m_flLastMadeNoiseTime" => 1,
    "m_flLowerBodyYawTarget" => 1,
    "m_flProgressBarStartTime" => 1,
    "m_flSimulationTime" => 0,
    "m_flThirdpersonRecoil" => 1,
    "m_flTimeOfLastInjury" => 1,
    "m_hActiveWeapon" => -1,
    "m_hColorCorrectionCtrl" => 0,
    "m_hGroundEntity" => 0,
    "m_hMyWeapons.000" => 0,
    "m_hMyWeapons.001" => 0,
    "m_hMyWeapons.002" => 0,
    "m_hMyWeapons.003" => 0,
    "m_hMyWeapons.004" => 0,
    "m_hMyWeapons.005" => 0,
    "m_hMyWeapons.006" => 0,
    "m_hMyWeapons.007" => 0,
    "m_hMyWeapons.008" => 0,
    "m_hObserverTarget" => 0,
    "m_hPlayerPing" => 0,
    "m_hPostProcessCtrl" => 0,
    "m_hRagdoll" => 0,
    "m_hViewModel" => 5,
    "m_hZoomOwner" => 0,
    "m_iAccount" => 0,
    "m_iAddonBits" => 0,
    "m_iAmmo.014" => 0,
    "m_iAmmo.015" => 0,
    "m_iAmmo.016" => 0,
    "m_iAmmo.017" => 0,
    "m_iAmmo.018" => 0,
    "m_iClass" => 0,
    "m_iDeathPostEffect" => 0,
    "m_iFOV" => 0,
    "m_iFOVStart" => 0,
    "m_iHealth" => 0,
    "m_iMoveState" => 0,
    "m_iNumRoundKills" => 0,
    "m_iNumRoundKillsHeadshots" => 0,
    "m_iObserverMode" => 0,
    "m_iPendingTeamNum" => 0,
    "m_iPlayerState" => 0,
    "m_iPrimaryAddon" => 0,
    "m_iProgressBarDuration" => 0,
    "m_iSecondaryAddon" => 0,
    "m_iStartAccount" => 0,
    "m_iTeamNum" => 0,
    "m_lifeState" => 0,
    "m_nForceBone" => 0,
    "m_nHeavyAssaultSuitCooldownRemaining" => 0,
    "m_nLastConcurrentKilled" => 0,
    "m_nLastKillerIndex" => 0,
    "m_nModelIndex" => 0,
    "m_nRelativeDirectionOfLastInjury" => 0,
    "m_nWaterLevel" => 0,
    "m_rank.005" => 0,
    "m_szLastPlaceName" => 4,
    "m_totalHitsOnServer" => 0,
    "m_ubEFNoInterpParity" => 0,
    "m_unCurrentEquipmentValue" => 0,
    "m_unFreezetimeEndEquipmentValue" => 0,
    "m_unMusicID" => 0,
    "m_unRoundStartEquipmentValue" => 0,
    "m_unTotalRoundDamageDealt" => 0,
    "m_vecForce" => 2,
    "m_vecForce_X" => 1,
    "m_vecForce_Y" => 1,
    "m_vecLadderNormal" => 2,
    "m_vecLadderNormal_X" => 1,
    "m_vecLadderNormal_Y" => 1,
    "m_vecPlayerPatchEconIndices.002" => 0,
    "movetype" => 0,
    "pl.deadflag" => 0,
    "m_bSilencerOn" => 0,
    "m_bReloadVisuallyComplete" => 1,
    "m_iCompetitiveRanking" => 10,
    "m_iPing" => 10,
    "m_iTeam" => 10,
    "m_iScore" => 10,
    "m_iDeaths" => 10,
    "m_iKills" => 10,
    "m_iAssists" => 10,
    "m_iMVPs" => 10,
    "m_iArmor" => 10,
    "m_iCompetitiveWins" => 10,
};
