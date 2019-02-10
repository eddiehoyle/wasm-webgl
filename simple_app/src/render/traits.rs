pub trait Drawable {
    fn buffer_vao();
    fn buffer_indices_u32(gl: &GL, indices: &[u32]);
    fn buffer_data_f32(gl: &GL, indices: &[f32]);
}