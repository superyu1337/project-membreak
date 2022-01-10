use dataview::Pod;
use json::JsonValue;
use memflow::{VirtualMemory, Address};
use memflow_win32::Win32Process;

pub mod offsets;
pub mod math;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct Matrix3x4 {
    row0: [f32; 4],
    row1: [f32; 4],
    row2: [f32; 4],
}

unsafe impl Pod for Matrix3x4 {}
unsafe impl Pod for Vector3 {}

// Important Stuff
pub fn get_clientstate<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_base: Address) -> Address {
    let offset = offsets::get_signature(offsets, "dwClientState").unwrap();
    process.virt_mem.virt_read_addr32(client_base + offset).expect("get_clientstate")
}

pub fn get_glowobject<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_base: Address) -> Address {
    let offset = offsets::get_signature(offsets, "dwGlowObjectManager").unwrap();
    process.virt_mem.virt_read_addr32(client_base + offset).expect("get_glowobject")
}

pub fn get_localid<T: VirtualMemory>(process: &mut Win32Process<T>, local: Address) -> i32 {
    let data: i32 = process.virt_mem.virt_read(local + 0x64).expect("get_localid");
    return data - 1;
}

pub fn get_viewangles<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_state: Address) -> Vector3 {
    let offset = offsets::get_signature(offsets, "dwClientState_ViewAngles").unwrap();
    process.virt_mem.virt_read(client_state + offset).expect("get_viewangles")
}

// Viewangles
pub fn set_viewangles<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_state: Address, angles: &Vector3) {
    let offset = offsets::get_signature(offsets, "dwClientState_ViewAngles").unwrap();
    process.virt_mem.virt_write(client_state + offset, angles).expect("get_viewangles");
}

// Get Players
pub fn get_localplayer<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_base: Address) -> Address {
    let offset = offsets::get_signature(offsets, "dwLocalPlayer").unwrap();
    process.virt_mem.virt_read_addr32(client_base + offset).expect("set_viewangles")
}

pub fn get_entity_by_index<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_base: Address, index: usize) -> Address {
    let offset = offsets::get_signature(offsets, "dwEntityList").unwrap();
    process.virt_mem.virt_read_addr32(client_base + offset + (index * 0x10)).expect("get_player_by_index")
}

// Player related stuff
pub fn get_health<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> i32 {
    let offset = offsets::get_netvar(offsets, "m_iHealth").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_health")
}

pub fn get_team<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> i32 {
    let offset = offsets::get_netvar(offsets, "m_iTeamNum").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_team")
}

pub fn get_glowindex<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> usize {
    let offset = offsets::get_netvar(offsets, "m_iGlowIndex").unwrap();
    let data: i32 = process.virt_mem.virt_read(entity_base + offset).expect("get_glowindex");
    return data as usize;
}

pub fn get_dormant<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> bool {
    let offset = offsets::get_signature(offsets, "m_bDormant").unwrap();
    let data: u8 = process.virt_mem.virt_read(entity_base + offset).expect("get_dormant");
    return data != 0;
}

pub fn get_bone_matrix<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> Address {
    let offset = offsets::get_netvar(offsets, "m_dwBoneMatrix").unwrap();
    process.virt_mem.virt_read_addr32(entity_base + offset).expect("get_bone_matrix")
}

pub fn get_pos<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> Vector3 {
    let offset = offsets::get_netvar(offsets, "m_vecOrigin").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_pos")
}

pub fn get_viewoffset<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> Vector3 {
    let offset = offsets::get_netvar(offsets, "m_vecViewOffset").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_viewoffset")
}

pub fn get_aimpunch<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> Vector3 {
    let offset = offsets::get_netvar(offsets, "m_aimPunchAngle").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_viewoffset")
}

pub fn set_spotted<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address, value: bool) {
    let offset = offsets::get_netvar(offsets, "m_bSpotted").unwrap();
    process.virt_mem.virt_write(entity_base + offset, &(value as u8)).expect("set_spotted");
}

pub fn get_spottedmask<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address) -> i32 {
    let offset = offsets::get_netvar(offsets, "m_bSpottedByMask").unwrap();
    process.virt_mem.virt_read(entity_base + offset).expect("get_spottedmask")
}

pub fn get_bone_pos<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address, bone_id: usize) -> Vector3 {
    let bone_matrix_address = get_bone_matrix(process, offsets, entity_base);
    let bone_matrix: Matrix3x4 = process.virt_mem.virt_read(bone_matrix_address + (bone_id * 0x30)).unwrap();
    Vector3{x: bone_matrix.row0[3], y: bone_matrix.row1[3], z: bone_matrix.row2[3]}
}

pub fn get_aim_angles<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, entity_base: Address, local_base: Address, boneid: usize) -> Vector3 {
    let mut local_pos = get_pos(process, offsets, local_base);
    let local_off = get_viewoffset(process, offsets, local_base);
    local_pos.x += local_off.x;
    local_pos.y += local_off.y;
    local_pos.z += local_off.z;
    let entity_pos = get_bone_pos(process, offsets, entity_base, boneid);
    let mut target_angles = math::calc_angles(local_pos, entity_pos);
    math::vector_normalise(&mut target_angles);
    return target_angles;
}

pub fn get_wep_def_index<T: VirtualMemory>(process: &mut Win32Process<T>, offsets: &JsonValue, client_base: Address, entity_base: Address) -> u16 {
    let offset = offsets::get_netvar(offsets, "m_hActiveWeapon").unwrap();
    let offset2 = offsets::get_netvar(offsets, "m_iItemDefinitionIndex").unwrap();

    let weapon_handle: usize = process.virt_mem.virt_read(entity_base + offset).expect("get_wep_def_index 0");
    let weapon_index = weapon_handle & 0xFFF;
    let weapon_base = get_entity_by_index(process, offsets, client_base, weapon_index - 1);
    process.virt_mem.virt_read(weapon_base + offset2).expect("get_wep_def_index 1")
}

pub fn is_valid_weapon(item_def_index: u16) -> bool {

    if item_def_index > 0 && item_def_index < 20 {
        return true;
    }

    if item_def_index > 20 && item_def_index < 37 {
        return true;
    }

    if item_def_index > 37 && item_def_index < 41 {
        return true;
    }

    if item_def_index == 60 && item_def_index == 61 {
        return true;
    }

    if item_def_index == 63 && item_def_index == 64 {
        return true;
    }

    false
}