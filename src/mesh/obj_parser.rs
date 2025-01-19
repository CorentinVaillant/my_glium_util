//TODO obj parse !

use std::{fs::read_to_string, path::Path};

use super::mesh::Mesh;

impl Mesh {
    pub fn load_from_wavefront<P: AsRef<Path>>(path : P)->Result<Self,WavefrontError>{

        let vertecies = vec![];

        for line in read_to_string(path).map_err(|_|{WavefrontError::CouldNotReadFile})?.lines(){
            match line_type(line) {
                WaveFrontLineType::GeoVertex => todo!(),
                WaveFrontLineType::TextureVertex => todo!(),
                WaveFrontLineType::VertexNormal => todo!(),
                WaveFrontLineType::ParameterSpaceVert => todo!(),
                WaveFrontLineType::Point => todo!(),
                WaveFrontLineType::Line => todo!(),
                WaveFrontLineType::Face => todo!(),
                WaveFrontLineType::Comment => todo!(),
                WaveFrontLineType::Empty => todo!(),
                WaveFrontLineType::Unknow => todo!(),
            }
        }

        Ok(Mesh::from(vertecies))
    }
}


pub enum WavefrontError {
    CouldNotReadFile
}


fn line_type(line:&str)->WaveFrontLineType{
    let mut split = line.split(" ");
    match split.next() {
        None=> WaveFrontLineType::Empty,
        Some(s)=>{
            let mut chars = s.chars();
            match chars.next() {
                Some('v')=> match chars.next() {
                    Some('t')=> WaveFrontLineType::TextureVertex,
                    Some('n')=> WaveFrontLineType::VertexNormal,
                    Some('p')=>WaveFrontLineType::ParameterSpaceVert,
                    Some(_)=> WaveFrontLineType::GeoVertex,
                    None=> WaveFrontLineType::Unknow,
                }
                Some('p')=> WaveFrontLineType::Point,
                Some('l')=>WaveFrontLineType::Line,
                Some('f')=>WaveFrontLineType::Face,
                Some('#')=>WaveFrontLineType::Comment,

                _=>WaveFrontLineType::Unknow
            }
        },

    }

}

enum WaveFrontLineType {

    //Vertex data
    GeoVertex,          //v
    TextureVertex,      //vt
    VertexNormal,       //vn
    ParameterSpaceVert, //vp

    //Element
    Point,              //p
    Line,               //l
    Face,               //f

    Comment,
    Empty,
    Unknow,
}


//if a parse failed, replace it with zero
fn parse_tree_float(line :&str)->[f32;3]{
    
    let mut result = [0.;3];
    let mut indice = 0;
    for split in line.split(" "){
        if indice > 3 {
            return result;
        }
        match split.parse() {
            Ok(val) => {result[indice] = val;},
            _=> ()
        };
    }

    result
}