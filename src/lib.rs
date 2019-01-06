use azul::app_state::AppStateNoData;
use azul::default_callbacks::{DefaultCallback, DefaultCallbackId, StackCheckedPointer};
use azul::prelude::*;
use azul::window::{FakeWindow, WindowEvent};

pub const CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/style.css"
));

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dropdown {
    on_input_callback: Option<DefaultCallbackId>,
}

pub struct DropdownState {
    pub open: bool,
    pub unselected_label: Option<String>,
    pub selections: Vec<String>,
    pub selected: Option<String>,
}

impl Default for DropdownState {
    fn default() -> Self {
        DropdownState {
            open: false,
            selections: vec![],
            selected: None,
            unselected_label: None,
        }
    }
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            on_input_callback: None,
        }
    }

    pub fn bind<T: Layout>(
        self,
        window: &mut FakeWindow<T>,
        field: &DropdownState,
        data: &T,
    ) -> Self {
        let ptr = StackCheckedPointer::new(data, field);
        let on_input_callback = ptr.and_then(|ptr| {
            Some(window.add_callback(ptr, DefaultCallback(DropdownState::on_input_private)))
        });
        Self { on_input_callback }
    }

    pub fn dom<T: Layout>(&self, state: &DropdownState) -> Dom<T> {
        let parent_div = Dom::new(NodeType::Div).with_class("__dropdown_parent");

        let with_label = |parent: Dom<T>, label: String| {
            parent.with_child({
                let mut node = Dom::new(NodeType::Label(label)).with_class("__dropdown_node");
                if let Some(on_input_callback) = self.on_input_callback {
                    node.add_default_callback_id(On::MouseUp, on_input_callback);
                }
                node
            })
        };

        if state.open {
            state
                .selections
                .iter()
                .cloned()
                .map(|s| NodeData {
                    node_type: NodeType::Label(s),
                    default_callback_ids: self
                        .on_input_callback
                        .map_or(vec![], |cb| vec![(On::MouseUp, cb)]),
                    classes: vec!["__dropdown_node".into()],
                    ..Default::default()
                })
                .collect()
        } else {
            with_label(
                parent_div,
                state
                    .selected
                    .clone()
                    .or_else(|| state.unselected_label.clone())
                    .unwrap_or_default(),
            )
        }
    }
}

impl DropdownState {
    fn on_input_private<T: Layout>(
        data: &StackCheckedPointer<T>,
        app_state_no_data: AppStateNoData<T>,
        window_event: WindowEvent<T>,
    ) -> UpdateScreen {
        unsafe { data.invoke_mut(Self::on_input, app_state_no_data, window_event) }
    }

    pub fn on_input<T: Layout>(
        &mut self,
        _app_state_no_data: AppStateNoData<T>,
        event: WindowEvent<T>,
    ) -> UpdateScreen {
        if self.open {
            if let Some(selected_idx) = event.index_path_iter().next() {
                self.selected = Some(self.selections[selected_idx].clone())
            }
        }
        self.open = !self.open;
        UpdateScreen::Redraw
    }
}
