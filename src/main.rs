#![feature(generic_const_exprs)]
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    this project uses const generics to create indiviudal structs for any degree of bezier curve/surface.
    since bezier curves use one more control point than their degree, and surfaces need a product / triangular number of ctrl points,
    it is necessary to compute the array sizes at compile time from the generic parameter.
    evaluating generic const expressions is a feature only available in the nightly version of rust and needs to be enabled manually.
    as such it is possible that some syntax needs to be changed in the future in order for this to work correctly.
    further, some trait bounds need to be explicitly stated:
    for a bezier curve of degree N, an array of size N+1 is needed. in the (extremely unrealistic case) of N=usize::max,
    such an array can not be allocated, therefore the trait bound
        [(); N+1]:
    is needed.
    note that the compiler cannot infer the degree of a bezier object only from an array size. it can however infer the type of control point.
    initialize a bezier curve of eg degree 2 like this:
        let b = BezierCurve::<_, 2>::new([1,2,3]);
*/

/*
    At some points there are very ugly calculations with many hard-to-read indices
    One improvement would be to create a Triangular Array struct that makes access
    by (i,j,k) indices easy and can return arrays containing the values at each edge.
    This could also make code more performant, it is not always clear where memory is or could be
    referenced, cloned or copied
*/

//include all submodules so tests run
mod bezier;
mod builder;
mod math;
mod subdivision;
mod triangle;

//use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
mod bevy_fly_camera;

use std::path::Path;
use bevy::prelude::*;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(parse(try_from_str=file_exists))]
    path: String,
}

impl FromWorld for Args {
    fn from_world(world: &mut World) -> Self {
        Args::parse()
    }
}

fn file_exists(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File not found".to_string())
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<Args>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(scene_setup)
        .add_startup_system(load_objects)
        .add_plugin(bevy_fly_camera::lib::FlyCameraPlugin)
        .run();
}

fn load_objects(
    args: Res<Args>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let my_meshes = match builder::parse_file(&args.path) {
        Ok(m) => m,
        Err(e) => {
            println!("An error occured while parsing the input file");
            panic!();
        }
    };
    for mesh in my_meshes {
        let mut triangle_material = StandardMaterial::default();
        triangle_material.metallic = 0.;
        triangle_material.reflectance = 0.0;
        triangle_material.cull_mode = None;
        triangle_material.base_color = Color::WHITE; //lets 100% of vertex colors through
        triangle_material.double_sided = false; //for lighting on backside not sure which is right

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(triangle_material),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    }
}

fn scene_setup(mut commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(1.5, 1.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(bevy_fly_camera::lib::FlyCamera::default());
}
