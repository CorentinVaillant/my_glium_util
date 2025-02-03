use std::{fs::read_to_string, path::Path};

use super::{
    WavefrontError, WavefrontFace, WavefrontGroup, WavefrontLine, WavefrontObj, WavefrontParsable,
};

impl WavefrontParsable for WavefrontObj {
    fn read_from_obj<P: AsRef<Path>>(path: P) -> Result<WavefrontObj, WavefrontError> {
        let data = read_to_string(path).map_err(|e| WavefrontError::IOError(e))?;

        let mut obj = WavefrontObj::empty();

        let mut lines = data.lines();
        let mut current_line = String::new();

        while let Some(line) = lines.next() {
            current_line.push_str(line.trim_end());
            if !line.ends_with('\\') {
                load_line_into_wave_front_obj(&mut obj, &current_line)?;
                current_line.clear();
            } else {
                current_line.pop(); // Remove trailing '\'
            }
        }

        Ok(obj)
    }
}

fn load_line_into_wave_front_obj(obj: &mut WavefrontObj, line: &str) -> Result<(), WavefrontError> {
    let (line, comment) = line.split_once("#").unwrap_or((line, ""));
    if comment.len() > 0 {
        obj.comments.push(comment.to_string());
    }
    match line_type(line) {
        //Vertex data
        WaveFrontLineType::GeoVert => obj
            .geometric_vertices
            .push(parse_array_with_default(line, 0.)),
        WaveFrontLineType::ParamSpaceVert => obj
            .parameter_space_vertices
            .push(parse_array_with_default(line, 1.)),
        WaveFrontLineType::VertNorm => obj.vertex_normals.push(parse_array_with_default(line, 0.)),
        WaveFrontLineType::TextureVert => obj
            .texture_vertices
            .push(parse_array_with_default(line, 0.)),

        //Free-form curve/surface attributes
        //not implemented

        //Elements
        WaveFrontLineType::Point => obj.points.push(parse_vec(line)),
        WaveFrontLineType::Line => obj.line.push(parse_linetype(line)?),
        WaveFrontLineType::Face => obj.face.push(parse_facetype(line)?),
        WaveFrontLineType::Curve | WaveFrontLineType::Curve2 | WaveFrontLineType::Surface => (),

        //Free-form curve/surface body statements
        //not implemented

        //Grouping
        WaveFrontLineType::GroupName => add_group_name(line, obj)?,
        WaveFrontLineType::SmoothGroup | WaveFrontLineType::MergGroupe => (),
        WaveFrontLineType::ObjectName => add_name(line, obj)?,

        _ => (),
    };
    Ok(())
}

use std::str::FromStr;
pub(crate) fn parse_array_with_default<T: Copy + FromStr, const N: usize>(
    line: &str,
    default: T,
) -> [T; N] {
    let mut result = [default; N];
    let mut index = 0;

    for word in line.split_whitespace() {
        if index >= N {
            break;
        }
        if let Ok(val) = word.parse() {
            result[index] = val;
            index += 1;
        }
    }
    result
}

pub(crate) fn parse_vec<T: FromStr>(line: &str) -> Vec<T> {
    let mut result = vec![];
    for word in line.split_whitespace() {
        if let Ok(val) = word.parse() {
            result.push(val);
        }
    }

    result
}

pub(crate) fn parse_linetype(line: &str) -> Result<WavefrontLine, WavefrontError> {
    let mut vertex_indices = Vec::with_capacity(line.len() / 2);
    let mut texture_vertex_indices = Vec::with_capacity(line.len() / 2);

    for word in line.split_whitespace() {
        if word.starts_with("l") {
            continue;
        }
        let mut split = word.split("/");
        let vert_index = split
            .next()
            .ok_or(WavefrontError::InvalidLineData(line.to_string()))?
            .parse()
            .map_err(|_| WavefrontError::InvalidLineData(line.to_string()))?;

        vertex_indices.push(vert_index);

        let text_vert_index = split.next().and_then(|s| s.parse().ok());

        texture_vertex_indices.push(text_vert_index);
    }

    let texture_vertex_indices = texture_vertex_indices.into_iter().collect();

    Ok(WavefrontLine {
        vertex_indices,
        texture_vertex_indices,
    })
}

pub(crate) fn parse_facetype(line: &str) -> Result<WavefrontFace, WavefrontError> {
    let mut vertex_indices = Vec::with_capacity(line.len() / 3);
    let mut texture_vertex_indices = Vec::with_capacity(line.len() / 3);
    let mut normal_vertex_indices = Vec::with_capacity(line.len() / 3);

    for word in line.split_whitespace() {
        if word.starts_with("f") {
            continue;
        }
        let mut split = word.split("/");
        let vert_index = split
            .next()
            .ok_or(WavefrontError::InvalidLineData(line.to_string()))?
            .parse()
            .map_err(|_| WavefrontError::InvalidLineData(line.to_string()))?;

        vertex_indices.push(vert_index);

        let text_vert_index = split.next().and_then(|s| s.parse().ok());

        texture_vertex_indices.push(text_vert_index);

        let norm_vert_index = split.next().and_then(|s| s.parse().ok());

        normal_vertex_indices.push(norm_vert_index);
    }

    let texture_vertex_indices = texture_vertex_indices.into_iter().collect();
    let normal_vertex_indices = normal_vertex_indices.into_iter().collect();
    Ok(WavefrontFace {
        vertex_indices,
        texture_vertex_indices,
        normal_vertex_indices,
    })
}

pub(crate) fn add_group_name(line: &str, obj: &mut WavefrontObj) -> Result<(), WavefrontError> {
    let start_index = obj.groups.last().map(|g| g.end_index + 1).unwrap_or(1);

    let end_index = obj.geometric_vertices.len();

    let split_line = line.split_once(" ");
    if let Some((g, name)) = split_line {
        if g == "g" && name.len() > 0 {
            obj.groups.push(WavefrontGroup {
                name: name.to_string(),
                start_index,
                end_index,
            });

            Ok(())
        } else {
            Err(WavefrontError::InvalidGroupeNameData(line.to_string()))
        }
    } else {
        Err(WavefrontError::InvalidGroupeNameData(line.to_string()))
    }
}

//TODO test
pub(crate) fn add_name(line: &str, obj: &mut WavefrontObj) -> Result<(), WavefrontError> {
    let split_line = line.split_once(" ");
    if let Some((g, name)) = split_line {
        if g == "o" && name.len() > 0 {
            if let Some(prev_name) = &obj.object_name {
                Err(WavefrontError::MultipleNamesDefined(
                    prev_name.clone(),
                    name.to_string(),
                ))
            } else {
                obj.object_name = Some(name.to_string());
                Ok(())
            }
        } else {
            Err(WavefrontError::InvalidNameData(line.to_string()))
        }
    } else {
        Err(WavefrontError::InvalidNameData(line.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WaveFrontLineType {
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

pub(crate) fn line_type(line: &str) -> WaveFrontLineType {
    if line.is_empty() {
        return WaveFrontLineType::Empty;
    }
    if line.starts_with("#") {
        return WaveFrontLineType::Comment;
    }

    match line.split_whitespace().next() {
        Some("v") => WaveFrontLineType::GeoVert,
        Some("vt") => WaveFrontLineType::TextureVert,
        Some("vn") => WaveFrontLineType::VertNorm,
        Some("vp") => WaveFrontLineType::ParamSpaceVert,

        Some("cstype") => WaveFrontLineType::Cstype,
        Some("deg") => WaveFrontLineType::Degree,
        Some("bmat") => WaveFrontLineType::BasisMat,
        Some("step") => WaveFrontLineType::StepSize,

        Some("p") => WaveFrontLineType::Point,
        Some("l") => WaveFrontLineType::Line,
        Some("f") => WaveFrontLineType::Face,
        Some("curv") => WaveFrontLineType::Curve,
        Some("curv2") => WaveFrontLineType::Curve2,
        Some("surf") => WaveFrontLineType::Surface,

        Some("parm") => WaveFrontLineType::ParamValues,
        Some("trim") => WaveFrontLineType::OuterTrimLoop,
        Some("hole") => WaveFrontLineType::InnerTrimLoop,
        Some("scrv") => WaveFrontLineType::SpecialCurv,
        Some("sp") => WaveFrontLineType::SpecialPoint,
        Some("end") => WaveFrontLineType::EndStatement,

        Some("con") => WaveFrontLineType::Connect,

        Some("g") => WaveFrontLineType::GroupName,
        Some("s") => WaveFrontLineType::SmoothGroup,
        Some("mg") => WaveFrontLineType::MergGroupe,
        Some("o") => WaveFrontLineType::ObjectName,

        _ => WaveFrontLineType::Unknown,
    }
}
