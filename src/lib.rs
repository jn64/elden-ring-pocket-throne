use std::time::Duration;

use eldenring::{
    cs::{
        CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, ChrInsExt, GeometrySpawnParameters,
        WorldChrMan,
    },
    fd4::FD4TaskData,
    position::PositionDelta,
    rotation::EulerAngles,
    util::input,
};
use fromsoftware_shared::{FromStatic, SharedTaskImpExt};

#[cfg(debug_assertions)]
use std::{fs::File, io::Write};

mod config;

// Godrick's Throne
static THRONE_ID: &str = "AEG210_285";
static THRONE_SCALE: f32 = 1.0;

#[unsafe(no_mangle)]
/// # Safety
///
/// This is exposed this way such that windows LoadLibrary API can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }

    // Kick off new thread.
    std::thread::spawn(|| {
        let key = config::get_key();

        let cs_task = CSTaskImp::wait_for_instance(Duration::MAX).unwrap();

        // Register a new task with the game to happen every frame during the game loop's
        // ChrIns_PostPhysics phase because all the physics calculations have ran at this
        // point.
        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                if !input::is_key_pressed(key) {
                    return;
                }

                let Some(player) = unsafe { WorldChrMan::instance() }
                    .ok()
                    .and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                let Some(block_geom_data) = unsafe { CSWorldGeomMan::instance_mut() }
                    .ok()
                    .and_then(|wgm| wgm.geom_block_data_by_id_mut(&player.chr_ins.block_id()))
                else {
                    return;
                };

                // Grab physics module from player.
                let physics = &player.chr_ins.modules.physics;

                // Make a directional vector that points backwards based on the
                // player's rotation. See fromsoftware-rs/examples/debug-line.
                let directional_vector = {
                    let backward = glam::vec3(0.0, 0.0, 1.0);
                    glam::Quat::from(physics.orientation).mul_vec3(backward)
                };

                let EulerAngles(_player_pitch, player_yaw, _player_roll) =
                    physics.orientation.to_euler_angles();

                let throne = block_geom_data.spawn_geometry(
                    THRONE_ID,
                    &GeometrySpawnParameters {
                        position: player.block_position
                            + PositionDelta(
                                directional_vector.x,
                                directional_vector.y,
                                directional_vector.z,
                            ),
                        rot_x: 0.0,
                        // Throne faces south by default (rot_y = 0.0).
                        // Make the throne face the same direction as the player.
                        rot_y: player_yaw,
                        rot_z: 0.0,
                        scale_x: THRONE_SCALE,
                        scale_y: THRONE_SCALE,
                        scale_z: THRONE_SCALE,
                    },
                );

                let throne_params =
                    unsafe { throne.unwrap().as_mut().info.asset_geometry_param.as_mut() };

                // Dump params. See eldenring::param::ASSET_GEOMETORY_PARAM_ST
                // File is created next to eldenring.exe.
                #[cfg(debug_assertions)]
                if let Ok(mut params_file) = File::create_new(format!("{THRONE_ID}.txt")) {
                    _ = writeln!(params_file, "{:#?}", throne_params);
                    // Above is missing some fields like is_break_by_*
                    _ = writeln!(
                        params_file,
                        "is_break_by_player_collide: {:#?}",
                        throne_params.is_break_by_player_collide()
                    );
                    _ = writeln!(
                        params_file,
                        "is_break_by_enemy_collide: {:#?}",
                        throne_params.is_break_by_enemy_collide()
                    );
                    _ = writeln!(
                        params_file,
                        "is_break_by_chr_ride: {:#?}",
                        throne_params.is_break_by_chr_ride()
                    );
                    _ = writeln!(
                        params_file,
                        "is_disable_break_for_first_appear: {:#?}",
                        throne_params.is_disable_break_for_first_appear()
                    );
                    _ = writeln!(
                        params_file,
                        "is_anim_break: {:#?}",
                        throne_params.is_anim_break()
                    );
                    _ = writeln!(
                        params_file,
                        "is_damage_cover: {:#?}",
                        throne_params.is_damage_cover()
                    );
                    _ = writeln!(
                        params_file,
                        "is_attack_backlash: {:#?}",
                        throne_params.is_attack_backlash()
                    );
                    _ = writeln!(
                        params_file,
                        "is_break_by_hugeenemy_collide: {:#?}",
                        throne_params.is_break_by_hugeenemy_collide()
                    );
                }

                // Unbreakable chairs/thrones (e.g AEG210_285 and AEG030_889)
                // seem to have these params in common:
                //   hit_create_type: 0
                //   behavior_type: 1
                //   hp: -1
                //   anim_break_id_max: 0
                //   is_anim_break: false
                //   is_attack_backlash: true
                //
                // Changing behavior_type to 0 and hp to positive makes them
                // breakable. They conveniently disappear when breaking
                // (no animation or debris).
                throne_params.set_behavior_type(0);
                // 200 HP survives a few jumps, but is still easy to break
                // with attacks.
                throne_params.set_hp(200);
                throne_params.set_defense(0);

                // Disable weapon bounce
                throne_params.set_is_attack_backlash(false);
            },
            // Task group in which physics calculations are already done.
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    // Signal that DllMain executed successfully
    true
}
