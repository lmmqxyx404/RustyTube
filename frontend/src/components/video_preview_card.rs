use config::Config;
use invidious::channel::Channel;
use invidious::common::CommonVideo;
use invidious::hidden::CountryCode;
use invidious::universal::{Trending, TrendingCategory, TrendingCategory::*};
use leptos::*;
use num_format::{Locale, ToFormattedString};
use rustytube_error::RustyTubeError;
use web_sys::ErrorEvent;

use crate::components::FerrisError;
use crate::icons::FerrisWtfIcon;

#[component]
pub fn VideoPreviewCard(cx: Scope, video: CommonVideo) -> impl IntoView {
	let thumbnail_url = video.thumbnails.first().expect("No thumbnail").url.clone();

	view! {cx,
        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden cursor-pointer">
            <VideoPreviewCardThumbnail url=thumbnail_url />
			<VideoPreviewCardInfo video=video />
        </div>
    }
}

#[component]
pub fn VideoPreviewCardInfo(cx: Scope, video: CommonVideo) -> impl IntoView {
	let view_count = video.views.to_formatted_string(&Locale::en);

	view! {cx,
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
            <h1 class=" font-sans font-semibold text-base line-clamp-2">{&video.title}</h1>
            <div class="flex flex-row flex-wrap font-normal text-sm gap-1">
                <h2 class="cursor-pointer text-primary">{&video.author}</h2>
                <p>{"•"}</p>
                <p>{view_count} {r#" views"#}</p>
                <p>{"•"}</p>
                <p>{&video.published_text}</p>
            </div>
        </div>
	}
}

#[derive(Clone)]
pub enum ThumbnailState {
    Loading,
    Success,
    Error(RustyTubeError)
}

#[component]
pub fn VideoPreviewCardThumbnail(cx: Scope, url: String) -> impl IntoView {
    use ThumbnailState::*;

    let (state, set_state) = create_signal(cx, Loading);

	view! {cx,
        <div class="w-full max-w-full overflow-hidden rounded-xl">
            {move ||
                match state.get() {
                    Loading => view! { cx, <VideoPreviewCardPlaceholder set_state=set_state url=url.clone() />}.into_view(cx),
                    Success => view! { cx, <VideoPreviewCardImage url=url.clone() /> }.into_view(cx),
                    Error(err) => view! { cx, <VideoPreviewCardThumbnailError error=err /> }.into_view(cx)
                }
            }
        </div>
	}
}

#[component]
pub fn VideoPreviewCardImage(cx: Scope, url: String) -> impl IntoView {
	view! {cx,
        <div class="aspect-w-16 aspect-h-9">
            <img src=url class="w-full h-full object-center object-cover bg-base-content rounded-xl"/>
        </div>
	}
}

#[component]
pub fn VideoPreviewCardPlaceholder(cx: Scope, set_state: WriteSignal<ThumbnailState>, url: String) -> impl IntoView {
    use ThumbnailState::*;

    let show_img = move |_| set_state.set(Success);
    let show_err = move |_| set_state.set(Error(RustyTubeError::fetch_thumbnail_error()));

	view! {cx,
		<div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
			<div class="animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl" />
		</div>
        <img 
            on:error=show_err
            on:load=show_img 
            src=url 
            class="hidden w-full h-full object-center object-cover bg-base-content rounded-xl"
        />
	}

}

#[component]
pub fn VideoPreviewCardThumbnailError(cx: Scope, error: RustyTubeError) -> impl IntoView {
	view! {cx,
        <div class="w-full h-full aspect-w-16 aspect-h-9">
            <div class="w-full h-full flex flex-col space-y-4 p-2 text-base-content">
                <div class="justify-self-center">
                    <FerrisWtfIcon width=32 />
                </div>
                <h1 class="w-fit font-semibold text-base">{error.title}</h1>
                <p class="w-fit font-normal text-sm font-mono">{error.description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn VideoPreviewCardPlaceholderArray(cx: Scope) -> impl IntoView {
	view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-1rem-128px)] overflow-y-scroll">
            {
                let mut i = 0;
                let mut vec = vec![];

                while i < 50 {
                    vec.push(view!
                        {cx,
                        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
                            <div class="animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl" />
                            <div class="flex flex-col w-full mt-3 space-y-3 px-2">
                                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                                <div class="animate-pulse w-[35%] h-2 rounded-xl bg-base-content"></div>
                            </div>
                        </div>
                        }
                );
                    i = i + 1;
                }
                vec.collect_view(cx)
            }
        </div>
    }
}