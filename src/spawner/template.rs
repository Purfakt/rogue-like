use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/templates.ron").expect("Failed to open template file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|template| template.levels.contains(&level))
            .for_each(|template| {
                for _ in 0..template.frequency {
                    available_entities.push(template);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|point| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(point, entity, &mut commands);
            }
        });
        commands.flush(ecs, &mut Resources::default());
    }

    pub fn spawn_entity(&self, point: &Point, template: &Template, commands: &mut CommandBuffer) {
        println!("{:?}", template);

        let entity = commands.push((
            *point,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
            EntityType::Item => commands.add_component(entity, Item),
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(provides, n)| match provides.as_str() {
                "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                "MagicMap" => commands.add_component(entity, ProvidesDungeonMap),
                _ => {
                    println!("Warning: we don't know how to provide {}", provides);
                }
            });
        }
    }
}
