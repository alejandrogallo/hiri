

pub struct Index {
    pub name: String,
}

pub struct Operator {
    pub name: String,
    pub creation_indices: Vec<Index>,
    pub annihilation_indices: Vec<Index>,

}
