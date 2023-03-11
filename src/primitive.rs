#[path = "vec3.rs"] mod vec3;
pub use vec3::*;






pub enum Primitive {
    Sphere(Point3, f32, Rgb),
    AAPlane(Axis, f32, Rgb),
    Cuboid(Vec3, Vec3, Rgb),
    // Csg([Option<Primitive>; 4], CsgOp),
}

impl Primitive {
    pub fn id(&self) -> isize {
        match &self {
            Self::Sphere(_, _, _) => 1,
            Self::AAPlane(_, _, _) => 2,
            Self::Cuboid(_, _, _) => 3,
            // Self::Csg(_, _) => 4,
            _ => 0,
        }
    }
    //* do something either with mats or arrays or vec4s to represent data of primitive
    //* as a single piece of data. 
    pub fn as_data(&self) -> [[f32; 4]; 4] {
        match &self {
            Self::Sphere(center, rad, color) => [
                [1.0, *rad, 0.0, 0.0],
                [center.x, center.y, center.z, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::AAPlane(axis, k, color) => [
                [2.0, axis.as_int() as f32, *k, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Cuboid(center, dims, color) => [
                [3.0, center.x, dims.x, 0.0],
                [0.0, center.y, dims.y, 0.0],
                [0.0, center.z, dims.z, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            // Self::Cuboid(center, dims, color) => [
            //     [3.0, 0.0, 0.0, 0.0],
            //     [0.0, 0.0, 0.0, 0.0],
            //     [0.0, 0.0, 0.0, 0.0],
            //     [color.x, color.y, color.z, 0.0],
            // ],
        }
    }
    // pub fn center(&self) -> Point3 {
    //     match &self {

    //     }
    // }
}


#[derive(Clone, Copy)]
pub enum CsgOp {
    Min,
    Max,
    SmoothMin(f32),
    SmoothMax(f32),
}



pub struct Csg {
    operator: CsgOp,
    objects: [Option<Primitive>; 2],
    obj_uids: [isize; 2],
}

// * Csgs have indices to two primitives. csgs just dictate how those two act together. 
// * in the end, there will be a list of csgs that will be passed to the shader
impl Csg {
    pub fn new(operator: CsgOp, objs: [Option<Primitive>; 2]) -> Self {
        Self {
            operator: operator,
            objects: objs,
            obj_uids: [0; 2]
        }
    }
    pub fn set_uids(&mut self, uids: Vec<isize>) {
        for (i, id) in uids.iter().enumerate() {
            self.obj_uids[i] = *id;
        }
    }
    pub fn as_array(&self) -> [isize; 2] {
        self.obj_uids
    }
    pub fn get(&self) -> [f32; 4] {
        match self.operator {
            CsgOp::Min => [1., self.obj_uids[0] as f32, self.obj_uids[1] as f32, 0.0],
            CsgOp::Max => [2., self.obj_uids[0] as f32, self.obj_uids[1] as f32, 0.0],
            CsgOp::SmoothMin(k) => [3., self.obj_uids[0] as f32, self.obj_uids[1] as f32, k],
            CsgOp::SmoothMax(k) => [4., self.obj_uids[0] as f32, self.obj_uids[1] as f32, k],
        }
    }
}