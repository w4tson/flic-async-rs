use crate::hue::HueApi;
use crate::flic::events::stream_mapper::EventResult;
use crate::flic::events::Event;
use crate::flic::enums::ClickType;
use rand::{Rng, thread_rng};


pub struct LightController {
    hue_api: HueApi
} 

impl LightController {
    
    pub async fn new(username: &String) -> LightController {
        let hue_api = HueApi::with_user(username).await;
        LightController { hue_api }
    }
    
    pub fn list_all(&self) {
        self.hue_api.list_lights();
    }
    
    pub fn toggle_kitchen(&self) {
        self.hue_api.toggle_light(6).expect("toggling globe");
    }
    
    fn toggle_all(&self) {
        let lights = self.hue_api.get_all_lights().expect("problem getting the lights");
        let any_on = lights
            .iter()
            .any(|l| l.light.state.on);
        
        if any_on {
            // self.state.remove_all_state();
            lights.iter()
                .filter(|&l| l.light.state.on)
                .for_each(|l| {
                    // self.state.add_light(l);
                    self.hue_api.toggle_light(l.id);
                })
            //for all lights that are on
            //save to db
            //turn off
        } else {
            //for all lights in the db
            //turn on
            lights.iter()
                .for_each(|l| {
                    self.hue_api.toggle_light(l.id);
                })
                
        }
        
        
    }

    pub async fn process_event_result(&self, event_result: EventResult) {
        // println!("got this far");
        if let EventResult::Some(event) = event_result {
            match event {
                Event::ButtonSingleOrDoubleClickOrHold {conn_id:_, click_type: ClickType::ButtonDoubleClick, was_queued:_, time_diff} if time_diff < 5=> self.on_doubbeclick(event).await,
                Event::ButtonSingleOrDoubleClickOrHold {conn_id:_, click_type: ClickType::ButtonSingleClick, was_queued:_, time_diff} if time_diff < 5=> self.on_click(event).await,
                Event::ButtonSingleOrDoubleClickOrHold {conn_id:_, click_type: ClickType::ButtonHold, was_queued:_, time_diff} if time_diff < 5=> self.on_hold(event).await,
                _ => {
                    //eprintln!("event = {:#?}", event);
                }
            }
        }
    }

    async fn on_click(&self, event : Event) {
        eprintln!("clicked = {:#?}", event);
        self.toggle_all();
    }

    async fn on_doubbeclick(&self, event : Event) {
        eprintln!("DOUBLE = {:#?}", event);
        let mut rng = thread_rng();
        let x: f32 = rng.gen_range(0.0, 0.8);
        let y: f32 = rng.gen_range(0.0, 0.9);
        
        self.hue_api.set_color(6, x, y);
        
    }

    async fn on_hold(&self, event : Event) {
        eprintln!("HOLD = {:#?}", event);
    }

}
