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

#[derive(Clone)]
pub struct RayInfo(pub Element,pub PointInfo);



impl PartialEq<RayInfo> for RayInfo {
    fn eq(&self, other: &RayInfo) -> bool { 
        self.1 == other.1
    }
}
impl PartialOrd<RayInfo> for RayInfo {
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
impl Eq for RayInfo {}

impl Ord for RayInfo {
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
