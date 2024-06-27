use akimo_runtime::{
    game::{Element, ElementRegistration, WorldChange},
    hashbrown::HashMap,
    log::{debug, info},
    ulid::Ulid,
    variant::Variant,
};

#[derive(Debug, Default)]
pub struct TestElement {
    own_id: Option<Ulid>,
}

impl Element for TestElement {
    fn on_registration(&mut self, ulid: &Ulid) -> ElementRegistration {
        self.own_id = Some(*ulid);

        ElementRegistration::default()
    }

    fn on_update(&mut self, delta_time: f64) -> Option<Vec<WorldChange>> {
        debug!("Delta: {}ms", delta_time);

        // Where this message should be send to. In this case ourself.
        let target_id = self.own_id.unwrap();
        // The message itself, just a simple String
        let mut message = HashMap::new();
        message.insert("msg".into(), Variant::String("Hello, World!".into()));
        // Queue the message for sending
        Some(vec![WorldChange::SendMessage(target_id, message)])
    }

    fn on_message(&mut self, message: HashMap<String, akimo_runtime::variant::Variant>) {
        info!("On Message: {:#?}", message);
    }
}
