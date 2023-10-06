pub mod array_material;

use bevy::asset::load_internal_asset;
use bevy::pbr::{MESH_SHADER_HANDLE, MESH_VERTEX_OUTPUT, PBR_BINDINGS_SHADER_HANDLE, PBR_SHADER_HANDLE, PREPASS_SHADER_HANDLE};
use bevy::prelude::{AddAsset, App, Deref, HandleUntyped, MaterialPlugin, Plugin, Shader};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;
use crate::systems::core_system::render_system::array_material::array_material::ArrayMaterial;


pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute = MeshVertexAttribute::new("ArrayIndex", 983245917, VertexFormat::Uint32);

pub struct ArrayMaterialSystemPlugin;
impl Plugin for ArrayMaterialSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ArrayMaterial>::default());
        app.add_asset::<ArrayMaterial>();

        load_internal_asset!(app, PREPASS_SHADER_HANDLE, "./array_material/custom_prepass.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, MESH_SHADER_HANDLE, "./array_material/custom_mesh.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, PBR_SHADER_HANDLE, "./array_material/custom_pbr_prepass.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, PBR_BINDINGS_SHADER_HANDLE, "./array_material/custom_pbr_bindings.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, MESH_VERTEX_OUTPUT, "./array_material/custom_vertex_output.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, PBR_SHADER_HANDLE, "./array_material/custom_pbr_fragment.wgsl", Shader::from_wgsl);
    }
}
