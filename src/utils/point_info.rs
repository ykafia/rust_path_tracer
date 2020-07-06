use super::*;

#[derive(Copy,Clone,Debug)]
pub struct PointInfo {
    pub distance : f32,
    pub normal : Vector3<f32>,
    pub intersection : Vector3<f32>
}

impl PartialEq<PointInfo> for PointInfo {
    fn eq(&self, other: &PointInfo) -> bool { 
        self.distance==other.distance
    }
    
}
impl PartialOrd<PointInfo> for PointInfo {
    fn partial_cmp(&self, other: &PointInfo) -> Option<std::cmp::Ordering> {
        if self.distance > other.distance {
            Some(std::cmp::Ordering::Greater)
        }
        else if self.distance < other.distance{
            Some(std::cmp::Ordering::Less)
        }
        else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

pub struct ECSRayInfo<'a>{
    pub e : ( &'a RenderableComponent, &'a TransformComponent),
    pub pi : PointInfo
}

impl<'a> ECSRayInfo<'a> {
    pub fn new(e : ( &'a RenderableComponent, &'a TransformComponent), pi : PointInfo) -> ECSRayInfo {
        ECSRayInfo {
            e : e,
            pi : pi
        }
    }
}

pub struct RayInfo<'a>(pub &'a Element,pub PointInfo);



impl<'a> PartialEq<RayInfo<'a>> for RayInfo<'a> {
    fn eq(&self, other: &RayInfo) -> bool { 
        self.1 == other.1
    }
}
impl<'a> PartialOrd<RayInfo<'a>> for RayInfo<'a> {
    fn partial_cmp(&self, other: &RayInfo) -> Option<std::cmp::Ordering> {
        if self.1 > other.1 {
            Some(std::cmp::Ordering::Greater)
        }
        else if self.1 < other.1{
            Some(std::cmp::Ordering::Less)
        }
        else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
impl<'a> Eq for RayInfo<'a> {}

impl<'a> Ord for RayInfo<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { 
        if self.1 > other.1 {
            std::cmp::Ordering::Greater
        }
        else if self.1 < other.1{
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Equal
        }
    }
}

impl<'a> PartialEq<ECSRayInfo<'a>> for ECSRayInfo<'a> {
    fn eq(&self, other: &ECSRayInfo) -> bool { 
        self.pi == other.pi
    }
}
impl<'a> PartialOrd<ECSRayInfo<'a>> for ECSRayInfo<'a> {
    fn partial_cmp(&self, other: &ECSRayInfo) -> Option<std::cmp::Ordering> {
        if self.pi > other.pi {
            Some(std::cmp::Ordering::Greater)
        }
        else if self.pi < other.pi{
            Some(std::cmp::Ordering::Less)
        }
        else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
impl<'a> Eq for ECSRayInfo<'a> {}

impl<'a> Ord for ECSRayInfo<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { 
        if self.pi > other.pi {
            std::cmp::Ordering::Greater
        }
        else if self.pi < other.pi{
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Equal
        }
    }
}

