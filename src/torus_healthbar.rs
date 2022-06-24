use bevy::{
    ecs::query::WorldQuery,
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
        render_resource::VertexFormat,
    },
    sprite::Mesh2dHandle,
};
use bevy_inspector_egui::Inspectable;
use rand::Rng;

#[derive(Debug, Clone, Component, Inspectable)]
pub struct TorusHealthbar2D {
    pub hole_radius: f32,
    pub circle_radius: f32,
    pub health: f32,
    max_health: f32,
    segment_count: usize,
    vertices: Vec<Vec3>,
    pub mesh_handle: Mesh2dHandle,
    pub color_handle: Handle<ColorMaterial>,
}

// impl Default for TorusHealthbar2D {
//     fn default() -> Self {
//         // TorusHealthbar2D {
//         //     hole_radius: 10.,
//         //     circle_radius: 20.,
//         //     health: 30.,
//         //     max_health: 30.,
//         //     segment_count: 6,
//         //     vertices: (),
//         //     mesh: todo!(),
//         //     color: todo!(),
//         // }
//         TorusHealthbar2D::new(10., 20., 30., 30., 6, Color::ALICE_BLUE)
//     }
// }

impl TorusHealthbar2D {
    const FULL_ROTATION_RAD: f32 = 6.28318530718;

    pub fn new(
        hole_radius: f32,
        circle_radius: f32,
        health: f32,
        max_health: f32,
        segment_count: usize,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let vertices =
            TorusHealthbar2D::generate_vertices(circle_radius, hole_radius, segment_count);
        let tri_indices =
            TorusHealthbar2D::form_healthbar_triangles(segment_count, max_health, health);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        const normal: [f32; 3] = [0., 0., 1.];
        const uv: [f32; 2] = [0., 1.];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];
        for vertex in 0..vertices.len() {
            normals.push(normal.clone());
            uvs.push(uv.clone());
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
        // tri_indices
        //     .iter()
        //     .map(|x| (&mut flattened_indices).extend_from_slice(x));
        mesh.set_indices(Some(Indices::U32(flattened_indices)));

        //wrap it handler
        let handler = Mesh2dHandle(meshes.add(mesh));

        TorusHealthbar2D {
            hole_radius,
            circle_radius,
            health,
            max_health,
            segment_count,
            vertices,
            mesh_handle: handler,
            color_handle,
        }
        // let mut torus = TorusHealthbar2D {
        //     hole_radius,
        //     circle_radius,
        //     health,
        //     max_health,
        //     segment_count,
        //     vertices: vec![],
        //     mesh: Mesh::new(PrimitiveTopology::TriangleList),
        //     color,
        // };

        // //calculate vertices
        // torus.vertices =
        //     TorusHealthbar2D::generate_vertices(circle_radius, hole_radius, segment_count);
        // let mut chungus: Vec<[f32; 3]> = vec![];
        // for v in &torus.vertices {
        //     chungus.push(v.to_array());
        // }
        // torus
        //     .mesh
        //     .insert_attribute(Mesh::ATTRIBUTE_POSITION, chungus);

        // //set color
        // let v_color = vec![color; torus.vertices.len()];
        // torus.mesh.insert_attribute(
        //     MeshVertexAttribute::new(
        //         "Vertex_Color",
        //         rand::thread_rng().gen::<usize>(),
        //         VertexFormat::Uint32,
        //     ),
        //     vec![color.as_linear_rgba_u32(); torus.vertices.len()],
        // );

        // //set triangulation
        // let mut indices = vec![];
        // for tri in TorusHealthbar2D::form_healthbar_triangles(segment_count, max_health, health) {
        //     indices.extend_from_slice(&tri);
        // }
        // torus.mesh.set_indices(Some(Indices::U32(indices)));

        // torus
    }

    pub fn generate_vertices(
        circle_radius: f32,
        hole_radius: f32,
        segment_count: usize,
    ) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = vec![];

        for rot in 0..segment_count {
            let mut outer = Vec3::new(0., circle_radius, 0.);
            let mut inner = Vec3::new(0., hole_radius, 0.);

            outer = TorusHealthbar2D::calculate_position(
                &outer,
                (rot as f32 / segment_count as f32) * TorusHealthbar2D::FULL_ROTATION_RAD,
            );
            inner = TorusHealthbar2D::calculate_position(
                &inner,
                (rot as f32 / segment_count as f32) * TorusHealthbar2D::FULL_ROTATION_RAD,
            );

            vertices.push(outer);
            vertices.push(inner);
        }

        vertices
    }

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

    pub fn form_triangles(segment_count: usize) -> Vec<[u32; 3]> {
        let mut triangles: Vec<[u32; 3]> = vec![];
        for segment_index in 0..segment_count {
            //let outer_vertex_index = usize * 2;
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

    pub fn form_healthbar_triangles(
        segment_count: usize,
        max_health: f32,
        health: f32,
    ) -> Vec<[u32; 3]> {
        let mut triangles = TorusHealthbar2D::form_triangles(segment_count);

        let health_float = health / max_health;
        let detached_health_segments =
            ((1. - health_float) * segment_count as f32).floor() as usize;

        if detached_health_segments > 0 {
            for _segment in 0..detached_health_segments {
                triangles.pop();
                triangles.pop();
            }
        }

        triangles
    }

    pub fn regenerate_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
        //set triangulation
        let mut indices = vec![];
        for tri in TorusHealthbar2D::form_healthbar_triangles(
            self.segment_count,
            self.max_health,
            self.health,
        ) {
            indices.extend_from_slice(&tri);
        }
        if let Some(mesh) = meshes.get_mut(&self.mesh_handle.0) {
            mesh.set_indices(Some(Indices::U32(indices)));
        }
        // self.mesh.set_indices(Some(Indices::U32(indices)));

        // &self.mesh

        //self.mesh_handle.0.
        self.mesh_handle.clone()
    }

    pub fn add_health(&mut self, meshes: &mut ResMut<Assets<Mesh>>, val: f32) -> Mesh2dHandle {
        self.health += val;
        self.health.clamp(0., self.max_health);

        self.regenerate_mesh(meshes)
    }
}
