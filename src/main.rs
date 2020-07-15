use legion::prelude::*;
use type_uuid::TypeUuid;
use serde::{Deserialize, Serialize};
mod serialize;
use serialize::{ComponentRegistration, TagRegistration};

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "3e878aaa-b147-4d6f-8a03-ce0acdb26191"]
struct IdentityTag(usize);

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "d21ed260-2438-417e-8701-6fb276c4ba09"]
struct TreeTag{}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "d3c0b587-eb8c-430b-af89-db5ccf094940"]
struct Position{idx: usize}

fn component_registration() -> (Vec<ComponentRegistration>, Vec<TagRegistration>) {
    let comp_registrations = vec![
        ComponentRegistration::of::<Position>(),
    ];
    let tag_registrations = vec![
        TagRegistration::of::<TreeTag>(),
        TagRegistration::of::<IdentityTag>(),
    ];
    (comp_registrations, tag_registrations)
}

fn insert_fake_trees(ecs: &mut World) {
    println!("Insert a bunch of fake trees, similar to those in my game");
    for i in 0..3 {
        ecs.insert(
            (TreeTag {}, IdentityTag(i)),
            vec![(
                Position{ idx: i * 2},
            )],
        );
    }
}

fn print_tree(ecs: &World, id: usize) {
    println!("Searching for tree {}", id);
    let tree_id = IdentityTag(id);
    <Read<Position>>::query()
        .filter(tag_value(&tree_id))
        .iter_entities(ecs)
        .for_each(|(entity, pos)| {
            println!("Found the entity: {:?}, world position: {:?}", entity, pos);
        }
    );
}

fn works_fine() {
    let universe = Universe::new();
    let mut ecs = universe.create_world();

    insert_fake_trees(&mut ecs);
    print_tree(&ecs, 1);

    println!("We want to kill tree #1");
    let tree_id = IdentityTag(1);
    let mut commands = CommandBuffer::new(&ecs);
    <Read<Position>>::query()
        .filter(tag_value(&tree_id))
        .iter_entities(&ecs)
        .for_each(|(entity, pos)| {
            println!("Found the entity to delete: {:?}, world position: {:?}", entity, pos);
            commands.delete(entity);
        }
    );

    println!("Running the delete buffer");
    commands.write(&mut ecs);

    // Check that it's really gone
    print_tree(&ecs, 1);
    println!("You shouldn't have seen a tree there!");
}

fn crashes() {
    let universe = Universe::new();
    let mut ecs = universe.create_world();

    insert_fake_trees(&mut ecs);
    print_tree(&ecs, 1);

    println!("Save the world and load it again");
    let serialized = serialize::serialize_world(&ecs);
    let mut ecs = serialize::deserialize_world(serialized, &universe);

    print_tree(&ecs, 1);

    println!("We want to kill tree #1");
    let tree_id = IdentityTag(1);
    let mut commands = CommandBuffer::new(&ecs);
    <Read<Position>>::query()
        .filter(tag_value(&tree_id))
        .iter_entities(&ecs)
        .for_each(|(entity, pos)| {
            println!("Found the entity to delete: {:?}, world position: {:?}", entity, pos);
            commands.delete(entity);
        }
    );

    println!("Running the delete buffer");
    commands.write(&mut ecs); // <--- CRASHES HERE

    // Check that it's really gone
    print_tree(&ecs, 1);
    println!("You shouldn't have seen a tree there!");
}

fn main() {
    println!("RUN 1: WORKS FINE");
    println!("-----------------");
    works_fine();
    println!("\n\nRUN 2: CRASHES");
    println!("------------------");
    crashes();
}
