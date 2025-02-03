use std::{fs::read_to_string, path::Path};

use super::{WavefrontError, WavefrontFace, WavefrontGroup, WavefrontLine, WavefrontObj, WavefrontParsable};

impl WavefrontParsable for WavefrontObj {
    fn read_from_obj<P : AsRef<Path> >(path: P) -> Result<WavefrontObj, WavefrontError> {
        let data = read_to_string(path)
        .map_err(|e|{WavefrontError::IOError(e)})?;

        let mut obj = WavefrontObj::empty();

        let mut lines = data.lines();
        let mut current_line = String::new();

        while let Some(line) = lines.next(){
            //if there is any "\" at the end of the line, read the next line with it
            if let Some(backslash_pos) = line.find("\\"){
                current_line.push_str(&line[..backslash_pos].trim_end());
            }else{
                current_line.push_str(line);
                load_line_into_wave_front_obj(&mut obj, &current_line)?;
                current_line.clear();
            }
        }

        Ok(obj)
    }
}

fn load_line_into_wave_front_obj(obj:&mut WavefrontObj, line:&String)->Result<(),WavefrontError>{
    let (line, comment) = line.split_once("#").unwrap_or((line,""));
    if comment.len() > 0{
        obj.comments.push(comment.to_string());
    }
    match line_type(line) {
//Vertex data
        WaveFrontLineType::GeoVert       => obj.geometric_vertices.push(parse_array_with_default(line, 0.)),
        WaveFrontLineType::ParamSpaceVert=>obj.parameter_space_vertices.push(parse_array_with_default(line, 1.)),
        WaveFrontLineType::VertNorm      =>obj.vertex_normals.push(parse_array_with_default(line, 0.)),
        WaveFrontLineType::TextureVert   =>obj.texture_vertices.push(parse_array_with_default(line, 0.)),


//Free-form curve/surface attributes
    //not implemented

//Elements
        WaveFrontLineType::Point => obj.points.push(parse_vec(line)),
        WaveFrontLineType::Line  => obj.line.push(parse_linetype(line)?),
        WaveFrontLineType::Face  => obj.face.push(parse_facetype(line)?), 
        WaveFrontLineType::Curve |
        WaveFrontLineType::Curve2|
        WaveFrontLineType::Surface => (),

//Free-form curve/surface body statements
    //not implemented

//Grouping
        WaveFrontLineType::GroupName  => add_group_name(line,obj)?,
        WaveFrontLineType::SmoothGroup |
        WaveFrontLineType::MergGroupe => (),
        WaveFrontLineType::ObjectName => add_name(line, obj)?,

        _ => ()
    };
    Ok(())
}

use std::str::FromStr;
//TODO test
fn parse_array_with_default<T: Copy + FromStr, const N: usize>(line: &str,default: T,) -> [T; N] {
    let mut result = [default; N];

    for (i, word) in line.split_whitespace().take(N).enumerate() {
        if let Ok(val) = word.parse() {
            result[i] = val;
        }
    }

    result
}

//TODO test
fn parse_vec<T :FromStr >(line :&str)->Vec<T>{
    let mut result = vec![];
    for word in line.split_whitespace(){
        if let Ok(val) = word.parse(){
            result.push(val);
        }
    }

    result
}

//TODO test
fn parse_linetype(line :&str) -> Result<WavefrontLine,WavefrontError>{
    let mut vertex_indices =Vec::with_capacity(line.len() / 2);
    let mut texture_vertex_indices = Vec::with_capacity(line.len() / 2);

    for word in line.split_whitespace(){
        if word[0..1].contains("l"){
            continue;
        }
        if word[0..1].contains("#"){
            break;
        }
        let mut split = word.split("/");
        let vert_index = split.next()
          .ok_or(WavefrontError::InvalidLineData(line.to_string()))?
          .parse().map_err(|_|WavefrontError::InvalidLineData(line.to_string()))?;

        vertex_indices.push(vert_index);

        let text_vert_index = split.next()
          .and_then(|s| s.parse().ok());


        texture_vertex_indices.push(text_vert_index);
    }


    let texture_vertex_indices = texture_vertex_indices.into_iter().collect();

    Ok(WavefrontLine { vertex_indices , texture_vertex_indices })

}

//TODO test
fn parse_facetype(line :&str) -> Result<WavefrontFace,WavefrontError>{
    let mut vertex_indices = Vec::with_capacity(line.len() / 3);
    let mut texture_vertex_indices = Vec::with_capacity(line.len() / 3);
    let mut normal_vertex_indices = Vec::with_capacity(line.len() / 3);

    for word in line.split_whitespace(){
        if word[0..1].contains("f"){
            continue;
        }
        if word[0..1].contains("#"){
            break;
        }
        let mut split = word.split("/");
        let vert_index = split.next()
        .ok_or(WavefrontError::InvalidLineData(line.to_string()))?
        .parse().map_err(|_|WavefrontError::InvalidLineData(line.to_string()))?;

        vertex_indices.push(vert_index);

        let text_vert_index = split.next()
        .and_then(|s| s.parse().ok());

        texture_vertex_indices.push(text_vert_index);

        let norm_vert_index = split.next()
        .and_then(|s| s.parse().ok());

        normal_vertex_indices.push(norm_vert_index);
    }

    let texture_vertex_indices = texture_vertex_indices.into_iter().collect();
    let normal_vertex_indices = normal_vertex_indices.into_iter().collect();
    Ok(WavefrontFace{vertex_indices,texture_vertex_indices,normal_vertex_indices})
}


//TODO test 
fn add_group_name(line:&str, obj:&mut WavefrontObj)->Result<(),WavefrontError>{
    let start_index = obj.groups.last()
    .map(|g|g.end_index).unwrap_or(0);

    let end_index = obj.geometric_vertices.len();

    let split_line = line.split_once(" ");
    if let Some((g,name)) =  split_line{
        if g == "g" && name.len() > 0{
            obj.groups.push(WavefrontGroup{
                name : name.to_string(),
                start_index,
                end_index
            });
            
            Ok(())
        }else {
            Err(WavefrontError::InvalidGroupeNameData(line.to_string()))
        }
    }else {
        Err(WavefrontError::InvalidGroupeNameData(line.to_string()))
    }

}

//TODO test
fn add_name(line :&str, obj:&mut WavefrontObj)->Result<(),WavefrontError>{
    let split_line = line.split_once(" ");
    if let Some((g,name)) =  split_line{
        if g == "o" && name.len() > 0{
            
            if let Some(prev_name) = &obj.object_name {
                Err(WavefrontError::MultipleNamesDefined(prev_name.clone(), name.to_string()))
            }else{
                obj.object_name = Some(name.to_string());
                Ok(())
            }
        }else {
            Err(WavefrontError::InvalidNameData(line.to_string()))
        }
    }else {
        Err(WavefrontError::InvalidNameData(line.to_string()))
    } 


}

#[derive(Debug, Clone, Copy)]
enum WaveFrontLineType {
    GeoVert,        //v
    TextureVert,    //vt
    VertNorm,       //vn
    ParamSpaceVert, //vp

    Cstype,   //cstype
    Degree,   //deg
    BasisMat, //bmat
    StepSize, //step

    Point,   //p
    Line,    //l
    Face,    //f
    Curve,   //curv
    Curve2,  //curv2
    Surface, //surf

    ParamValues,   //parm
    OuterTrimLoop, //trim
    InnerTrimLoop, //hole
    SpecialCurv,   //scrv
    SpecialPoint,  //sp
    EndStatement,  //end

    Connect, //con

    GroupName,   //g
    SmoothGroup, //s
    MergGroupe,  //mg
    ObjectName,  //o

    Empty,
    Unknown,
    Comment,
}

//TODO test
fn line_type(line :&str)->WaveFrontLineType{
    if line.len()<2{
        WaveFrontLineType::Empty
    }else {
        match &line[0..2] {
            "v " => WaveFrontLineType::GeoVert,
            "vt" => WaveFrontLineType::TextureVert,
            "vn" => WaveFrontLineType::VertNorm,
            "vp" => WaveFrontLineType::ParamSpaceVert,

            "cs" => WaveFrontLineType::Cstype,
            "de" => WaveFrontLineType::Degree,
            "bm" => WaveFrontLineType::BasisMat,
            "st" => WaveFrontLineType::StepSize,

            "p " => WaveFrontLineType::Point,
            "l " => WaveFrontLineType::Line,
            "f " => WaveFrontLineType::Face,
            "cu" => if line.len() < 5{
                    WaveFrontLineType::Unknown
                }else {
                    match &line[0..6] {
                        "curve "=> WaveFrontLineType::Curve,
                        "curve2"=> WaveFrontLineType::Curve2,
                        _ => WaveFrontLineType::Unknown,
                    }
                }
            "su" => WaveFrontLineType::Surface,

            "pa" => WaveFrontLineType::ParamValues,
            "tr" => WaveFrontLineType::OuterTrimLoop,
            "ho" => WaveFrontLineType::InnerTrimLoop,
            "sc" => WaveFrontLineType::SpecialCurv,
            "sp" => WaveFrontLineType::SpecialPoint,
            "en" => WaveFrontLineType::EndStatement,

            "co" => WaveFrontLineType::Connect,

            "g " => WaveFrontLineType::GroupName,
            "s " => WaveFrontLineType::SmoothGroup,
            "mg" => WaveFrontLineType::MergGroupe,
            "o " => WaveFrontLineType::ObjectName,

            s => match &s[0..1] {
                "#"=> WaveFrontLineType::Comment,
                _  => WaveFrontLineType::Unknown,
            }
        }
    }
}