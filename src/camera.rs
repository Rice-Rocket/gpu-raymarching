#[path = "primitive.rs"] mod primitive;
pub use primitive::*;



#[derive(Clone)]
pub struct Camera {
    pub origin: Point3,
    pub forward: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3) -> Self {
        let w = (look_from - look_at).normalize();
        let u = Vec3::new(0., 1., 0.).cross(w).normalize();
        let v = w.cross(u);

        let lower_left = look_from - u / 2.0 - v / 2.0 - w;
        Self {
            origin: look_from,
            forward: w,
            horizontal: u,
            vertical: v,
            lower_left: lower_left,
        }
    }
    pub fn move_by(&mut self, delta: Vec3, dt: f32) {
        self.origin = self.origin + (delta * self.forward).normalize() * dt;
        self.lower_left = self.lower_left + (delta * self.forward).normalize() * dt;
    }
    pub fn rotate_x(&mut self, delta: f32, dt: f32) {
        let left = Vec3::new(0., 1., 0.).cross(self.forward).normalize() * delta * dt;
        self.forward = (self.forward + left).normalize();
        self.horizontal = Vec3::new(0., 1., 0.).cross(self.forward).normalize();
        self.vertical = self.forward.cross(self.horizontal);
        self.lower_left = self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.forward;
    }
    pub fn as_data(&self) -> [[f32; 4]; 4] {
        [
            [self.lower_left.x, self.horizontal.x, self.vertical.x, self.origin.x],
            [self.lower_left.y, self.horizontal.y, self.vertical.y, self.origin.y],
            [self.lower_left.z, self.horizontal.z, self.vertical.z, self.origin.z],
            [0.0, 0.0, 0.0, 0.0],
        ]
    }
    pub fn get_ray(&self, uv: Vec2) -> (Point3, Vec3) {
        (
            self.origin, 
            (self.lower_left + self.horizontal * uv.x + self.vertical * uv.y - self.origin).normalize()
        )
    }
}