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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Just{
    Equal,
    NonEqual,
}

impl Just{
    pub fn to_i32(self)->i32{
        match self{
            Just::Equal=>1,
            _=>0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AxesStyle{
    Blank,
    BoxOnly,
    WithLabels,
    WithLabelsAndAxis0,
    WithLabelsAxis0AndGrid,
    LogX,
    LogY,
    LogXY
}

impl AxesStyle{
    pub fn to_i32(self)->i32{
        use AxesStyle::*;
        match self{
            Blank=>-2,
            BoxOnly=>-1,
            WithLabels=>0,
            WithLabelsAndAxis0=>1,
            WithLabelsAxis0AndGrid=>2,
            LogX=>10,
            LogY=>20,
            LogXY=>30,
        }
    }
}
