use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use std::{collections::HashMap, fs, path::Path};

use crate::remote_assets::DownloadReport;

const REPEAT_SUPPRESSION_SECONDS: f64 = 0.08;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioCue {
    Select,
    Confirm,
    Back,
    Warning,
    Ready,
    WeaponFire,
    ShieldImpact,
    HullImpact,
    Explosion,
}

impl AudioCue {
    pub const ALL: [Self; 9] = [
        Self::Select,
        Self::Confirm,
        Self::Back,
        Self::Warning,
        Self::Ready,
        Self::WeaponFire,
        Self::ShieldImpact,
        Self::HullImpact,
        Self::Explosion,
    ];

    pub const fn paths(self) -> &'static [&'static str] {
        match self {
            Self::Select => &["audio/ui/select.ogg"],
            Self::Confirm => &["audio/ui/confirm.ogg"],
            Self::Back => &["audio/ui/back.ogg"],
            Self::Warning => &["audio/ui/error.ogg"],
            Self::Ready => &["audio/ui/open.ogg"],
            Self::WeaponFire => &[
                "audio/effects/laser-small.ogg",
                "audio/effects/laser-large.ogg",
                "audio/effects/laser-retro.ogg",
            ],
            Self::ShieldImpact => &["audio/effects/force-field.ogg"],
            Self::HullImpact => &["audio/effects/impact-metal.ogg"],
            Self::Explosion => &["audio/effects/explosion-crunch.ogg"],
        }
    }
}

pub struct AudioManager {
    sounds: HashMap<AudioCue, Vec<Sound>>,
    custom_sounds: HashMap<String, Sound>,
    master_volume: f32,
    last_played: HashMap<AudioCue, f64>,
    last_custom_played: HashMap<String, f64>,
    next_variant: HashMap<AudioCue, usize>,
}

impl AudioManager {
    pub fn empty(master_volume: f32) -> Self {
        Self {
            sounds: HashMap::new(),
            custom_sounds: HashMap::new(),
            master_volume: master_volume.clamp(0.0, 1.0),
            last_played: HashMap::new(),
            last_custom_played: HashMap::new(),
            next_variant: HashMap::new(),
        }
    }

    pub async fn load_from_report(
        report: Option<&DownloadReport>,
        cache_root: impl AsRef<Path>,
        master_volume: f32,
    ) -> Self {
        let mut manager = Self::empty(master_volume);
        let Some(report) = report else {
            return manager;
        };

        for cue in AudioCue::ALL {
            let mut variants = Vec::new();
            for path in cue.paths() {
                if !report.ready_paths.iter().any(|ready| ready == path) {
                    continue;
                }
                let path = cache_root.as_ref().join(&report.release_id).join(path);
                let Ok(bytes) = fs::read(path) else {
                    continue;
                };
                if let Ok(sound) = load_sound_from_bytes(&bytes).await {
                    variants.push(sound);
                }
            }
            if !variants.is_empty() {
                manager.sounds.insert(cue, variants);
            }
        }
        manager
    }

    pub async fn load_custom_paths<'a>(&mut self, paths: impl Iterator<Item = &'a str>) {
        for path in paths {
            if self.custom_sounds.contains_key(path) {
                continue;
            }
            let Ok(bytes) = fs::read(path) else {
                continue;
            };
            if let Ok(sound) = load_sound_from_bytes(&bytes).await {
                self.custom_sounds.insert(path.to_string(), sound);
            }
        }
    }

    pub fn play(&mut self, cue: AudioCue) -> bool {
        if self.master_volume <= 0.0 {
            return false;
        }
        let Some(sounds) = self.sounds.get(&cue) else {
            return false;
        };
        let now = macroquad::prelude::get_time();
        if self
            .last_played
            .get(&cue)
            .is_some_and(|last| now - last < REPEAT_SUPPRESSION_SECONDS)
        {
            return false;
        }
        let variant_index = self.next_variant.entry(cue).or_default();
        let sound = &sounds[*variant_index % sounds.len()];
        *variant_index = (*variant_index + 1) % sounds.len();
        self.last_played.insert(cue, now);
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: self.master_volume,
            },
        );
        true
    }

    pub fn play_custom_or(&mut self, path: Option<&str>, fallback: AudioCue) -> bool {
        let Some(path) = path else {
            return self.play(fallback);
        };
        if self.master_volume <= 0.0 {
            return false;
        }
        let Some(sound) = self.custom_sounds.get(path) else {
            return self.play(fallback);
        };
        let now = macroquad::prelude::get_time();
        if self
            .last_custom_played
            .get(path)
            .is_some_and(|last| now - last < REPEAT_SUPPRESSION_SECONDS)
        {
            return false;
        }
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: self.master_volume,
            },
        );
        self.last_custom_played.insert(path.to_string(), now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cues_use_stable_manifest_paths() {
        assert_eq!(AudioCue::Select.paths()[0], "audio/ui/select.ogg");
        assert_eq!(AudioCue::Warning.paths()[0], "audio/ui/error.ogg");
        assert_eq!(AudioCue::WeaponFire.paths().len(), 3);
        assert_eq!(AudioCue::ALL.len(), 9);
    }

    #[test]
    fn volume_is_clamped_for_mute_and_safety() {
        let manager = AudioManager {
            sounds: HashMap::new(),
            custom_sounds: HashMap::new(),
            master_volume: 2.0_f32.clamp(0.0, 1.0),
            last_played: HashMap::new(),
            last_custom_played: HashMap::new(),
            next_variant: HashMap::new(),
        };
        assert_eq!(manager.master_volume, 1.0);
        assert!(!manager.sounds.contains_key(&AudioCue::Confirm));
    }
}
