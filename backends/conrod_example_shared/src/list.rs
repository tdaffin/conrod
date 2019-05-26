use conrod_core::{
    Colorable,
    Widget,
    Labelable,
    Positionable,
    Sizeable,
    Ui,
    UiCell,
    widget,
};


widget_ids! {
    pub struct Ids {
        list,
    }
}

pub struct GuiState {
    pub list: Vec<bool>,
}

pub struct Gui {
    ids: Ids,
}

impl Gui {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    pub fn update(&self, ui: &mut UiCell, app: &mut GuiState){
        let ids = &self.ids;

        let (mut items, scrollbar) = widget::List::flow_down(app.list.len())
            .down(10.0)
            .item_size(50.0)
            .scrollbar_on_top()
            .w_h(250.0, 200.0)
            .set(ids.list, ui);

        while let Some(item) = items.next(ui) {
            let i = item.i;
            let label = format!("item {}: {}", i, app.list[i]);
            let toggle = widget::Toggle::new(app.list[i])
                .label(&label)
                .label_color(conrod_core::color::WHITE)
                .color(conrod_core::color::LIGHT_BLUE);
            for v in item.set(toggle, ui) {
                app.list[i] = v;
            }
        }

        if let Some(s) = scrollbar { s.set(ui) }
    }
}
