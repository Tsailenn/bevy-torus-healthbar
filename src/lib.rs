use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Mesh2dHandle,
};
/// Component to represent a radial bar's data.
///
/// Provides automatic mesh-generation. Use the ```mesh_handle``` field to access the mesh or with the ```fn set_value(...)``` and ```fn add_value(...)``` functions.
///
/// Component also holds the ColorMaterial handle for you (mandatory for now).
///
/// Use the ```fn new(...)``` function to create this component.
#[derive(Debug, Clone, Component)]
pub struct RadialBar2D {
    pub hole_radius: f32,
    pub circle_radius: f32,
    pub value: f32,
    max_value: f32,
    segment_count: usize,
    pub mesh_handle: Mesh2dHandle,
    pub color_handle: Handle<ColorMaterial>,
}

impl RadialBar2D {
    const FULL_ROTATION_RAD: f32 = 6.28318530718;
    const NORMAL: [f32; 3] = [0., 0., 1.];
    const UV: [f32; 2] = [0., 1.];

    ///
    /// # Examples
    /// ```
    /// fn test_spawn_valuebar(
    ///     mut commands: Commands,
    ///     mut meshes: ResMut<Assets<Mesh>>,
    ///     mut materials: ResMut<Assets<ColorMaterial>>,
    /// ) {
    ///     let torus_value = RadialBar2D::new(
    ///         0.1,
    ///         0.2,
    ///         360.,
    ///         360.,
    ///         18,
    ///         Color::MIDNIGHT_BLUE,
    ///         &mut meshes,
    ///         &mut materials,
    ///     );
    ///     commands
    ///         .spawn_bundle(MaterialMesh2dBundle {
    ///             mesh: torus_value.mesh_handle.clone(),
    ///             material: torus_value.color_handle.clone(),
    ///             ..default()
    ///         })
    ///         .insert(torus_value);
    /// }
    /// ```
    pub fn new(
        hole_radius: f32,
        circle_radius: f32,
        value: f32,
        max_value: f32,
        segment_count: usize,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let vertices = RadialBar2D::generate_vertices(circle_radius, hole_radius, segment_count);
        let tri_indices = RadialBar2D::form_valuebar_triangles(segment_count, max_value, value);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];
        for _vertex in 0..vertices.len() {
            normals.push(RadialBar2D::NORMAL.clone());
            uvs.push(RadialBar2D::UV.clone());
        }

        //set vertex position
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
                .iter()
                .map(|x| x.to_array())
                .collect::<Vec<[f32; 3]>>(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        //set color
        let color_handle = materials.add(ColorMaterial::from(color));
        // mesh.insert_attribute(
        //     MeshVertexAttribute::new(
        //         "Vertex_Color",
        //         rand::thread_rng().gen::<usize>(),
        //         VertexFormat::Uint32,
        //     ),
        //     vec![color.as_linear_rgba_u32(); vertices.len()],
        // );

        //set triangulation
        let mut flattened_indices: Vec<u32> = vec![];
        for tri in tri_indices.iter() {
            flattened_indices.extend_from_slice(tri);
        }
        mesh.set_indices(Some(Indices::U32(flattened_indices)));

        //wrap it handler
        let handler = Mesh2dHandle(meshes.add(mesh));

        RadialBar2D {
            hole_radius,
            circle_radius,
            value,
            max_value,
            segment_count,
            mesh_handle: handler,
            color_handle,
        }
    }

    /// Generates vertices for the mesh
    pub fn generate_vertices(
        circle_radius: f32,
        hole_radius: f32,
        segment_count: usize,
    ) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = vec![];

        for rot in 0..segment_count {
            let mut outer = Vec3::new(0., circle_radius, 0.);
            let mut inner = Vec3::new(0., hole_radius, 0.);

            outer = RadialBar2D::calculate_position(
                &outer,
                (rot as f32 / segment_count as f32) * RadialBar2D::FULL_ROTATION_RAD,
            );
            inner = RadialBar2D::calculate_position(
                &inner,
                (rot as f32 / segment_count as f32) * RadialBar2D::FULL_ROTATION_RAD,
            );

            vertices.push(outer);
            vertices.push(inner);
        }

        vertices
    }

    /// Calculates vertex position of the mesh
    fn calculate_position(pos: &Vec3, rad: f32) -> Vec3 {
        let mut result = *pos;
        let x = result.x;
        let y = result.y;

        let x_ = x * rad.cos() - y * rad.sin();
        let y_ = y * rad.cos() + x * rad.sin();

        result.x = x_;
        result.y = y_;

        result
    }

    /// Forms triangulation with indices. Ignoring the current ```value```.
    pub fn form_triangles(segment_count: usize) -> Vec<[u32; 3]> {
        let mut triangles: Vec<[u32; 3]> = vec![];
        for segment_index in 0..segment_count {
            triangles.push([
                (segment_index * 2) as u32,
                ((segment_index * 2 + 2) % (segment_count * 2)) as u32,
                ((segment_index * 2 + 2) % (segment_count * 2) + 1) as u32,
            ]);
            triangles.push([
                (segment_index * 2) as u32,
                ((segment_index * 2 + 2) % (segment_count * 2) + 1) as u32,
                ((segment_index * 2) + 1) as u32,
            ]);
        }

        triangles
    }

    /// Forms triangulation with indices, affected by how much the current ```value``` is compared to the ```max_value```
    pub fn form_valuebar_triangles(
        segment_count: usize,
        max_value: f32,
        value: f32,
    ) -> Vec<[u32; 3]> {
        let mut triangles = RadialBar2D::form_triangles(segment_count);

        let value_float = value / max_value;
        let detached_value_segments = ((1. - value_float) * segment_count as f32).floor() as usize;

        if detached_value_segments > 0 {
            for _segment in 0..detached_value_segments {
                triangles.pop();
                triangles.pop();
            }
        }

        triangles
    }

    /// Applies re-triangulation to the mesh, based on ```value```. Requires access to mutable meshes asset.
    pub fn regenerate_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
        //set triangulation
        let mut indices = vec![];
        for tri in
            RadialBar2D::form_valuebar_triangles(self.segment_count, self.max_value, self.value)
        {
            indices.extend_from_slice(&tri);
        }
        if let Some(mesh) = meshes.get_mut(&self.mesh_handle.0) {
            mesh.set_indices(Some(Indices::U32(indices)));
        }

        self.mesh_handle.clone()
    }

    /// Adds parameter ```val``` to component's ```value``` field
    pub fn add_value(&mut self, meshes: &mut ResMut<Assets<Mesh>>, val: f32) -> Mesh2dHandle {
        self.value += val;
        self.value = self.value.clamp(0., self.max_value);

        self.regenerate_mesh(meshes)
    }

    /// Sets component's ```value``` field
    pub fn set_value(&mut self, meshes: &mut ResMut<Assets<Mesh>>, val: f32) -> Mesh2dHandle {
        self.value = val;

        self.regenerate_mesh(meshes)
    }
}
