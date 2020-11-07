use hueclient::bridge::IdentifiedLight;

pub struct Light {
    pub id: usize,
    pub name: String,
    pub on: bool
}

pub struct State {
    pub lights: Vec<Light>
}

impl State {
    pub fn new() -> State {
        State{ lights: vec![] }
    }
    
    pub fn save(&mut self, light : &IdentifiedLight) {
        let l = Light {
            id: light.id,
            name: light.light.name.clone(),
            on: light.light.state.on
        };
        self.lights.push(l);
    }
    
    pub fn clear(&mut self)
    {
        self.lights.clear();
    }
}

