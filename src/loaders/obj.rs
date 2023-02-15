use std::{fmt::Error, io};

use super::{LoaderTrait, LoadError};
use glam::{Vec3, Vec3Swizzles};

pub struct OBJLoader {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub colors: Vec<Vec3>,
}

impl OBJLoader {
    pub fn new() -> Self {
        OBJLoader { 
            vertices: vec![], 
            indices: vec![], 
            colors: vec![] 
        }
    }
}

impl LoaderTrait for OBJLoader {
    fn load(&mut self, path: &str) -> Result<(), LoadError> {
        let contents = std::fs::read_to_string(path)?;

        for line in contents.lines() {
            if line.starts_with("#") || line.starts_with("o") {
                continue;
            }

            let parts : Vec<&str> = line.split(" ").collect();

            match parts[0] {
                "v" => {
                    let v = Self::parse_vertex(&parts[1..=3])?;
                    self.vertices.extend(v.to_array());
                },
                "f" => {
                    let face = Self::parse_face(&parts[1..=3])?;
                    self.indices.extend(face);
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_vertex(line: &[&str]) -> Result<Vec3, LoadError>  {
        let x = line[0].parse::<f32>()?;
        let y = line[0].parse::<f32>()?;
        let z = line[0].parse::<f32>()?;

        Ok(Vec3::new(x, y, z))
    }

    fn parse_face(line: &[&str]) -> Result<[u32 ; 3], LoadError> {
        let v1 = line[0].split("/").next().unwrap().parse::<u32>()?;
        let v2 = line[1].split("/").next().unwrap().parse::<u32>()?;
        let v3 = line[2].split("/").next().unwrap().parse::<u32>()?;

        Ok([v1, v2, v3])
    }
}