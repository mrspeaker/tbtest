use bevy::{
	ecs::{component::ComponentId, world::DeferredWorld},
	prelude::*,
    math::*
};
use bevy_flycam::prelude::*;
use bevy_trenchbroom::prelude::*;
use nil::prelude::*;
use std::f32::consts::*;

// The required worldspawn class makes up the main structural
// world geometry and settings. Exactly one exists in every map.
#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[geometry(GeometryProvider::new().smooth_by_default_angle())]
pub struct Worldspawn {
    pub fog_color: Color,
    pub fog_density: f32,
}

#[derive(PointClass, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[component(on_add = Self::on_add)]
pub struct Cube;
impl Cube {
	fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
		let Some(asset_server) = world.get_resource::<AssetServer>() else { return };
		let cube = asset_server.add(Mesh::from(Cuboid::new(0.42, 0.42, 0.42)));
		let material = asset_server.add(StandardMaterial::default());

		world.commands().entity(entity).insert((Mesh3d(cube), MeshMaterial3d(material)));
	}
}

#[derive(PointClass, Component, Reflect, Clone, Copy, SmartDefault)]
#[reflect(Component)]
#[require(Transform)]
pub struct Light {
	#[default(Color::srgb(1., 1., 1.))]
	pub _color: Color,
	#[default(300.)]
	pub light: f32,
}

#[derive(PointClass, Component, Reflect, Clone, Copy, SmartDefault)]
#[reflect(Component)]
#[require(Transform)]
pub struct InfoPlayerStart;

const TB_CONFIG_LOCATION: &str = "/Applications/TrenchBroom.app/Contents/Resources/Games/TbTest";

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.00005,
			speed: 6.,
		})
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::KeyE,
            move_descend: KeyCode::KeyQ,
            ..Default::default()
        })
		.add_plugins(TrenchBroomPlugin(
			TrenchBroomConfig::new("TbTest")
				.no_bsp_lighting(true)
				.register_class::<Worldspawn>()
		))
		.add_systems(PostStartup, setup_scene)
		.add_systems(Update, spawn_lights)
        .add_systems(Startup, write_trenchbroom_config)

		.run();
}

fn setup_scene(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(SceneRoot(asset_server.load("maps/map01.map#Scene")));

    // Sun light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 2.0 -0.4),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::linear_rgb(1.0,1.0, 1.0),
        brightness: 100.0,
    });


}

fn spawn_lights(
	mut commands: Commands,
	query: Query<(Entity, &Light),
	Changed<Light>>,
) {
	for (entity, light) in &query {
		commands.entity(entity).insert(PointLight {
			color: light._color,
			intensity: light.light * 300.,
			shadows_enabled: true,
			..default()
		});
	}
}


// Write out <folder_path>/GameConfig.cfg, <folder_path>/example_game.fgd
fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    if let Err(err) = server.config.write_folder(TB_CONFIG_LOCATION) {
        error!("Could not write TrenchBroom config: {err}");
    }
}
