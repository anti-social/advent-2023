#![allow(non_snake_case)]
use std::collections::BTreeMap;

use dioxus::prelude::*;

use paste::paste;

macro_rules! days {
    ($($day:expr),*) => {
        paste! {
            $(mod [<day_ $day>];)*

            const DAYS: &'static [Day] = &[
                $(
                    Day::new($day, include_str!(concat!("day_", stringify!($day), ".rs")), [<day_ $day>]::solve_1, [<day_ $day>]::solve_2),
                )*
            ];
        }
    };
}

days!(01, 02, 03, 04);

struct Day {
    pub ord: u32,
    pub code: &'static str,
    pub solve1: fn(&str) -> String,
    pub solve2: fn(&str) -> String,
}

impl Day {
    const fn new(
        ord: u32,
        code: &'static str,
        solve1: fn(&str) -> String,
        solve2: fn(&str) -> String,
    ) -> Self {
        Self { ord, code, solve1, solve2 }
    }
}

struct Puzzle {
    pub code: &'static str,
    pub solve: fn(&str) -> String,
}

impl Puzzle {
    const fn new(
        code: &'static str,
        solve: fn(&str) -> String,
    ) -> Self {
        Self { code, solve }
    }
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            div {
                h1 {
                    class: "text-xl text-center",
                    "Advent of Code 2023 Solver"
                }
            }
            Solver {}
        }
    })
}

fn Solver(cx: Scope) -> Element {
    let src = use_state(cx, || DAYS.last().unwrap().code);
    let answer = use_state(cx, || None);

    let puzzles = DAYS.iter()
        .flat_map(|d| {
            [
                (format!("{:0>2}-1", d.ord), Puzzle::new(d.code, d.solve1)),
                (format!("{:0>2}-2", d.ord), Puzzle::new(d.code, d.solve2))
            ]
        })
        .collect::<BTreeMap<_, _>>();

    let tasks = vec!("01-1", "01-2", "02-1", "02-2", "03-1", "03-2", "04-1", "04-2");
    let tasks_len = tasks.len();

    cx.render(rsx!{
        form {
            onsubmit: move |event| {
                let input = &event.data.values["input"][0];
                let res = if let Some(puzzle) = &puzzles.get(event.data.values["task"][0].as_str()) {
                    (puzzle.solve)(input)
                } else {
                    "Invalid task".to_string()
                };
                answer.set(Some(res));
            },
            div {
                class: "py-2",
                legend {
                    class: "p-2",
                    "Choose task"
                }
                div {
                    class: "grid grid-flow-col auto-cols-min grid-rows-2 gap-2",
                    puzzles.iter().enumerate().map(|(i, (t, p))| rsx!{
                        div {
                            class: "py-2",
                            input {
                                id: "task-{t}",
                                r#type: "radio",
                                name: "task",
                                value: "{t}",
                                checked: i + 1 == tasks_len,
                                class: "hidden peer",
                            }
                            label {
                                r#for: "task-{t}",
                                class: "p-2 border rounded-lg cursor-pointer hover:text-gray-600 hover:bg-gray-100 peer-checked:border-blue-600 peer-checked:text-blue-600",
                                onclick: |event| {
                                    log::info!("Setting new source");
                                    src.set(p.code);
                                    event.stop_propagation();
                                },
                                "{t}"
                            }
                        }
                    })
                }
            }
            div {
                class: "grid grid-rows-1 grid-cols-2 gap-2",
                div {
                    class: "w-full",
                    div {
                        class: "py-2",
                        legend {
                            "Input data"
                        }
                        textarea {
                            name: "input",
                            placeholder: "Paster your input data",
                            class: "block p-2 w-full resize border bg-gray-50",
                            rows: "20",
                            // cols: "80"
                        }
                    }
                    div {
                        button {
                            class: "inline-flex justify-center px-4 py-1 text-white bg-blue-600 border border-blue-700 rounded-md",
                            "Solve it"
                        }
                        span { class: "pl-2", "Result is: " }
                        span { answer.as_ref().map(|a| rsx!{ "{a}" }) }
                    }
                }
                div {
                    class: "border p-2 w-full",
                    Source {
                        code: src.to_string()
                    }
                }
            }
        }
    })
}

#[inline_props]
fn Source(cx: Scope, code: String) -> Element {
    // let code = cx.props.code.clone();
    // log::info!("Got code: {}", &code);

    let create_eval = use_eval(cx);
    let hl_code = use_state(cx, String::new);

    let future = use_future!(cx, |code| {
        to_owned![create_eval];
        let hl_code = hl_code.clone();
        async move {
            log::info!("Evaluating JS code...");
            let eval = create_eval(
                r#"
                let code = await dioxus.recv();
                console.log(code);
                let hlCode = hljs.highlight(code, {"language": "rust"}).value;
                console.log(hlCode);
                dioxus.send(hlCode);
                "#,
            )
            .unwrap();
            eval.send(serde_json::Value::String(code.to_string())).unwrap();

            if let serde_json::Value::String(res) = eval.recv().await.unwrap() {
                hl_code.set(res);
            }
        }
    });
    future.value();
    cx.render(rsx!{
        pre {
            code {
                class: "language-rust",
                dangerous_inner_html: "{hl_code}"
            }
        }
    })
}
