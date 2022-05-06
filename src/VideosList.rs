use yew::prelude::*;

#[derive(Clone, PartialEq)]
public struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            html! {
                <p>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
