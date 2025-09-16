use yew::prelude::*;
use ycl::elements::youtube_video::BBYouTubeVideo;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Minimal YouTube Video Player Example"}</h1>
            <BBYouTubeVideo
                src="https://www.youtube.com/embed/jdNp4UPo4RM"
                title="Example Video"
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
