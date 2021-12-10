use json::JsonValue;

fn download_newest() -> String {
    let resp = reqwest::blocking::get("https://raw.githubusercontent.com/frk1/hazedumper/master/csgo.json")
        .unwrap()
        .text()
        .unwrap();

    resp
}

fn check_updated(cur_version: usize) -> bool {
    let newest = download_newest();
    let newest = json::parse(&newest).expect("Error while parsing!");

    if newest["timestamp"].as_usize().unwrap() > cur_version {
        return false;
    }

    true
}

fn save_newest() {
    let newest = download_newest();
    std::fs::write("csgo.json", newest.as_bytes())
        .expect("Could not save newest offset file.");
}

pub fn get_offsets() -> JsonValue {
    let file_content = std::fs::read_to_string("csgo.json");

    if file_content.is_err() {
        println!("Offset file not found or corrupted, redownloading.");
        save_newest();
        get_offsets()
    } else {
        let parsed = json::parse(&file_content.unwrap()).expect("Error while parsing!");
        
        if !check_updated(parsed["timestamp"].as_usize().unwrap()) {
            println!("Offset file outdated, redownloading.");
            save_newest();
            get_offsets()
        } else {
            println!("Loaded offsets!");
            return parsed;
        }
    }
}

pub fn get_signature(offsets: &JsonValue, name: &str) -> Result<usize, usize> {
    offsets["signatures"][name].as_usize().ok_or(0x10)
}

pub fn get_netvar(offsets: &JsonValue, name: &str) -> Result<usize, usize> {
    offsets["netvars"][name].as_usize().ok_or(0x10)
}