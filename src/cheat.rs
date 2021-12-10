use json::JsonValue;
use memflow::{ConnectorArgs, ConnectorInventory, VirtualDMA, CachedMemoryAccess, ConnectorInstance, TimedCacheValidator, CachedVirtualTranslate, DirectTranslate, Address, VirtualMemory, PhysicalMemory, VirtualTranslate};
use memflow_win32::{Win32Process, Kernel, Error, Win32VirtualTranslate, kernel, Keyboard};

use crate::{menu::Config, sdk};

pub fn setup() -> (Win32Process<VirtualDMA<CachedMemoryAccess<'static, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>, Win32VirtualTranslate>>, Kernel<CachedMemoryAccess<'static, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>>, Address, Address) {
    let inventory = unsafe { ConnectorInventory::scan() };
    /*let connector = unsafe {
        inventory.create_connector(
            "qemu_procfs", 
            &ConnectorArgs::default()
        )
    }.unwrap();*/
    let connector = unsafe { inventory.create_connector_default("qemu_procfs")? };


    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()
        .unwrap();

    let kernel_clone = kernel.clone();
    let proc_info = kernel.process_info("csgo.exe").unwrap();
    let mut process = Win32Process::with_kernel(kernel_clone, proc_info);

    let mut modules = process.module_list().unwrap().into_iter();

    let client_module = modules.find(|m| m.name == "client.dll")
    .ok_or(Error::Other("Could not find the client module!")).unwrap();

    let engine_module = modules.find(|m| m.name == "engine.dll")
    .ok_or(Error::Other("Could not find the engine module!")).unwrap();

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

pub unsafe fn hack_loop<T: PhysicalMemory, V: VirtualTranslate>(
    process: &mut Win32Process<impl VirtualMemory>, 
    kernel: &Kernel<T, V>, 
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
        let local_id = sdk::get_localid(process, &offsets, local_base) - 1;
        let mut view_angles = sdk::get_viewangles(process, &offsets, client_state);

        if local_base.is_null() {
            return true;
        }

        if config.glow_enable || config.aimbot_enable {
            let local_team = sdk::get_team(process, &offsets, local_base);
            let mut best_target = usize::MAX;
            let mut lowest_yaw_diff: f32 = f32::MAX;
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
                        
                        if config.aimbot_enable {
                            let aim_angles = sdk::get_aim_angles(process, &offsets, entity_base, local_base, 8);
                            let yaw_difference = libm::fabsf(view_angles.y - aim_angles.y);

                            if yaw_difference < lowest_yaw_diff {
                                lowest_yaw_diff = yaw_difference;
                                best_target = index;
                                target_angles = aim_angles;
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
                    
                }
            }
        }

        return true;
    }
}

// osmose, hypotonisch, etc.    