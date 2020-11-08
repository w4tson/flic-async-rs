use crate::hue::HueApi;
use crate::flic::events::stream_mapper::EventResult;
use crate::flic::events::Event;
use crate::flic::enums::ClickType;
use rand::{Rng, thread_rng};
use crate::state::State;


pub struct LightController {
    hue_api: HueApi,
    state: State
} 

impl LightController {
    
    pub async fn new(username: &String) -> LightController {
        let hue_api = HueApi::with_user(username).await;
        LightController { hue_api, state: State::new() }
    }
    
    pub fn list_all(&self) {
        self.hue_api.list_lights();
    }
    
    pub fn toggle_kitchen(&self) {
        self.hue_api.toggle_light(6).expect("toggling globe");
    }
    
    fn toggle_all(&mut self) {
        let lights = self.hue_api.get_all_lights().expect("problem getting the lights");
        let any_on = lights
            .iter()
            .any(|l| l.light.state.on);
        
        if any_on {
            self.state.clear();
            lights.iter()
                .for_each(|l| {
                    self.state.save(l);
                    if l.light.state.on {
                        self.hue_api.turn_off_light(l.id);
                    }
                });
        } else {
            self.state.lights.iter()
                .filter(|&l| l.on)
                .for_each(|l| {
                    self.hue_api.turn_on_light(l.id);
                });
        }
    }

    pub async fn process_event_result(&mut self, event_result: EventResult) {
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

    async fn on_click(&mut self, event : Event) {
        eprintln!("clicked = {:#?}", event);
        self.toggle_all();
    }

    async fn on_doubbeclick(&self, event : Event) {
        let light = hueclient::bridge::CommandLight::default()
        .on()
        .with_bri(254)
        .with_hue(8418)
        .with_sat(140)
        .with_xy(0.4573,
                      0.41);

        self.hue_api.set_group_state(9, &light).expect("nope");        
    }

    async fn on_hold(&self, event : Event) {
        eprintln!("HOLD = {:#?}", event);
    }

}
