use bevy::math::*;
use bevy::{
	ecs::{component::ComponentId, world::DeferredWorld},
	prelude::*,
};
use bevy_flycam::prelude::*;
use bevy_trenchbroom::prelude::*;
use nil::prelude::*;
use std::f32::consts::*;

// TODO: We aren't using inventory to register here because it's broken on wasm.

#[derive(SolidClass, Component, Reflect)]
#[no_register]
#[reflect(Component)]
#[geometry(GeometryProvider::new().smooth_by_default_angle().render())]
pub struct Worldspawn;

#[derive(SolidClass, Component, Reflect)]
#[no_register]
#[reflect(Component)]
#[require(Transform)]
#[geometry(GeometryProvider::new().smooth_by_default_angle().render())]
pub struct FuncDoor;

#[derive(PointClass, Component, Reflect)]
#[no_register]
#[reflect(Component)]
#[require(Transform)]
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
#[no_register]
#[reflect(Component)]
#[require(Transform)]
pub struct Light {
	#[default(Color::srgb(1., 1., 1.))]
	pub _color: Color,
	#[default(300.)]
	pub light: f32,
}

#[derive(PointClass, Component, Reflect, Clone, Copy, SmartDefault)]
#[no_register]
#[reflect(Component)]
#[require(Transform)]
pub struct InfoPlayerStart;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin {
			default_sampler: repeating_image_sampler(false),
		}))
		// bevy_flycam setup so we can get a closer look at the scene, mainly for debugging
		.add_plugins(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.00005,
			speed: 6.,
		})
		.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
		.add_plugins(TrenchBroomPlugin(
			TrenchBroomConfig::new("TbTest")
				.no_bsp_lighting(true)
				.register_class::<Worldspawn>()
				.register_class::<Cube>()
				.register_class::<Light>()
				.register_class::<FuncDoor>(),
		))
		.add_systems(PostStartup, setup_scene)
		.add_systems(Update, spawn_lights)
        .add_systems(Startup, write_trenchbroom_config)

		.run();
}

fn setup_scene(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut projection_query: Query<&mut Projection>,
) {
	commands.spawn(SceneRoot(asset_server.load("maps/map-01.map#Scene")));

	// Wide FOV
	for mut projection in &mut projection_query {
		*projection = Projection::Perspective(PerspectiveProjection {
			fov: 90_f32.to_radians(),
			..default()
		});
	}

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

fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    if let Err(err) = server.config.write_folder("/Applications/TrenchBroom.app/Contents/Resources/Games/TbTest") {
        error!("Could not write TrenchBroom config: {err}");
    }

    // This will write <folder_path>/GameConfig.cfg,
    // and <folder_path>/example_game.fgd
}
