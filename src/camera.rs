#[path = "primitive.rs"] mod primitive;
pub use primitive::*;



#[derive(Clone)]
pub struct Camera {
    pub origin: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub focal_length: f32
}

impl Camera {
    pub fn new(origin: Point3, target: Point3, radius: f32, focal_length: f32) -> Self {
        let w = (target - origin).normalize();
        let p = Vec3::new(radius.sin(), radius.cos(), 0.0);
        let u = w.cross(p).normalize();
        let v = u.cross(w).normalize();
        Self {
            origin: origin,
            u, v, w,
            focal_length
        }
    }
    // pub fn move_by(&mut self, delta: Vec3, dt: f32) {
    //     self.origin = self.origin + (delta * self.forward).normalize() * dt;
    //     self.lower_left = self.lower_left + (delta * self.forward).normalize() * dt;
    // }
    // pub fn rotate_x(&mut self, delta: f32, dt: f32) {
    //     let left = Vec3::new(0., 1., 0.).cross(self.forward).normalize() * delta * dt;
    //     self.forward = (self.forward + left).normalize();
    //     self.horizontal = Vec3::new(0., 1., 0.).cross(self.forward).normalize();
    //     self.vertical = self.forward.cross(self.horizontal);
    //     self.lower_left = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.forward;
    // }
    pub fn as_data(&self) -> [[f32; 3]; 3] {
        [
            [self.u.x, self.v.x, self.w.x],
            [self.u.y, self.v.y, self.w.y],
            [self.u.z, self.v.z, self.w.z],
        ]
    }
    // pub fn get_ray(&self, uv: Vec2) -> (Point3, Vec3) {
    //     (
    //         self.origin, 
    //         (self.lower_left + self.horizontal * uv.x + self.vertical * uv.y - self.origin).normalize()
    //     )
    // }
}