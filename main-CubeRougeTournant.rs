use three_d::*;

fn main() {
    // Initialisation de la fenêtre
    let window = Window::new(WindowSettings {
        title: "Cube 3D".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Contexte GPU
    let context = window.gl();

    // Création de la caméra
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(2.0, 2.0, 4.0),  // position
        vec3(0.0, 0.0, 0.0),  // cible
        vec3(0.0, 1.0, 0.0),  // up
        degrees(45.0),        // angle de vue
        0.1,                 // near
        1000.0,              // far
    );

    // Contrôle de la caméra (orbitale)
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Création d'un cube
    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()), // Géométrie du cube (1x1x1)
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(204, 76, 51), // Couleur rouge (R=204, G=76, B=51)
                ..Default::default()
            },
        ),
    );

    // Lumière directionnelle
    let light = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, -1.0));

    // Boucle de rendu principale
    window.render_loop(move |mut frame_input| {  // Notez le 'mut' ajouté ici
        // Mise à jour de la caméra
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Rotation du cube
        cube.set_transformation(Mat4::from_angle_y(Rad(frame_input.accumulated_time as f32 * 0.001)));

        // Rendu de la scène
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &[&cube], &[&light]);

        FrameOutput::default()
    });
}