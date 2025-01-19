use serde::Serialize;


#[derive(Serialize)]
pub struct LoadResult {
    pub version: String
}

#[inline(always)]
pub fn load_inner(version: String) -> LoadResult {


    LoadResult {
        version
    }
}