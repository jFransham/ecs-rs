# ECS.rs

An [entity-component system](https://en.wikipedia.org/wiki/Entity_component_system)
written in Rust, with the main focus being avoiding macro usage in the consumer
while minimising boilerplate. The main breakthrough here is that Rust's type and
trait system allow for completely insane shenanigans. The implementation could
use more unsafe code in lieu of Any-based virtual calls, but other than that
monomorphisation and inlining means that basically all of the type shenanigans
still mean that the code is identical to hand-rolled boilerplate.

A complex, but reasonably real-world example (it's from my own game):

```rust
struct CameraComponent;
impl SetComponent for CameraComponent {}

struct RenderComponent {
	texture: Texture2d,
	depth: i8,
}
impl SetComponent for RenderComponent {}

struct VelocityComponent(f64, f64);
impl SetComponent for VelocityComponent {}

struct PositionComponent(f64, f64);
impl SetComponent for PositionComponent {}

struct ClipComponent(u32, u32, u32, u32);
impl SetComponent for ClipComponent {}

struct ScaleComponent(f64);
impl SetComponent for ScaleComponent {}

struct RenderSystem;

impl SimpleSystem<(), UpdateData> for RenderSystem {
	type Input = Either<(
		&'static RenderComponent,
		&'static PositionComponent,
		Option<&'static ClipComponent>,
		Option<&'static ScaleComponent>,
	), (
		&'static PositionComponent,
		&'static CameraComponent,
	)>;
	type Output = ();

	fn update(
		&mut self,
		entities: &[
			(
				EntityId,
				Either<(
					&RenderComponent,
					&PositionComponent,
					Option<&ClipComponent>,
					Option<&ScaleComponent>,
				), (
					&PositionComponent,
					&CameraComponent,
				)>
			)
		],
		ud: &UpdateData
	) -> Vec<(EntityId, (), ())> {
		unimplemented!();
	}
}
```

Each entity has a heterogenous list of components associated with it. These are
intended to be pure-data objects, which is why update receives an immutable
reference and not a mutable one. If you are not mutating a
given component you don't need to thread it through the function, which you _do_
need to do in most functional-style/immutable entity-component systems. If you
need more control over the entities, such as to add a new entity or to delete
existing entities or so forth, you can implement System instead of SimpleSystem.

## TODO:
* Allow running Output=() systems in parallel. This will probably be acheived
  by creating a new ReadonlySystem trait and blanket implementing it for
  `T: SimpleSystem<Input=_, Output=()> + Send`, and then having the SystemStore
  type have seperate lists of mutable and readonly systems. Maybe there would
  be seperate lists of pre-mutation and post-mutation RO systems but that would
  be added as I, personally, come to need it.
* Implement SetComponent for Either<A, B> (which would just forward to
  whichever of the two it is).
* Use impl specialization to implement SetComponent for T.
