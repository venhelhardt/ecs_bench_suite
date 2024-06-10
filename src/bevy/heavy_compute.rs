use bevy_ecs::prelude::*;
use bevy_tasks::TaskPool;
use cgmath::*;

#[derive(Copy, Clone, Component)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone, Component)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone, Component)]
struct Velocity(Vector3<f32>);

#[derive(Copy, Clone, Component)]
struct Xform(Matrix4<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..1000).map(|_| {
            (
                Xform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let task_pool = TaskPool::new();
        let mut query = self.0.query::<(&mut Position, &mut Xform)>();

        query
            .par_iter_mut(&mut self.0)
            .for_each(|(mut pos, mut mat)| {
                for _ in 0..100 {
                    mat.0 = mat.0.invert().unwrap();
                }

                pos.0 = mat.0.transform_vector(pos.0);
            });
    }
}
