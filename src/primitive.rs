#[path = "vec3.rs"] mod vec3;
pub use vec3::*;





pub enum Primitive {
    Sphere(Point3, f32, Rgb), // Center, Radius
    Plane(Vec3, f32, Rgb), // Plane Normal, Distance Along Normal
    Cuboid(Point3, Vec3, f32, Rgb), // Center, Dimensions, Rounding Value
    BoxFrame(Point3, Vec3, f32), // Center, Dimensions, Edge Thickness
    Torus(Point3, f32, f32), // Center, Inner Radius, Outer Radius
    // Horseshoe()
}

impl Primitive {
    pub fn id(&self) -> isize {
        match &self {
            Self::Sphere(_, _, _) => 1,
            Self::Plane(_, _, _) => 2,
            Self::Cuboid(_, _, _, _) => 3,
            // Self::Csg(_, _) => 4,
            _ => 0,
        }
    }
    pub fn as_data(&self) -> [[f32; 4]; 4] {
        match &self {
            Self::Sphere(center, rad, color) => [
                [1.0, *rad, 0.0, 0.0],
                [center.x, center.y, center.z, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Plane(normal, k, color) => [
                [2.0, *k, 0.0, 0.0],
                [normal.x, normal.y, normal.z, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Cuboid(center, dims, rounding, color) => [
                [3.0, *rounding, 0.0, 0.0],
                [center.x, center.y, center.z, 0.0],
                [dims.x, dims.y, dims.z, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            _ => [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
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