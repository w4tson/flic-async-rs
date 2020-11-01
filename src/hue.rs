use tokio::task;
use hueclient::bridge::Bridge;
use anyhow::Result;


pub struct HueApi {
    bridge : Bridge
}

impl HueApi {
    
    pub async fn with_user(user: &String) -> HueApi {
        let username = user.clone().into();
        let bridge = task::spawn_blocking(move || {
            Bridge::discover().unwrap().with_user(username)
        }).await.expect("Couldn't find bridge");
        
        HueApi { bridge }
    }

    pub fn toggle_light(&self, id: usize) -> Result<bool> {
        let lights = self.bridge.get_all_lights().expect("failed to get the lights");
        let light = lights.iter().find(|&light| light.id == id).expect(&format!("No light with id {}", id));
        let result = if light.light.state.on {   
            self.bridge.set_light_state(id, &hueclient::bridge::CommandLight::default().off())?
        } else {
            self.bridge.set_light_state(id, &hueclient::bridge::CommandLight::default().on())?
        };

        eprintln!("result = {:#?}", result);
        
        Ok(true)
    }

    
    pub fn list_lights(&self) {
        match self.bridge.get_all_lights() {
            Ok(lights) => {
                println!("id name                 on    bri   hue sat temp  x      y");
                for ref l in lights.iter() {
                    println!(
                        "{:2} {:20} {:5} {:3} {:5} {:3} {:4}K {:4} {:4}",
                        l.id,
                        l.light.name,
                        if l.light.state.on { "on" } else { "off" },
                        if l.light.state.bri.is_some() {l.light.state.bri.unwrap()} else {0},
                        if l.light.state.hue.is_some() {l.light.state.hue.unwrap()} else {0},
                        if l.light.state.sat.is_some() {l.light.state.sat.unwrap()} else {0},
                        if l.light.state.ct.is_some() {l.light.state.ct.map(|k| if k != 0 { 1000000u32 / (k as u32) } else { 0 }).unwrap()} else {0},
                        if l.light.state.xy.is_some() {l.light.state.xy.unwrap().0} else {0.0},
                        if l.light.state.xy.is_some() {l.light.state.xy.unwrap().1} else {0.0},
                    );
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                ::std::process::exit(2)
            }
        }
    }
} 