use tera::Tera;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateSet {
    #[serde(skip)]
    tera: Option<Tera>,
}

impl Default for TemplateSet {
    fn default() -> Self {
        Self {
            tera: None
        }
    }
}

impl TemplateSet {
    pub fn load(path: &str) -> Result<TemplateSet, String> {
        let tera = Tera::new(path).map_err(|e| e.to_string())?;
        println!("created tera! {:?}", tera);
        return Ok(TemplateSet { tera: None })
    }
}
