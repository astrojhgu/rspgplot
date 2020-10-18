#[derive(Clone, Copy, PartialEq)]
pub enum FillStyle{
    Filled,
    Outline,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ArrayHeadStyle{
    pub fill_style:FillStyle,
    pub angle: f32, 
    pub barb: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct HatchingStyle{
    pub angle:f32, 
    pub sepn:f32,
    pub phase: f32,
}
