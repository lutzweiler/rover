use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::bezier::rectangle::{BezierRectangle, FromString};
use crate::subdivision::SubdivisionSet;
use crate::triangle::{ToTriangle, Triangle};
use bevy::prelude::{Mesh, Vec3};

/*
    for now, building and parsing is tedious. Because loaded objects have their own types,
    we cannot create an array that contains all of them.
    using Vec<dyn Subdivide> seems like a possibility, but is not immediately possible.
    This leads to some unwanted code duplication in build_objects and build_meshes
*/

#[derive(Debug, Clone, Copy)]
enum LineType {
    Header(OffType),
    Values,
}

#[derive(Debug, Clone, Copy)]
enum OffType {
    None,
    Rect33,
    Rect44,
}

fn offtype_id(offtype: &OffType) -> usize {
    match offtype {
        OffType::None => 0,
        OffType::Rect33 => 1,
        OffType::Rect44 => 2,
    }
}

fn line_length(offtype: OffType) -> usize {
    match offtype {
        OffType::None => 0,
        OffType::Rect33 => 4 * 4 + 4,
        OffType::Rect44 => 5 * 5 + 4,
    }
}

fn match_line_type(line: &String) -> LineType {
    match line.as_str() {
        "CBEZ333" => LineType::Header(OffType::Rect33),
        "CBEZ443" => LineType::Header(OffType::Rect44),
        _ => LineType::Values,
    }
}

struct MeshBuilder {
    strings: [String; 3],
    objects: ((), Vec<BezierRectangle<Vec3, 3, 3>>, Vec<BezierRectangle<Vec3, 4, 4>>),
}

impl MeshBuilder {
    fn new() -> Self {
        MeshBuilder {
            strings: [String::new(), String::new(), String::new()],
            objects: (
                (),
                Vec::<BezierRectangle<Vec3, 3, 3>>::new(),
                Vec::<BezierRectangle<Vec3, 4, 4>>::new(),
            ),
        }
    }

    fn build_objects(&mut self) {
        //None
        //do nothing

        //Rect33
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[1].lines() {
            i += 1;
            input.push_str(&line);
            input.push_str("\n");
            if i == line_length(OffType::Rect33) {
                if let Ok(surf) = BezierRectangle::<Vec3, 3, 3>::from_string(&input) {
                    self.objects.1.push(surf);
                }
                input.clear();
                i = 0;
            }
        }

        //Rect44
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[2].lines() {
            i += 1;
            input.push_str(&line);
            if i == line_length(OffType::Rect44) {
                if let Ok(surf) = BezierRectangle::<Vec3, 4, 4>::from_string(&input) {
                    self.objects.2.push(surf);
                }
                input.clear();
                i = 0;
            }
        }
    }

    fn build_meshes(self) -> Vec<Mesh> {
        let mut meshes = Vec::<Mesh>::new();
        //None
        //do nothing

        //Rect33
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.1;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));

        //Rect44
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.2;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));

        meshes
    }
}

pub fn parse_file<P>(path: P) -> Result<Vec<Mesh>, String>
where
    P: AsRef<Path>,
{
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let mut mesh_builder = MeshBuilder::new();
    let mut current_type = OffType::None;
    for line in io::BufReader::new(file).lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => continue,
        };
        if line.is_empty() {
            continue;
        }
        match match_line_type(&line) {
            LineType::Values => {
                mesh_builder.strings[offtype_id(&current_type)].push_str(&line);
                mesh_builder.strings[offtype_id(&current_type)].push_str("\n");
            }
            LineType::Header(offtype) => {
                current_type = offtype;
            }
        }
    }

    mesh_builder.build_objects();
    let meshes = mesh_builder.build_meshes();
    Ok(meshes)
}
