use nostr_minions::relay_pool::{NostrProps, RelayProvider, UserRelay};
use yew::prelude::*;
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let relays = vec![
        UserRelay {
            url: "wss://relay.illuminodes.com".to_string(),
            read: true,
            write: true,
        },
        UserRelay {
            url: "wss://relay.arrakis.lat".to_string(),
            read: true,
            write: true,
        },
    ];

    html! {
        <>
        <RelayProvider
            {relays} >
            <NostrComponent />
        </RelayProvider>
        </>
    }
}

#[function_component(NostrComponent)]
pub fn app() -> Html {
    let relay_ctx = use_context::<NostrProps>().expect("NostrProps not found");
    let subscriber = relay_ctx.subscribe.clone();
    let relay_notes = relay_ctx.unique_notes.clone();

    use_effect_with((), move |_| {
        let filter = nostro2::relays::NostrSubscription {
            // this defines the KIND of context we have!!
            kinds: Some(vec![10533]),
            limit: Some(10),
            ..Default::default()
        };
        subscriber.emit(filter.into());
        || {}
    });

    use_effect_with(relay_notes, move |notes| {
        if let Some(notes) = notes.last() {
            gloo::console::log!("Notes: {:?}", &notes.to_string());
            if notes.kind == 10533 {
                gloo::console::log!("Notes: {:?}", &notes.to_string());
            }
        }
        || {}
    });
    html! {
    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        {"Button"}
    </button>
    }
}
