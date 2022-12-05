use bevy::{
    asset::HandleId,
    ecs::{
        bundle::Bundles,
        system::{EntityCommands, InsertResource},
    },
    prelude::*,
    utils::{tracing::Instrument, Uuid},
};
extern crate rand;

use bevy_flycam::{MovementSettings, PlayerPlugin};
use rand::Rng;

#[derive(Debug, Clone, Copy, Component)]
struct Vox {
    id: (i32, i32, i32),
}
#[derive(Debug, Clone, Copy)]

struct World([[[Vox; 10]; 10]; 10]);
// fn crate_ent() {
//     let commands = &mut Commands::spawn_bundle(self);
//     // mut meshes= ResMut<Assets<Mesh>>,
//     // mut materials: ResMut<Assets<StandardMaterial>>
//     Commands::spawn_bundle(0).id()
// }
impl World {
    fn new(default_vox_value: Vox) -> Self {
        Self([[[default_vox_value; 10]; 10]; 10])
    }
    fn set_vox(&mut self, pos_xyz: (usize, usize, usize), val: Vox) {
        self.0[pos_xyz.0][pos_xyz.1][pos_xyz.2] = val;
    }
    fn get_vox(&self, pos_xyz: (usize, usize, usize)) -> &Vox {
        return &self.0[pos_xyz.0][pos_xyz.1][pos_xyz.2];
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust WFC!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .insert_resource(World([[[Vox { id: (0, 0, 0) }; 10]; 10]; 10]))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup)
        // .add_startup_system(add_people)
        .add_system(logic)
        // .add_system(greet_people)
        .run();
}
// fn updater(query: Query) {
//     //query.instrument(span)
// }

fn logic(world: ResMut<World>, mut q: Query<&mut Transform, With<Vox>>) {
    // world.resource.get_vox(pos_xyz).id
    let vox_pos = world.get_vox((1, 0, 0)).id;
    q.iter_mut().last().unwrap().scale.x += -0.1;
    println!("{:#?}", vox_pos);
    //let incrment = obj.transform.scale[1];
    // obj.transform.scale[1] += 0.1;
    //meshes.get_mut(&mesh_handel);
}
//logic
//8ecbac0ff5454473ad43e1f4243af51e,
// 6416496396338074875
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut world: ResMut<World>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 20.0;
    // cube
    {
        let mut rng = rand::thread_rng();
        for d1 in 0..10 {
            for d2 in 0..10 {
                for d3 in 0..10 {
                    //let a = world.get_vox((d3, d2, d1));
                    let t1: u16 = d1;
                    let t2: u16 = d2;
                    let t3: u16 = d3;
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.9 })),
                            material: materials.add(
                                Color::rgb(
                                    rng.gen_range(0.3..1.0).into(),
                                    rng.gen_range(0.0..1.0).into(),
                                    rng.gen_range(0.0..1.0).into(),
                                )
                                .into(),
                            ),
                            transform: Transform::from_xyz(t3.into(), t2.into(), t1.into()),
                            ..default()
                        })
                        .insert(Vox {
                            id: ((t3.into(), t2.into(), t1.into())),
                        })
                        .insert(Name::new("VOX"));

                    world.set_vox(
                        (t3.into(), t2.into(), t1.into()),
                        Vox {
                            id: (t3.into(), t2.into(), t1.into()),
                        },
                    )
                }
            }
        }
        // light
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 150000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
    }

    // camera
    {
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-7.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }
}
