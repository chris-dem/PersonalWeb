use component::root::Root;
mod component;
mod context;

fn main() {
    yew::Renderer::<Root>::new().render();
}
