
pub type NotImpl =();

/*------------------*\
|     Wavefront      |
\*------------------*/

#[derive(Debug,Clone)]
pub struct WavefrontObj{
    
//vertex data
    pub(crate) geometric_vertices :      [f32;4],
    pub(crate) texture_verices :         [f32;3],
    pub(crate) vertex_normals :          [f32;3],
    pub(crate) parameter_space_vertices :[f32;3],

//Free-form curve/surface attributes
    //not implemented

//Elements
    pub(crate)point: Vec<u32>,
    pub(crate)line : Vec<WavefrontLine>,
    pub(crate)face : Vec<WavefrontFace>,

    pub(crate)curv :  NotImpl,  //not implemented
    pub(crate)curv2:  NotImpl,  //not implemented
    pub(crate)surface:NotImpl,  //not implemented

//Free-form curve/surface body statements
    //not implemented

//Grouping
    pub(crate)groupe_name:    Vec<WavefrontGroup>,
    pub(crate)smoothing_group:NotImpl,
    pub(crate)merging_group:  NotImpl,
    pub(crate)object_name:    String,

//Display/render attributes
    //not implemented

//Comments
    pub(crate)comments :Vec<String>
}
#[derive(Debug,Clone, Copy)]
pub(crate) struct WavefrontLine{
    pub(crate)vertex_index : u32,
    pub(crate)texture_vertex_index : Option<u32>
}

#[derive(Debug,Clone, Copy)]
pub(crate) struct WavefrontFace{
    pub(crate)vertex_index :u32,
    pub(crate)texture_vertex_index : Option<u32>,
    pub(crate)normal_vertex_index : Option<u32>,
}

#[derive(Debug,Clone)]
pub(crate) struct WavefrontGroup{
    pub(crate)name : String,
    pub(crate)start_index : u32,
    pub(crate)end_index : u32,
}