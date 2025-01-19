//TODO obj parse !

use std::{fs::{read_to_string, File}, io::Read, path::Path};

use super::mesh::Mesh;

impl Mesh {
    pub fn load_from_wavefront<P: AsRef<Path>>(path : P)->Result<Self,WavefrontError>{
        for line in read_to_string(path){
            
        }

        todo!()
    }
}



pub enum WavefrontError {
    CouldNotReadFile(File)
}


fn line_type(line:String)->WaveFrontLineType{
    let mut split = line.split(" ");
    let mut str;
    match split.next() {
        Some(s)=>{str = s;},

        None=> {return WaveFrontLineType::Empty;}
    }
//TODO

    todo!()
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

    Empty,
    Other,
}