use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Run {
    pub igt: f64,
    pub place: usize,
}
