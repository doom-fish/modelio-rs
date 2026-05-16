use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)?;
    let submesh = mesh.submesh(0).expect("submesh");
    let material = Material::new("SubmeshMaterial", true)?;
    submesh.set_material(Some(&material));

    println!("submesh_name={:?}", submesh.name());
    println!("index_count={}", submesh.index_count());
    println!("has_material={}", submesh.material().is_some());
    println!("✅ modelio submesh OK");
    Ok(())
}
