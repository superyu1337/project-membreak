use json::JsonValue;
use memflow::{ConnectorArgs, ConnectorInventory, VirtualDMA, CachedMemoryAccess, ConnectorInstance, TimedCacheValidator, CachedVirtualTranslate, DirectTranslate, Address, VirtualMemory};
use memflow_win32::{Win32Process, Kernel, Error, Win32VirtualTranslate, Keyboard};

use crate::{menu::Config, sdk};

static BONEIDS: [usize; 6] = [8, 6, 5, 4, 3, 0];

pub fn setup() -> (Win32Process<VirtualDMA<CachedMemoryAccess<'static, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>, Win32VirtualTranslate>>, Kernel<CachedMemoryAccess<'static, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>>, Address, Address) {
    let inventory = unsafe { ConnectorInventory::scan() };
    let connector = unsafe {
        inventory.create_connector(
            "qemu_procfs", 
            &ConnectorArgs::default()
        )
    }.unwrap();

    println!("Created connector!");

    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()
        .unwrap();

    println!("Created kernel with version \"{}\" - addr: {}", kernel.kernel_info.kernel_winver, kernel.kernel_info.kernel_base);

    let kernel_clone = kernel.clone();
    let proc_info = kernel.process_info("csgo.exe").unwrap();
    let mut process = Win32Process::with_kernel(kernel_clone, proc_info);

    println!("Found csgo process - {} - addr: {}", process.proc_info.pid, process.proc_info.address);

    let mut modules = process.module_list().unwrap().into_iter();

    let client_module = modules.clone().find(|m| m.name == "client.dll")
    .ok_or(Error::Other("Could not find the client module!")).unwrap();

    println!("Found client module - addr: {}", client_module.base);

    let engine_module = modules.find(|m| m.name == "engine.dll")
    .ok_or(Error::Other("Could not find the engine module!")).unwrap();

    println!("Found engine module - addr: {}", engine_module.base);

    return (process, kernel, client_module.base, engine_module.base);
}

fn glow<T: VirtualMemory>(process: &mut Win32Process<T>, glow_object: Address, glow_index: usize) {
    let off: usize = glow_index * 0x38;
    process.virt_mem.virt_write(Address::from(glow_object + (off + 0x8)), &1f32).unwrap();
    process.virt_mem.virt_write(Address::from(glow_object + (off + 0xC)), &0.25f32).unwrap();
    process.virt_mem.virt_write(Address::from(glow_object + (off + 0x10)), &0.25f32).unwrap();
    process.virt_mem.virt_write(Address::from(glow_object + (off + 0x14)), &0.75f32).unwrap();

    process.virt_mem.virt_write(Address::from(glow_object + (off + 0x28)), &1u8).unwrap();
    process.virt_mem.virt_write(Address::from(glow_object + (off + 0x29)), &0u8).unwrap();
}

pub unsafe fn hack_loop(
    process: &mut Win32Process<impl VirtualMemory>,
    client_base: Address, 
    engine_base: Address,
    keyboard: &Keyboard,
    offsets: &JsonValue,
    config: &Config) -> bool {
    
    let keyboard_state = keyboard.state_with_process(process)
        .expect("keyboard state");

    if keyboard_state.is_down(0x77) {
        return false;
    } else {
        let local_base = sdk::get_localplayer(process, &offsets, client_base);
        let glow_object = sdk::get_glowobject(process, &offsets, client_base);
        let client_state = sdk::get_clientstate(process, &offsets, engine_base);
        let local_id = sdk::get_localid(process, local_base);
        let mut view_angles = sdk::get_viewangles(process, &offsets, client_state);

        if local_base.is_null() {
            return true;
        }

        if config.glow_enable || config.aimbot_enable {
            let local_team = sdk::get_team(process, &offsets, local_base);
            let local_wep_def_index = sdk::get_wep_def_index(process, offsets, client_base, local_base);
            let mut best_target = usize::MAX;
            let mut lowest_total_diff: f32 = f32::MAX;
            let mut target_angles = sdk::Vector3{x: 0f32, y: 0f32, z: 0f32};

            for index in 1..=64 {
                let entity_base = sdk::get_entity_by_index(process, &offsets, client_base, index);

                if !entity_base.is_null() {
                    let entity_team = sdk::get_team(process, &offsets, entity_base);
                    let entity_health = sdk::get_health(process, &offsets, entity_base);

                    if entity_team != local_team 
                        && entity_team > 1 
                        && entity_health > 0 
                        && !sdk::get_dormant(process, &offsets, entity_base) {

                        if config.radar_enable {
                            sdk::set_spotted(process, offsets, entity_base, true);
                        }
                        
                        if config.aimbot_enable && sdk::is_valid_weapon(local_wep_def_index) {

                            if config.rage_mode {
                                let aim_angles = sdk::get_aim_angles(process, &offsets, entity_base, local_base, 0);
                                let yaw_diff = libm::fabsf(view_angles.y - aim_angles.y);
                                let pitch_diff = libm::fabsf(view_angles.x - aim_angles.x);
                                let total_diff = libm::sqrtf(pitch_diff * pitch_diff + yaw_diff * yaw_diff);

                                if total_diff < lowest_total_diff {
                                    lowest_total_diff = total_diff;
                                    best_target = index;
                                    target_angles = aim_angles;
                                }
                            } else {
                                for boneindex in 0..=5 {
                                    let aim_angles = sdk::get_aim_angles(process, &offsets, entity_base, local_base, BONEIDS[boneindex]);
                                    let yaw_diff = libm::fabsf(view_angles.y - aim_angles.y);
                                    let pitch_diff = libm::fabsf(view_angles.x - aim_angles.x);
                                    let total_diff = libm::sqrtf(pitch_diff * pitch_diff + yaw_diff * yaw_diff);
    
                                    if total_diff < lowest_total_diff {
                                        lowest_total_diff = total_diff;
                                        best_target = index;
                                        target_angles = aim_angles;
                                    }
                                }
                            }
                        }

                        if config.glow_enable {
                            let entity_glowindex = sdk::get_glowindex(process, &offsets, entity_base);
                            glow(process, glow_object, entity_glowindex);
                        }
                    }
                }
            }

            if best_target != usize::MAX {
                if keyboard_state.is_down(0x01) {
                    let entity_base = sdk::get_entity_by_index(process, offsets, client_base, best_target);
                    let spotted_mask = sdk::get_spottedmask(process, offsets, entity_base);

                    if config.rage_mode {
                        let aimpunch = sdk::get_aimpunch(process, offsets, local_base);

                        view_angles.x = target_angles.x;
                        view_angles.y = target_angles.y;
                        view_angles.x -= aimpunch.x*2.0;
                        view_angles.y -= aimpunch.y*2.0;

                        sdk::math::vector_normalise(&mut view_angles);
                        sdk::set_viewangles(process, offsets, client_state, &view_angles);
                    } else {
                        if spotted_mask != 0 {
                            if (spotted_mask & (1i32 << local_id)) > 0 {
    
                                let entity_pos = sdk::get_pos(process, offsets, entity_base);
                                let local_pos = sdk::get_pos(process, offsets, local_base);
    
                                let mut diff = sdk::Vector3{x: target_angles.x - view_angles.x, y: target_angles.y - view_angles.y, z: target_angles.z - view_angles.z };
                                let player_distance = libm::sqrtf((local_pos.x-entity_pos.x)*(local_pos.x-entity_pos.x)
                                                                    + (local_pos.y-entity_pos.y)*(local_pos.y-entity_pos.y)
                                                                    + (local_pos.z-entity_pos.z)*(local_pos.z-entity_pos.z));
    
                                let yaw_diff = libm::sinf(libm::fabsf(diff.y).to_radians()) * player_distance;
                                let pitch_diff = libm::sinf(libm::fabsf(diff.x).to_radians()) * player_distance;
    
                                let dist = libm::sqrtf(pitch_diff * pitch_diff + yaw_diff * yaw_diff);
    
                                if dist < config.aimbot_fov && dist > -config.aimbot_fov {
    
                                    if config.recoil_control_enable {
                                        let aimpunch = sdk::get_aimpunch(process, offsets, local_base);
                                        diff.x -= (aimpunch.x*2.0) * (config.recoil_control_amount / 100.0);
                                        diff.y -= (aimpunch.y*2.0) * (config.recoil_control_amount / 100.0);
                                    }
                                    
                                    if config.aimbot_smoothing != 0.0 {
                                        view_angles.x += diff.x / (25.0 * (1.0 + config.aimbot_smoothing));
                                        view_angles.y += diff.y / (25.0 * (1.0 + config.aimbot_smoothing));
                                    } else {
                                        view_angles.x += diff.x;
                                        view_angles.y += diff.y;
                                    }
        
                                    sdk::math::vector_normalise(&mut view_angles);
                                    sdk::set_viewangles(process, offsets, client_state, &view_angles);
                                }
                            }
                        }
                    }
                }
            }
        }

        return true;
    }
}