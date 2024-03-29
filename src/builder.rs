use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::bezier::rectangle::{BezierRectangle, FromString};
use crate::subdivision::SubdivisionSet;
use crate::triangle::{ToTriangle, Triangle};
use bevy::prelude::{Mesh, Vec3};
use Vec3 as Color;

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
    Off,
    Rect11,
    Rect22,
    Rect33,
    Rect44,
}

fn offtype_id(offtype: &OffType) -> usize {
    match offtype {
        OffType::None => 0,
        OffType::Off => 1,
        OffType::Rect11 => 2,
        OffType::Rect22 => 3,
        OffType::Rect33 => 4,
        OffType::Rect44 => 5,
    }
}

fn line_length(offtype: OffType) -> usize {
    match offtype {
        OffType::None => 0,
        OffType::Off => 1,
        OffType::Rect11 => 2 * 2 + 4,
        OffType::Rect22 => 3 * 3 + 4,
        OffType::Rect33 => 4 * 4 + 4,
        OffType::Rect44 => 5 * 5 + 4,
    }
}

fn match_line_type(line: &String) -> LineType {
    match line.as_str() {
        "OFF" | "COFF" => LineType::Header(OffType::Off),
        "CBEZ113" => LineType::Header(OffType::Rect11),
        "CBEZ223" => LineType::Header(OffType::Rect22),
        "CBEZ333" => LineType::Header(OffType::Rect33),
        "CBEZ443" => LineType::Header(OffType::Rect44),
        _ => LineType::Values,
    }
}

pub struct MeshBuilder {
    strings: [String; 6],
    objects: (
        (),
        Vec<Triangle<Vec3>>,
        Vec<BezierRectangle<Vec3, 1, 1>>,
        Vec<BezierRectangle<Vec3, 2, 2>>,
        Vec<BezierRectangle<Vec3, 3, 3>>,
        Vec<BezierRectangle<Vec3, 4, 4>>,
    ),
    default_color: Color
}

impl MeshBuilder {
    pub fn new(default_color: Color) -> Self {
        MeshBuilder {
            strings: [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()],
            objects: (
                (),
                Vec::<Triangle<Vec3>>::new(),
                Vec::<BezierRectangle<Vec3, 1, 1>>::new(),
                Vec::<BezierRectangle<Vec3, 2, 2>>::new(),
                Vec::<BezierRectangle<Vec3, 3, 3>>::new(),
                Vec::<BezierRectangle<Vec3, 4, 4>>::new(),
            ),
            default_color: default_color,
        }
    }

    fn build_objects(&mut self) {
        {&mut self.build_triangles()};
        {&mut self.build_rect11()};
        {&mut self.build_rect22()};
        {&mut self.build_rect33()};
        {&mut self.build_rect44()};
    }

    fn build_rect11(&mut self) {
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[2].lines() {
            i += 1;
            input.push_str(&line);
            input.push_str("\n");
            if i == line_length(OffType::Rect11) {
                if let Ok(surf) = BezierRectangle::<Vec3, 1, 1>::from_string(&input) {
                    self.objects.2.push(surf);
                }
                input.clear();
                i = 0;
            }
        }
    }

    fn build_rect22(&mut self) {
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[3].lines() {
            i += 1;
            input.push_str(&line);
            input.push_str("\n");
            if i == line_length(OffType::Rect22) {
                if let Ok(surf) = BezierRectangle::<Vec3, 2, 2>::from_string(&input) {
                    self.objects.3.push(surf);
                }
                input.clear();
                i = 0;
            }
        }
    }
    fn build_rect33(&mut self) {
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[4].lines() {
            i += 1;
            input.push_str(&line);
            input.push_str("\n");
            if i == line_length(OffType::Rect33) {
                if let Ok(surf) = BezierRectangle::<Vec3, 3, 3>::from_string(&input) {
                    self.objects.4.push(surf);
                }
                input.clear();
                i = 0;
            }
        }
    }

    fn build_rect44(&mut self) {
        let mut input = String::new();
        let mut i = 0;
        for line in self.strings[5].lines() {
            i += 1;
            input.push_str(&line);
            if i == line_length(OffType::Rect44) {
                if let Ok(surf) = BezierRectangle::<Vec3, 4, 4>::from_string(&input) {
                    self.objects.5.push(surf);
                }
                input.clear();
                i = 0;
            }
        }
    }

    fn build_triangles(&mut self) {
        if self.strings[1].is_empty() {
            return;
        }
        let mut line_iter = self.strings[1].lines();
        let first_line = line_iter.next().unwrap();
        let num_vertices = first_line.split_whitespace().next().unwrap().parse::<usize>().unwrap();
        let lines: Vec<&str> = line_iter.collect();
        'line: for line in &lines[num_vertices..] {
            let mut line_iter = line.split_whitespace();
            if line_iter.next().unwrap() != "3" {
                continue 'line;
            }
            let positions: Vec<String> = line_iter
                .map(|s| s.parse::<usize>().unwrap())
                .map(|i| lines[i])
                .map(|s| s.to_string())
                .collect();
            if positions.len() == 3 {
                let triangle = Triangle::from_string([&positions[0], &positions[1], &positions[2]], self.default_color);
                match triangle {
                    Ok(t) => {
                        self.objects.1.push(t);
                    }
                    _ => (),
                }
            }
        }

    }

    fn build_meshes(self) -> Vec<Mesh> {
        let mut meshes = Vec::<Mesh>::new();
        //None
        //do nothing

        //Triangle
        meshes.push(Triangle::triangle_list_to_mesh(self.objects.1));

        //Rect11
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.2;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));

        //Rect22
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.3;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));
        //Rect33
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.4;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));

        //Rect44
        let mut subdiv = SubdivisionSet::new();
        subdiv.elements = self.objects.5;
        subdiv.subdivide();
        meshes.push(Triangle::triangle_list_to_mesh(subdiv.to_triangles()));

        meshes
    }

    pub fn parse_file<P>(mut self, path: P) -> Result<Vec<Mesh>, String>
    where
        P: AsRef<Path>,
    {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };

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
                    self.strings[offtype_id(&current_type)].push_str(&line);
                    self.strings[offtype_id(&current_type)].push_str("\n");
                }
                LineType::Header(offtype) => {
                    current_type = offtype;
                }
            }
        }

        self.build_objects();
        let meshes = self.build_meshes();
        Ok(meshes)
    }
}