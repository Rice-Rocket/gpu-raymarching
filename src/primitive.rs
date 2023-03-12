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
    pub fn as_data(&self, op_group: f32) -> [[f32; 4]; 4] {
        match &self {
            Self::Sphere(center, rad, color) => [
                [1.0, *rad, 0.0, op_group],
                [center.x, center.y, center.z, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Plane(normal, k, color) => [
                [2.0, *k, 0.0, op_group],
                [normal.x, normal.y, normal.z, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Cuboid(center, dims, rounding, color) => [
                [3.0, *rounding, 0.0, op_group],
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
            //     [3.0, 0.0, 0.0, op_group],
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
pub enum BooleanOpType {
    Union,
    Intersect,
    Subtract,
    SmoothUnion(f32),
    SmoothIntersect(f32),
}



pub struct BooleanOp {
    operator: BooleanOpType,
    pub obj_uids: Vec<usize>,
    pub uid: usize
}

// * Csgs have indices to two primitives. csgs just dictate how those two act together. 
// * in the end, there will be a list of csgs that will be passed to the shader
impl BooleanOp {
    pub fn new(operator: BooleanOpType, objs: Vec<usize>) -> Self {
        Self {
            operator: operator,
            obj_uids: objs,
            uid: 0
        }
    }
    pub fn get(&self) -> [f32; 2] {
        match self.operator {
            BooleanOpType::Union => [1., 0.0],
            BooleanOpType::Intersect => [2., 0.0],
            BooleanOpType::Subtract => [3., 0.0],
            BooleanOpType::SmoothUnion(k) => [4., k],
            BooleanOpType::SmoothIntersect(k) => [5., k],
        }
    }
}