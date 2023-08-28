use egui::{CollapsingHeader, Context, Window};
use anyhow::{Ok, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Item {
    File { name: String },
    Dir { value: Dir },
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Dir {
    name: String,
    children: Vec<Item>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DirExplorer {
    root: Dir,
}

impl DirExplorer {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        Self::head(ui, &self.root, true);
    }

    fn head(ui: &mut egui::Ui, dir: &Dir, default_open: bool) {
        CollapsingHeader::new(&dir.name)
            .default_open(default_open)
            .show(ui, |ui| Self::children(ui, &dir.children));
    }

    fn children(ui: &mut egui::Ui, items: &Vec<Item>) {
        for item in items {
            match item {
                Item::File { name } => {
                    ui.label(name);
                }
                Item::Dir { value } => Self::head(ui, value, false),
            }
        }
    }
}

#[allow(dead_code)] // it's used for demo currently
pub fn demo(ctx: &Context) {
    let items = vec![
        Item::File {
            name: "lil_1".to_string(),
        },
        Item::File {
            name: "lil_2".to_string(),
        },
        Item::File {
            name: "lil_3".to_string(),
        },
        Item::Dir {
            value: Dir {
                name: "kek_1".to_string(),
                children: vec![
                    Item::Dir {
                        value: Dir {
                            name: "kek_2".to_string(),
                            children: vec![
                                Item::File {
                                    name: "lil_1".to_string(),
                                },
                                Item::File {
                                    name: "lil_2".to_string(),
                                },
                            ],
                        },
                    },
                    Item::Dir {
                        value: Dir {
                            name: "kek_3".to_string(),
                            children: Vec::with_capacity(0),
                        },
                    },
                    Item::File {
                        name: "lil_1".to_string(),
                    },
                ],
            },
        },
    ];

    let mut explorer = DirExplorer {
        root: Dir {
            name: "root_kek".to_string(),
            children: items,
        },
    };

    Window::new("explorer_demo")
        .open(&mut true)
        .vscroll(true)
        .hscroll(true)
        .show(ctx, |ui| explorer.ui(ui));
}
