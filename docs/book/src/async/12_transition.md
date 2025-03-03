# `<Transition/>`

You’ll notice in the `<Suspense/>` example that if you keep reloading the data, it keeps flickering back to `"Loading..."`. Sometimes this is fine. For other times, there’s [`<Transition/>`](https://docs.rs/leptos/latest/leptos/fn.Suspense.html).

`<Transition/>` behaves exactly the same as `<Suspense/>`, but instead of falling back every time, it only shows the fallback the first time. On all subsequent loads, it continues showing the old data until the new data are ready. This can be really handy to prevent the flickering effect, and to allow users to continue interacting with your application.

This example shows how you can create a simple tabbed contact list with `<Transition/>`. When you select a new tab, it continues showing the current contact until the new data loads. This can be a much better user experience than constantly falling back to a loading message.

[Click to open CodeSandbox.](https://codesandbox.io/p/sandbox/12-transition-sn38sd?selection=%5B%7B%22endColumn%22%3A15%2C%22endLineNumber%22%3A2%2C%22startColumn%22%3A15%2C%22startLineNumber%22%3A2%7D%5D&file=%2Fsrc%2Fmain.rs)

<iframe src="https://codesandbox.io/p/sandbox/12-transition-sn38sd?selection=%5B%7B%22endColumn%22%3A15%2C%22endLineNumber%22%3A2%2C%22startColumn%22%3A15%2C%22startLineNumber%22%3A2%7D%5D&file=%2Fsrc%2Fmain.rs" width="100%" height="1000px" style="max-height: 100vh"></iframe>

<details>
<summary>CodeSandbox Source</summary>

```rust
use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (tab, set_tab) = create_signal(cx, 0);

    // this will reload every time `tab` changes
    let user_data = create_resource(cx, tab, |tab| async move { important_api_call(tab).await });

    view! { cx,
        <div class="buttons">
            <button
                on:click=move |_| set_tab(0)
                class:selected=move || tab() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab(1)
                class:selected=move || tab() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab(2)
                class:selected=move || tab() == 2
            >
                "Tab C"
            </button>
            {move || if user_data.loading().get() {
                "Loading..."
            } else {
                ""
            }}
        </div>
        <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            <p>
                {move || user_data.read(cx)}
            </p>
        </Transition>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

```

</details>
</preview>
