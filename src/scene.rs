#[path = "camera.rs"] mod camera;
pub use camera::*;



pub struct Scene {
    pub objects: Vec<(Primitive, usize)>,
    pub lights: Vec<Vec3>,
    pub bool_ops: Vec<BooleanOp>,
    pub camera: Camera,
    uid_counter: isize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(MAX_OBJECTS),
            lights: Vec::with_capacity(MAX_LIGHTS),
            bool_ops: Vec::new(),
            camera: Camera::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), 0.0, 2.5),
            uid_counter: 0,
        }
    }
    pub fn add(&mut self, object: Primitive) -> usize {
        self.objects.push((object, 0));
        self.uid_counter += 1;
        return (self.uid_counter - 1) as usize;
    }
    pub fn add_light(&mut self, point: Point3) {
        self.lights.push(point);
    }
    pub fn add_bool_op(&mut self, mut bool_op: BooleanOp) {
        bool_op.uid = self.bool_ops.len() + 1;
        for i in bool_op.obj_uids.iter() {
            self.objects[*i].1 = bool_op.uid;
        }
        self.bool_ops.push(bool_op);
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn get_objects(&mut self) -> [[[f32; 4]; 4]; MAX_OBJECTS] {
        let mut arr = [[[0.; 4]; 4]; MAX_OBJECTS];
        self.objects.sort_by_key(|x| x.1);
        for (i, obj) in self.objects.iter().enumerate() {
            arr[i] = obj.0.as_data(obj.1 as f32);
        }
        return arr;
    }
    pub fn get_lights(&self) -> [[f32; 4]; MAX_LIGHTS] {
        let mut arr = [[0.0; 4]; MAX_LIGHTS];
        for (i, light) in self.lights.iter().enumerate() {
            arr[i] = [light.x, light.y, light.z, 1.0];
        };
        return arr;
    }
    pub fn get_bool_ops(&self) -> [[f32; 2]; MAX_BOOL_OPS] {
        let mut arr = [[0.0; 2]; MAX_BOOL_OPS];
        for (i, op) in self.bool_ops.iter().enumerate() {
            arr[i] = op.get();
        };
        println!("{:?}", arr);
        return arr;
    }
}


#[derive(Clone, Copy)]
pub struct UniformBlockObjects {
    pub objects: [[[f32; 4]; 4]; MAX_OBJECTS],
}
#[derive(Clone, Copy)]
pub struct UniformBlockLights {
    pub lights: [[f32; 4]; MAX_LIGHTS],
}
#[derive(Clone, Copy)]
pub struct UniformBlockBoolOps {
    pub bool_ops: [[f32; 2]; MAX_BOOL_OPS],
}
implement_uniform_block!(UniformBlockObjects, objects);
implement_uniform_block!(UniformBlockLights, lights);
implement_uniform_block!(UniformBlockBoolOps, bool_ops);

#[derive(Clone, Copy)]
pub struct SceneSettingsBlock {
    pub background_color: [f32; 4],
    pub fog_color: [f32; 4],
}
implement_uniform_block!(SceneSettingsBlock, background_color, fog_color);