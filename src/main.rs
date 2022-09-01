use std::fs;

use rfkill_rs::RfkillType;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        op @ ("block" | "unblock") => {
            let block = op == "block";
            if let Ok(index) = args[2].parse::<u32>() {
                rfkill_rs::block_index(block, index).unwrap();
                return;
            }
            let ty = match args[2].as_str() {
                "all" => RfkillType::All,
                "wifi" => RfkillType::Wlan,
                "bluetooth" => RfkillType::Bluetooth,
                _ => {
                    println!("Usage: [un]block {{index | device}}");
                    return;
                }
            };
            rfkill_rs::block_type(block, ty).unwrap();
        }
        "list" => {
            for event in rfkill_rs::list().unwrap() {
                let idx = event.idx;
                let type_ = event.type_;
                let name =
                    fs::read_to_string(format!("/sys/class/rfkill/rfkill{idx}/name")).unwrap();
                let name = name.trim_end();
                println!("{idx}: {name}: {type_:?}");
                println!("\tSoft blocked: {}", event.soft);
                println!("\tHard blocked: {}", event.soft);
            }
        }
        _ => {
            println!("Commands: block | unblock | list")
        }
    }
}
