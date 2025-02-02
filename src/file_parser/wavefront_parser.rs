use super::{WavefrontError, WavefrontObj, WavefrontParsable};

impl WavefrontParsable for WavefrontObj {
    fn read_from_obj(path: &std::path::Path) -> Result<WavefrontObj, WavefrontError> {
        todo!()
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
            "mg" => WaveFrontLineType::SmoothGroup,
            "o " => WaveFrontLineType::ObjectName,

            s => match &s[0..1] {
                "#"=> WaveFrontLineType::Comment,
                _  => WaveFrontLineType::Unknown,
            }
        }
    }
}