use std::fs::File;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::{plugins::particle::ParticleStartupSet, prelude::*};

pub fn debug_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FPSUpdateTimer{timer: Timer::from_seconds(1., TimerMode::Repeating)})
        .insert_resource(AverageEnergy(0.0))
        .add_systems(Startup, spawn_fps_text)
        .add_systems(Update, update_fps);
        // .add_systems(Startup, spawn_energy_text.before(ParticleStartupSet))
        // .add_systems(Update, (display_energy))
        // .init_resource::<CsvWriter>()
        // .insert_resource(EnergyTimer {
        //     timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        //     count: 0,
        // });
}

#[derive(Component)]
struct EnergyText;
#[derive(Component)]
struct FPSText;
#[derive(Resource)]
struct EnergyTimer{
    timer: Timer,
    count: u32,
}
#[derive(Resource)]
struct CsvWriter(csv::Writer<File>);
#[derive(Resource)]
struct FPSUpdateTimer{
    timer: Timer,
}
#[derive(Resource)]
struct AverageEnergy(f64);

impl Default for CsvWriter {
    fn default() -> Self {
        let datetime = chrono::Local::now();
        Self(csv::Writer::from_path(format!(".debug/{}.csv", datetime.format("%Y%m%d-%H%M%S"))).unwrap())
    }
}

/* -- Setup -- */
fn spawn_energy_text(mut commands: Commands, mut writer: ResMut<CsvWriter>) {
    commands.spawn(Text::new(""))
    .with_child((
        TextSpan::default(),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.3, 0.2, 1.0)),
        EnergyText,
    ));

    writer.0.write_record(["t", "K", "U", "E", "<E>"]);
}

fn spawn_fps_text(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS:"),
        TextFont {
            font_size: 35.,
            ..default()
        },
        TextColor(Color::linear_rgb(1.0, 0.8, 0.1))
    )).with_child((
        TextSpan::default(),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        FPSText,
        TextColor(Color::linear_rgb(1.0, 0.8, 0.1))
    ));
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<&mut TextSpan, With<FPSText>>,
    time: Res<Time>,
    mut timer: ResMut<FPSUpdateTimer>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        for mut span in query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() { **span = format!("{value:.2}") };
            }
        }
    }
}

/* -- Update -- */
fn display_energy(
    particle_query: Query<(&Velocity, &Transform), With<Particle>>,
    boundary_box_query: Single<&Transform, With<BoundaryBox>>,
    text_query: Single<&mut TextSpan, With<EnergyText>>,
    mut timer: ResMut<EnergyTimer>,
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut average_energy: ResMut<AverageEnergy>,
    mut writer: ResMut<CsvWriter>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        timer.count += 1;
        // let box_height = boundary_box_query.into_inner().scale.y;
        let mut kinetic_energy = 0.0;
        let mut potential_energy = 0.0;

        for (velocity, transform) in particle_query {
            // let y = transform.translation.y + box_height/2.;
            kinetic_energy += velocity.0.length_squared() / 2.;
            //potential_energy += gravity.magnitude * y;
            potential_energy += - gravity.magnitude * Vec2::from_angle(gravity.direction).dot(transform.translation.xy());
        }
        kinetic_energy /= PARTICLE_NUM.element_product() as f32;
        potential_energy /= PARTICLE_NUM.element_product() as f32;
        let total_energy = kinetic_energy + potential_energy;
        average_energy.0 = (average_energy.0 * (timer.count as f64 - 1.0) + total_energy as f64) / timer.count as f64;
        let mut span = text_query.into_inner();
        **span = format!("T: {kinetic_energy:.2}\nU: {potential_energy:.2}\nE: {total_energy:.2}\n<E>_t: {:.2}", average_energy.0);
        let _ = writer.0.write_record([(timer.timer.duration().as_secs() as u32).to_string(), kinetic_energy.to_string(), potential_energy.to_string(), total_energy.to_string(), average_energy.0.to_string()]);
    }
}

fn write_energy_to_file(mut writer: ResMut<CsvWriter>, mut exit_event: EventReader<AppExit>) {
    if !exit_event.is_empty() {
        exit_event.clear();
        writer.0.flush();
    }
}