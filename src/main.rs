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
mod math;
mod triangle;

//use bevy::math::f64::DVec3 as Vec3;
use bezier::rectangle::BezierRectangle;
use bezier::rectangle::FromString;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        camera::{Camera3d, CameraPlugin}
    },
    pbr::wireframe::{WireframeConfig, WireframePlugin},

};
use bevy_flycam::{PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn example_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let up = Vec3::Z;
    let indices = vec![0,1,2];
    let positions = vec![[0.,0.,0.], [0.,1.,0.], [1.,0.,0.]];
    let normals = vec![[0.,0.,1.], [0.,0.,1.], [0.,0.,1.]];
    let colors = vec![[1.,0.,0.,1.], [0.,1.,0.,1.], [0.,0.,1.,1.]];
    let uvs = vec![[0.0, 0.0],[0.0, 0.0],[0.0, 0.0]];

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    return mesh;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = example_mesh();
    let mut triangle_material = StandardMaterial::default();
    triangle_material.cull_mode = None;
    triangle_material.base_color = Color::WHITE; //lets 100% of vertex colors through
    triangle_material.double_sided = false; //for lighting on backside not sure which is right
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(triangle_material),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, -1.5),
        ..default()
    });
    
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });

}

#[cfg(test)]
mod tests {}
