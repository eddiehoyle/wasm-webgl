use specs::*;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            info!("Hello, {:?}", &position);
        }
    }
}

#[derive(Default)]
struct DeltaTime(f32);

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DeltaTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;

        // `Read` implements `Deref`, so it
        // coerces to `&DeltaTime`.
        let delta = delta.0;

        info!("Updating with delta: {}", delta);

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

pub fn run_ecs() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.add_resource(DeltaTime(0.05)); // Let's use some start value

    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    world.create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world.create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
}