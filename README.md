Shipyard is an Entity Component System focused on usability and speed.

[![LICENSE](https://img.shields.io/crates/l/shipyard)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/shipyard)](https://crates.io/crates/shipyard)
[![Documentation](https://docs.rs/shipyard/badge.svg)](https://docs.rs/shipyard)
[![Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg)](https://shipyard.zulipchat.com)

If you have any question or want to follow the development more closely join the [Zulip](https://shipyard.zulipchat.com).

There's two big learning resources:
- (Soon™) The Tutorial for people new to ECS or preferring to learn by making a project.
- [The Guide](https://leudz.github.io/shipyard/book) for people that already know how to use an ECS and mostly want to learn Shipyard's syntax.  
  It also go more in depth and provide useful recipes for common patterns.

## Simple Example
```rust
use shipyard::*;

struct Health(f32);
struct Position {
    _x: f32,
    _y: f32,
}

fn in_acid(positions: View<Position>, mut healths: ViewMut<Health>) {
    for (_, mut health) in (&positions, &mut healths)
        .iter()
        .filter(|(pos, _)| is_in_acid(pos))
    {
        health.0 -= 1.0;
    }
}

fn is_in_acid(_: &Position) -> bool {
    // it's wet season
    true
}

fn main() {
    let world = World::new();

    world.run(
        |mut entities: EntitiesViewMut,
         mut positions: ViewMut<Position>,
         mut healths: ViewMut<Health>| {
            entities.add_entity(
                (&mut positions, &mut healths),
                (Position { _x: 0.0, _y: 0.0 }, Health(1000.0)),
            );
        },
    );

    world.run(in_acid);
}
```

## Table of Contents
- [Let there be SparseSets](#let-there-be-sparsesets)
- [Systems](#systems)
  - [Not just storage](#not-just-storage)
  - [Return](#return)
  - [Generics](#generics)
  - [All at once](#all-at-once)
- [Unique Storage (Resource)](#unique-storage-resource)
- [!Send and !Sync Components](#send-and-sync-components)
- [Cargo Features](#cargo-features)
- [Unsafe](#unsafe)
- [License](#license)
- [Contributing](#contributing)

## Let there be SparseSets

I initially started to make an ECS to learn how it works. A failed attempt and some research later, I started to work on Shipyard.

[Specs](https://github.com/amethyst/specs) was already well established as the go-to Rust ECS but I thought I could do better and went with [EnTT](https://github.com/skypjack/entt) core data-structure: `SparseSet`.

It's extremely flexible and is the core data-structure behind Shipyard.  
Today I wouldn't say Shipyard is better or worse than Specs, it's just different.

## Systems

Systems make it very easy to split your logic in manageable chunk. Shipyard takes the concept quite far.

You always start with a function or closure and almost always take a few views (reference to storage) as arguments.  
The basic example shown above does just that:
```rust
fn in_acid(positions: View<Position>, mut healths: ViewMut<Health>) {
    // -- snip --
}
```
A function with two views as argument.

### Not just storage

The first argument doesn't have to be a view, you can pass any data to a system. You don't even have to own it.

```rust
fn in_acid(season: &Season, positions: View<Position>, mut healths: ViewMut<Health>) {
    // -- snip --
}

world.run_with_data(in_acid, &season);
```
You have to provide the data when running the system of course.

### Return

Systems can also have a return type, if run directly with `World::run` or `AllStorages::run` you'll get the returned value right away.  
For workloads you can only get back errors.

```rust
fn lowest_hp(healths: View<Health>) -> EntityId {
    // -- snip --
}

let entity = world.run(lowest_hp);
```

### Generics

Just like any function you can add some generics. You'll have to specify them when running the system.

```rust
fn in_acid<F: Float>(positions: View<Position<F>>, mut healths: ViewMut<Health>) {
    // -- snip --
}

world.run(in_acid::<f32>);
```

### All at once

You can of course use all of them at the same time.

```rust
fn debug<T: Debug + 'static>(fmt: &mut Formatter, view: View<T>) -> Result<(), fmt::Error> {
    // -- snip --
}

world.run_with_data(debug::<u32>, fmt)?;
```

## Unique Storage (Resource)

Unique storages are used to store data you only have once in the `World` and aren't related to any entity.

```rust
fn render(renderer: UniqueView<Renderer>) {
    // -- snip --
}

world.add_unique(Renderer::new());
```

## !Send and !Sync Components

`!Send` and `!Sync` components can be stored directly in the `World` and accessed almost just like any other component.  
Make sure to add the cargo feature to have access to this functionality.

```rust
fn run(rcs: NonSendSync<View<Rc<u32>>>) {
    // -- snip --
}
```

## Cargo Features

- **panic** *(default)* adds panicking functions
- **parallel** *(default)* &mdash; adds parallel iterators and dispatch
- **serde** &mdash; adds (de)serialization support with [serde](https://github.com/serde-rs/serde)
- **non_send** &mdash; add methods and types required to work with `!Send` components
- **non_sync** &mdash; add methods and types required to work with `!Sync` components
- **std** *(default)* &mdash; let shipyard use the standard library

## Unsafe

This crate uses `unsafe` both because sometimes there's no way around it, and for performance gain.  
Releases should have all invocation of `unsafe` explained.  
If you find places where a safe alternative is possible without repercussion (small ones are sometimes acceptable) please open an issue or a PR.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
