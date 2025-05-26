use std::sync::Arc;
use std::sync::LazyLock;

use egui::FontData;
use egui::FontDefinitions;
use egui::FontFamily;
use egui::FontId;

// TODO: egui does not really support bold (?!)

pub struct FontInfo {
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub family: FontFamily,
}

impl FontInfo {
    pub fn sized(&self, size: f32) -> FontId {
        FontId::new(size, self.family.clone())
    }

    pub fn register_as(&self, fonts: &mut FontDefinitions, family: FontFamily) {
        // Name -> Data
        fonts
            .font_data
            .entry(self.name.to_owned())
            .or_insert_with(|| Arc::new(FontData::from_static(self.bytes)));

        // Family -> Name
        fonts
            .families
            .entry(family)
            .or_default()
            .insert(0, self.name.to_owned());
    }

    pub fn register(&self, fonts: &mut FontDefinitions) {
        self.register_as(fonts, self.family.clone());
    }
}

pub static FONTIN_REGULAR: LazyLock<FontInfo> = LazyLock::new(|| {
    let name = "Fontin";
    FontInfo {
        name,
        bytes: include_bytes!("./Fontin-Regular.ttf"),
        family: FontFamily::Name(name.into()),
    }
});

pub static FONTIN_SMALL_CAPS: LazyLock<FontInfo> = LazyLock::new(|| {
    let name = "Fontin Small Caps";
    FontInfo {
        name,
        bytes: include_bytes!("./Fontin-SmallCaps.ttf"),
        family: FontFamily::Name(name.into()),
    }
});
