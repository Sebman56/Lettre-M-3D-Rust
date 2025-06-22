use three_d::*;

fn main() {
    // Initialisation de la fenêtre
    let window = Window::new(WindowSettings {
        title: "Lettre M 3D".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Contexte GPU
    let context = window.gl();

    // Création de la caméra
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 10.0), // position
        vec3(0.0, 0.0, 0.0),  // cible
        vec3(0.0, 1.0, 0.0),  // up
        degrees(45.0),        // angle de vue
        0.1,                  // near
        1000.0,               // far
    );

    // Contrôle de la caméra (orbitale)
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Création des jambes de la lettre M (triangles)
    let jambe1 = create_triangle_fan(
        &context,
        &[
            vec3(-1.0, 1.0, -1.0), // Centre
            vec3(-2.0, -2.0, -1.0),
            vec3(-4.0, -2.0, -1.0),
            vec3(-2.0, 3.0, -1.0),
            vec3(-1.0, 3.0, -1.0),
            vec3(0.0, 2.0, -1.0),
            vec3(0.0, 0.0, -1.0)
        ],
        Srgba::new_opaque(255, 0, 0) // Rouge
    );

    let jambe2 = create_triangle_fan(
        &context,
        &[
            vec3(1.0, 1.0, -1.0), // Centre
            vec3(2.0, -2.0, -1.0),
            vec3(4.0, -2.0, -1.0),
            vec3(2.0, 3.0, -1.0),
            vec3(1.0, 3.0, -1.0),
            vec3(0.0, 2.0, -1.0),
            vec3(0.0, 0.0, -1.0)
        ],
        Srgba::new_opaque(0, 0, 255) // Bleu
    );

    let jambe3 = create_triangle_fan(
        &context,
        &[
            vec3(-1.0, 1.0, 1.0), // Centre
            vec3(-2.0, -2.0, 1.0),
            vec3(-4.0, -2.0, 1.0),
            vec3(-2.0, 3.0, 1.0),
            vec3(-1.0, 3.0, 1.0),
            vec3(0.0, 2.0, 1.0),
            vec3(0.0, 0.0, 1.0)
        ],
        Srgba::new_opaque(0, 255, 0) // Vert
    );

    let jambe4 = create_triangle_fan(
        &context,
        &[
            vec3(1.0, 1.0, 1.0), // Centre
            vec3(2.0, -2.0, 1.0),
            vec3(4.0, -2.0, 1.0),
            vec3(2.0, 3.0, 1.0),
            vec3(1.0, 3.0, 1.0),
            vec3(0.0, 2.0, 1.0),
            vec3(0.0, 0.0, 1.0)
        ],
        Srgba::new_opaque(255, 0, 255) // Rose
    );

    // Création des épaisseurs (quad strips)
    let epaisseur1 = create_quad_strip(&context, false); // Bleu
    let epaisseur2 = create_quad_strip(&context, true);  // Rouge

    // Lumière directionnelle
    let light = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, -1.0));

    // Boucle de rendu principale
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                &[&jambe1, &jambe2, &jambe3, &jambe4, &epaisseur1, &epaisseur2],
                &[&light]
            );

        FrameOutput::default()
    });
}

fn create_triangle_fan(context: &Context, points: &[Vec3], color: Srgba) -> Gm<Mesh, ColorMaterial> {
    // Création des indices pour un triangle fan (premier point comme centre)
    let mut indices = Vec::new();
    for i in 1..points.len()-1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i+1) as u32);
    }

    let colors = vec![color; points.len()];
    let positions = points.to_vec();

    let mut cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        colors: Some(colors),
        ..Default::default()
    };
    cpu_mesh.compute_normals();

    Gm::new(
        Mesh::new(context, &cpu_mesh),
        ColorMaterial {
            color,
            ..Default::default()
        }
    )
}

fn create_quad_strip(context: &Context, second_part: bool) -> Gm<Mesh, ColorMaterial> {
    let (points, color) = if !second_part {
        // Première partie de l'épaisseur
        (
            vec![
                vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, -1.0),
                vec3(-1.0, 1.0, 1.0), vec3(-1.0, 1.0, -1.0),
                vec3(-2.0, -2.0, 1.0), vec3(-2.0, -2.0, -1.0),
                vec3(-4.0, -2.0, 1.0), vec3(-4.0, -2.0, -1.0),
                vec3(-2.0, 3.0, 1.0), vec3(-2.0, 3.0, -1.0),
                vec3(-1.0, 3.0, 1.0), vec3(-1.0, 3.0, -1.0),
                vec3(0.0, 2.0, 1.0), vec3(0.0, 2.0, -1.0)
            ],
            Srgba::new_opaque(0, 0, 255) // Bleu
        )
    } else {
        // Deuxième partie de l'épaisseur
        (
            vec![
                vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, -1.0),
                vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, -1.0),
                vec3(2.0, -2.0, 1.0), vec3(2.0, -2.0, -1.0),
                vec3(4.0, -2.0, 1.0), vec3(4.0, -2.0, -1.0),
                vec3(2.0, 3.0, 1.0), vec3(2.0, 3.0, -1.0),
                vec3(1.0, 3.0, 1.0), vec3(1.0, 3.0, -1.0),
                vec3(0.0, 2.0, 1.0), vec3(0.0, 2.0, -1.0)
            ],
            Srgba::new_opaque(255, 0, 0) // Rouge
        )
    };

    // Création des indices pour un quad strip
    let mut indices = Vec::new();
    for i in 0..(points.len()/2 - 1) {
        let start = i * 2;
        indices.push(start as u32);
        indices.push((start + 1) as u32);
        indices.push((start + 2) as u32);
        indices.push((start + 1) as u32);
        indices.push((start + 2) as u32);
        indices.push((start + 3) as u32);
    }

    let colors = vec![color; points.len()];
    let positions = points; // On utilise directement le Vec<Vec3>

    let mut cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        colors: Some(colors),
        ..Default::default()
    };
    cpu_mesh.compute_normals();

    Gm::new(
        Mesh::new(context, &cpu_mesh),
        ColorMaterial {
            color,
            ..Default::default()
        }
    )
}