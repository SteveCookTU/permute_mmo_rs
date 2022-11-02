use clap::Parser;
use permute_mmo_rs::generation::EntityResult;
use permute_mmo_rs::permutation::Advance;
use permute_mmo_rs::permuter;
use permute_mmo_rs::structure::{
    MassOutbreakSet8a, MassiveOutbreakArea8a, MassiveOutbreakSet8a, MassiveOutbreakSpawnerStatus,
};
use permute_mmo_rs::util::area_util::AREA_TABLE;
use permute_mmo_rs::util::{area_util, get_summary, SpawnInfo};
use pkhex_rs::game_strings::SPECIES_EN;
use std::cell::RefCell;
use std::rc::Rc;
use sysbot_rs::SysBotClient;

fn satisfy_criteria(entity: &EntityResult, _advances: &[Advance]) -> bool {
    entity.is_shiny
}

#[derive(Parser)]
struct PermuteMmo {
    #[arg(help = "IP of the switch you are connecting to")]
    ip: String,
    #[arg(
        long,
        short,
        help = "Port to connect to on the switch. Default is 6000",
        default_value_t = 6000
    )]
    port: u16,
}

fn main() {
    let args: PermuteMmo = PermuteMmo::parse();

    if let Ok(client) = SysBotClient::connect(&args.ip, args.port) {
        if let Ok(mo_data) = client.pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x20], 0x190) {
            if let Ok(mmo_data) =
                client.pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x1B0], 0x3980)
            {
                let mo_data = &mo_data[..(mo_data.len() - 1)];
                let mmo_data = &mmo_data[..(mmo_data.len() - 1)];
                permute_massive_mass_outbreak(mmo_data, Some(satisfy_criteria));
                println!("\n==========");
                permute_block_mass_outbreak(mo_data, Some(satisfy_criteria));
            } else {
                println!("Failed to get massive outbreak data from console!")
            }
        } else {
            println!("Failed to get outbreak data from console!")
        }
    } else {
        println!("Failed to connect to console!");
    }
}

fn permute_massive_mass_outbreak(
    data: &[u8],
    criteria: Option<fn(&EntityResult, &[Advance]) -> bool>,
) {
    println!("Permuting Mass Outbreaks.");
    let block: MassiveOutbreakSet8a = data.into();
    for i in 0..MassiveOutbreakSet8a::AREA_COUNT {
        let area = block[i];
        let area_name = if let Some(area_name) = area_util::AREA_TABLE.get(&area.area_hash) {
            area_name
        } else {
            AREA_TABLE.get(&0).unwrap()
        };
        if !area.is_active {
            println!("No outbreak in {area_name}");
            continue;
        }

        debug_assert!(area.is_valid());
        let mut has_printed_area_mmo = false;
        for j in 0..MassiveOutbreakArea8a::SPAWNER_COUNT {
            let spawner = area[j];
            if spawner.status() == MassiveOutbreakSpawnerStatus::None {
                continue;
            }

            debug_assert!(spawner.has_base());

            let seed = spawner.group_seed;
            let spawn: Rc<RefCell<SpawnInfo>> = spawner.into();

            let result = permuter::permute(spawn.clone(), seed, 15, criteria);
            if !result.has_results() {
                continue;
            }

            if !has_printed_area_mmo {
                println!("Found paths for Massive mass Outbreaks in {area_name}.\n==========");
                has_printed_area_mmo = true;
            }

            println!(
                "Spawner {} at ({:.1},{:.1},{}) shows {}",
                j + 1,
                spawner.x,
                spawner.y,
                spawner.z,
                SPECIES_EN[spawner.display_species as usize]
            );
            println!("{}", get_summary(&spawn, "Parameters: "));
            println!("Seed: {}", seed);
            for line in result.get_lines() {
                println!("{}", line);
            }
            println!();
        }

        if !has_printed_area_mmo {
            println!("Found no results for any Massive Mass Outbreak in {area_name}");
        } else {
            println!("Done permuting area.\n==========");
        }
    }
}

fn permute_block_mass_outbreak(
    data: &[u8],
    criteria: Option<fn(&EntityResult, &[Advance]) -> bool>,
) {
    println!("Permuting mass Outbreaks.");
    let block: MassOutbreakSet8a = data.into();
    for i in 0..MassOutbreakSet8a::AREA_COUNT {
        let spawner = block[i];
        let area_name = if let Some(area_name) = area_util::AREA_TABLE.get(&spawner.area_hash) {
            area_name
        } else {
            area_util::AREA_TABLE.get(&0).unwrap()
        };

        if !spawner.has_outbreak() {
            println!("No outbreak in {area_name}");
            continue;
        }

        debug_assert!(spawner.is_valid());

        let seed = spawner.group_seed;
        let spawn: Rc<RefCell<SpawnInfo>> = spawner.into();
        let result = permuter::permute(spawn.clone(), seed, 15, criteria);
        if !result.has_results() {
            println!(
                "Found no paths for {} Mass Outbreak in {area_name}",
                SPECIES_EN[spawner.display_species as usize]
            );
            continue;
        }

        println!(
            "Found paths for {} Mass Outbreak in {area_name}:",
            SPECIES_EN[spawner.display_species as usize]
        );
        println!("==========");
        println!(
            "Spawner at ({:.1}, {:.1}, {:}) shows {}",
            spawner.x, spawner.y, spawner.z, SPECIES_EN[spawner.display_species as usize]
        );
        println!("{}", get_summary(&spawn, "Parameters: "));
        println!("Seed: {}", seed);
        for line in result.get_lines() {
            println!("{}", line);
        }
        println!();
    }
    println!("Done permuting Mass Outbreaks.");
    println!("==========");
}
