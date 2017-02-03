use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct EmoticonSets
{
    pub emoticon_sets: HashMap<u64, Vec<Emoticon>>,
}

#[derive(Deserialize, Debug)]
pub struct Emoticon
{
    pub id: u64,
    pub code: String,
}
