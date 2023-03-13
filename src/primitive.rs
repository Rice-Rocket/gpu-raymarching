#[allow(dead_code, unused)]
#[path = "vec3.rs"] mod vec3;
pub use vec3::*;




#[derive(PartialEq, Clone)]
pub enum Primitive {
    Sphere(f32, Rgb), // Radius
    Plane(Vec3, f32, Rgb), // Plane Normal, Distance Along Normal
    Cuboid(Vec3, f32, Rgb), // Dimensions, Rounding Value
    BoxFrame(Vec3, f32, Rgb), // Dimensions, Edge Thickness
    Torus(f32, f32, Rgb), // Inner Radius, Outer Radius
    // Horseshoe(f32, f32, f32, f32, Rgb), // Inner Radius, Outer Radius, Cap A, Cap B
    // Link(f32, f32, f32), // Length, Radius 1, Radius 2
    // Cone(f32, f32), // Angle, Height
    // HexagonalPrism(f32, f32), // Radius, Height
    // TriangularPrism(f32, f32), // Radius, Height
    // Capsule(Point3, Point3, f32), // Center 1, Center 2, Radius
    // CappedCylinder(Point3, Point3, f32), // Center 1, Center 2, Radius
    // RoundedCylinder(f32, f32, f32), // Radius Max, Radius Min, Height
    // CappedCone() // 
}

impl Primitive {
    pub fn id(&self) -> isize {
        match &self {
            Self::Sphere(_, _) => 1,
            Self::Plane(_, _, _) => 2,
            Self::Cuboid(_, _, _) => 3,
            Self::BoxFrame(_, _, _) => 4,
            Self::Torus(_, _, _) => 5,
            _ => 0,
        }
    }
    pub fn as_str(&self) -> String {
        match &self {
            Self::Sphere(_, _) => "Sphere".to_string(),
            Self::Plane(_, _, _) => "Plane".to_string(),
            Self::Cuboid(_, _, _) => "Cuboid".to_string(),
            Self::BoxFrame(_, _, _) => "Box Frame".to_string(),
            Self::Torus(_, _, _) => "Torus".to_string(),
        }
    }
    pub fn reposition(&self, pos: Point3) -> Self {
        match &self {
            Self::Sphere(radius, color) => Self::Sphere(*radius, *color),
            Self::Plane(norm, k, color) => Self::Plane(*norm, *k, *color),
            Self::Cuboid(dims, round, color) => Self::Cuboid(*dims, *round, *color),
            Self::BoxFrame(dims, edge_width, color) => Self::BoxFrame(*dims, *edge_width, *color),
            Self::Torus(in_rad, out_rad, color) => Self::Torus(*in_rad, *out_rad, *color),
        }
    }
    pub fn as_data(&self, op_group: f32) -> [[f32; 4]; 4] {
        match &self {
            Self::Sphere(rad, color) => [
                [1.0, *rad, 0.0, op_group],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Plane(normal, k, color) => [
                [2.0, *k, 0.0, op_group],
                [0.0, 0.0, 0.0, 0.0],
                [normal.x, normal.y, normal.z, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Cuboid(dims, rounding, color) => [
                [3.0, *rounding, 0.0, op_group],
                [0.0, 0.0, 0.0, 0.0],
                [dims.x, dims.y, dims.z, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::BoxFrame(dims, edge, color) => [
                [4.0, *edge, 0.0, op_group],
                [0.0, 0.0, 0.0, 0.0],
                [dims.x, dims.y, dims.z, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            Self::Torus(in_rad, out_rad, color) => [
                [3.0, *in_rad, *out_rad, op_group],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [color.x, color.y, color.z, 0.0],
            ],
            // Self::Cuboid(center, dims, color) => [
            //     [3.0, 0.0, 0.0, op_group],
            //     [0.0, 0.0, 0.0, 0.0],
            //     [0.0, 0.0, 0.0, 0.0],
            //     [color.x, color.y, color.z, 0.0],
            // ],
            _ => [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
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



#[derive(Clone, Copy)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
    // pub shear: Vec3,
}

impl Transform {
    pub fn new(translate: Vec3, rotate: Vec3, scale: Vec3) -> Self {
        Self {
            translate, 
            rotate, 
            scale
        }
    }
    pub fn translation(translate: Vec3) -> Self {
        Self {
            translate,
            rotate: vec3(0., 0., 0.),
            scale: vec3(1., 1., 1.)
        }
    }
    pub fn rotation(rotate: Vec3) -> Self {
        Self {
            translate: vec3(0., 0., 0.),
            rotate,
            scale: vec3(1., 1., 1.)
        }
    }
    pub fn scaler(scale: Vec3) -> Self {
        Self {
            translate: vec3(0., 0., 0.),
            rotate: vec3(0., 0., 0.),
            scale: scale
        }
    }
    pub fn transrot(translate: Vec3, rotate: Vec3) -> Self {
        Self {
            translate,
            rotate,
            scale: vec3(1., 1., 1.)
        }
    }
    pub fn transscale(translate: Vec3, scale: Vec3) -> Self {
        Self {
            translate,
            rotate: vec3(0., 0., 0.),
            scale
        }
    }
    pub fn rotscale(rotate: Vec3, scale: Vec3) -> Self {
        Self {
            translate: vec3(0., 0., 0.),
            rotate,
            scale
        }
    }
    pub fn none() -> Self {
        Self {
            translate: vec3(0., 0., 0.),
            rotate: vec3(0., 0., 0.),
            scale: vec3(1., 1., 1.)
        }
    }
    pub fn get_data(&self) -> [[f32; 4]; 4] {
        [
            [self.translate.x, self.translate.y, self.translate.z, 0.0],
            [self.rotate.x, self.rotate.y, self.rotate.z, 0.0],
            [self.scale.x, self.scale.y, self.scale.z, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]
    }
}