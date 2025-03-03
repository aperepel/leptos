use leptos::*;

#[component]
pub fn TestComponent(
    _cx: Scope,
    /// Rust code
    /// ```
    /// assert_eq!("hello", stringify!(hello));
    /// ```
    /// View containing rust code
    /// ```view
    /// assert!(true);
    /// ```
    /// View containing rsx
    /// ```view
    /// # use example::TestComponent;
    /// <TestComponent key="hello"/>
    /// ```
    /// View containing rsx
    /// ```view compile_fail
    /// # use example::TestComponent;
    /// <TestComponent/>
    /// ```
    #[prop(into)]
    key: String,
    /// rsx unclosed
    /// ```view
    /// # use example::TestComponent;
    /// <TestComponent key="hello"/>
    #[prop(optional)]
    another: usize,
    /// rust unclosed
    /// ```view
    /// use example::TestComponent;
    #[prop(optional)]
    and_another: usize,
) -> impl IntoView {
    _ = (key, another, and_another);
}

#[component]
fn TestMutCallback<'a, F>(
    cx: Scope,
    mut callback: F,
    value: &'a str,
) -> impl IntoView
where
    F: FnMut(u32) + 'static,
{
    let value = value.to_owned();
    view! { cx,
        <button on:click=move |_| {
            callback(5);
        }>
            {value}
        </button>
        <TestComponent key="test"/>
    }
}
