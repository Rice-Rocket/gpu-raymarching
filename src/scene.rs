#[path = "camera.rs"] mod camera;
pub use camera::*;



pub struct Scene {
    pub objects: Vec<Primitive>,
    pub lights: Vec<Vec3>,
    pub csgs: Vec<Csg>,
    pub camera: Camera,
    uid_counter: isize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(MAX_OBJECTS),
            lights: Vec::with_capacity(MAX_LIGHTS),
            csgs: Vec::new(),
            camera: Camera::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.)),
            uid_counter: 0,
        }
    }
    pub fn add(&mut self, object: Primitive) {
        self.objects.push(object);
        self.uid_counter += 1;
    }
    pub fn add_light(&mut self, point: Point3) {
        self.lights.push(point);
    }
    pub fn add_csg(&mut self, csg: Csg) {
        self.csgs.push(csg);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn get_objects(&self) -> [[[f32; 4]; 4]; MAX_OBJECTS] {
        let mut arr = [[[0.; 4]; 4]; MAX_OBJECTS];
        for (i, obj) in self.objects.iter().enumerate() {
            arr[i] = obj.as_data();
        }
        return arr;
    }
    pub fn get_lights(&self) -> [[f32; 3]; MAX_LIGHTS] {
        let mut arr = [[0.0; 3]; MAX_LIGHTS];
        for (i, light) in self.lights.iter().enumerate() {
            arr[i] = light.to_tuple();
        };
        return arr;
    }
    pub fn get_csgs(&self) -> [[f32; 4]; MAX_CSGS] {
        let mut arr = [[0.0; 4]; MAX_CSGS];
        for (i, csg) in self.csgs.iter().enumerate() {
            arr[i] = csg.get();
        };
        return arr;
    }
}


#[derive(Clone, Copy)]
pub struct UniformBlock {
    pub objects: [[[f32; 4]; 4]; MAX_OBJECTS],
    pub lights: [[f32; 3]; MAX_LIGHTS],
    pub csgs: [[f32; 4]; MAX_CSGS],
}
implement_uniform_block!(UniformBlock, objects, lights, csgs);