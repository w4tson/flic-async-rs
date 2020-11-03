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
        self.toggle_kitchen();
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
