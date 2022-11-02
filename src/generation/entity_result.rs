use crate::util::behavior_util;
use crate::util::json_decoder::SlotDetail;
use pkhex_rs::game_strings::NATURES_EN;

#[derive(Default, Clone)]
pub struct EntityResult {
    pub slot: SlotDetail,
    pub ivs: [u8; 6],
    pub group_seed: u64,
    pub index: usize,
    pub slot_seed: u64,
    pub slot_roll: f32,
    pub gen_seed: u64,
    pub alpha_seed: u64,
    pub level: usize,
    pub ec: u32,
    pub fake_tid: u32,
    pub pid: u32,
    pub shiny_xor: u32,
    pub roll_count_used: usize,
    pub roll_count_allowed: usize,
    pub species: u16,
    pub form: u16,
    pub is_shiny: bool,
    pub is_alpha: bool,
    pub ability: u8,
    pub gender: u8,
    pub nature: u8,
    pub height: u8,
    pub weight: u8,
}

impl EntityResult {
    pub fn is_oblivious(&self) -> bool {
        behavior_util::OBLIVIOUS.contains(&self.species)
    }

    pub fn is_skittish(&self) -> bool {
        behavior_util::SKITTISH.contains(&self.species)
    }

    pub fn is_aggressive(&self) -> bool {
        self.is_alpha || !(self.is_skittish() || self.is_oblivious())
    }

    pub fn get_summary(&self) -> String {
        let shiny = self.get_shiny_str();
        let ivs = format!(
            " {:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2}/{:0>2} ",
            self.ivs[0], self.ivs[1], self.ivs[2], self.ivs[3], self.ivs[4], self.ivs[5]
        );
        let nature = NATURES_EN[self.nature as usize];
        let alpha = if self.is_alpha { "α-" } else { " " };
        let not_alpha = if !self.is_alpha { " -- NOT ALPHA" } else { "" };
        let gender = match self.gender {
            2 => "",
            1 => " (F)",
            _ => " (M)",
        };

        format!(
            "{alpha}{}{gender}:{shiny}{ivs}{nature:<8}{not_alpha}",
            self.slot.name
        )
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(15);
        let shiny = self.get_shiny_str();
        let alpha = if self.is_alpha { "α-" } else { " " };
        lines.push(format!("{}{}{}", shiny, alpha, self.slot.name));

        lines.push(format!("Group Seed: {:0>16X}", self.group_seed));
        lines.push(format!("Alpha Move Seed: {:0>16X}", self.alpha_seed));
        lines.push(format!("Slot Seed: {:0>16X}", self.slot_seed));
        lines.push(format!("Slot: {:.5}", self.slot_roll as f32));
        lines.push(format!("Level: {}", self.level));
        lines.push(format!("Gen Seed: {:0>16X}", self.gen_seed));
        lines.push(format!("  EC: {:0>8X}", self.ec));
        lines.push(format!("  PID: {:0>8X}", self.pid));
        lines.push(format!("  Flawless IVs: {}", self.slot.flawless_ivs));
        lines.push(format!(
            "  IVs: {}",
            self.ivs.iter().map(|iv| iv.to_string()).collect::<String>()
        ));
        lines.push(format!("  Ability: {}", self.ability));
        lines.push(format!(
            "  Gender: {}",
            match self.gender {
                0 => 'M',
                1 => 'F',
                _ => '-',
            }
        ));
        lines.push(format!("  Nature: {}", NATURES_EN[self.nature as usize]));
        lines.push(format!("  {} | {}", self.height, self.weight));

        lines
    }

    fn get_shiny_str(&self) -> String {
        if self.is_shiny {
            format!(
                " {:2} {}",
                self.roll_count_used,
                if self.shiny_xor == 0 { '■' } else { '*' }
            )
        } else {
            String::new()
        }
    }
}
