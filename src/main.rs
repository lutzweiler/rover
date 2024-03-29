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
mod util;

//use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
mod bevy_fly_camera;

use bevy::{app::AppExit, input::keyboard::KeyboardInput, prelude::*};
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// File containing objects to be displayed
    #[clap(parse(try_from_str=util::file_exists))]
    path: String,

    /// Background color in rgb hex format, eg. ffffff for white
    #[clap(short, long, parse(try_from_str=util::str_to_color))]
    background_color: Option<Color>,

    /// Default color for objects that do not contain color data in rgb hex format
    #[clap(short, long, parse(try_from_str=util::str_to_color))]
    default_color: Option<Color>,
}

impl FromWorld for Args {
    fn from_world(world: &mut World) -> Self {
        Args::parse()
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
        .add_system(app_exit)
        .run();
}

fn load_objects(
    args: Res<Args>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let default_color = match args.default_color {
        Some(x) => x,
        None => Color::rgb(0.8, 0.8, 0.8),
    };
    let default_color = Vec3::new(default_color.r(), default_color.g(), default_color.b());
    let builder = builder::MeshBuilder::new(default_color);
    let my_meshes = match builder.parse_file(&args.path) {
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

fn scene_setup(
    args: Res<Args>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(bg_color) = args.background_color {
        commands.insert_resource(ClearColor(bg_color));
    }
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
    commands
        .spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(1.5, 1.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(bevy_fly_camera::lib::FlyCamera::default());
}

fn app_exit(mut exit: EventWriter<AppExit>, input: Res<Input<KeyCode>>) {
    let esc = input.any_pressed([KeyCode::Escape]);
    let q = input.any_pressed([KeyCode::Q]);
    let ctrl = input.any_pressed([KeyCode::LControl, KeyCode::RControl]);

    if esc || ctrl && q {
        exit.send(AppExit);
    }
}
