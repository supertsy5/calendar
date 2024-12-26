use std::{rc::Rc, str::FromStr};

use chrono::{Datelike, Local, Month, NaiveDate, Weekday};
use nongli::{
    calendar::{Calendar, Options},
    is_weekend,
    iter::{Months, Weekdays},
    language::{Language, ShortTranslate, Translate},
};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct YearMonth {
    year: i32,
    month: Month,
}

enum YearMonthAction {
    NextMonth,
    PrevMonth,
    Today,
    Set(i32, Month),
    SetYear(i32),
    SetMonth(Month),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dialog {
    Jump,
    Styles,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ActiveDialog(Option<Dialog>);

impl Reducible for YearMonth {
    type Action = YearMonthAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        use YearMonthAction::*;
        match action {
            NextMonth => {
                if self.month == Month::December {
                    if let Some(next_year) = self
                        .year
                        .checked_add(1)
                        .filter(|next_year| NaiveDate::from_ymd_opt(*next_year, 1, 1).is_some())
                    {
                        Rc::new(Self {
                            year: next_year,
                            month: Month::January,
                        })
                    } else {
                        self
                    }
                } else {
                    Rc::new(Self {
                        year: self.year,
                        month: self.month.succ(),
                    })
                }
            }
            PrevMonth => {
                if self.month == Month::January {
                    if let Some(prev_year) = self
                        .year
                        .checked_sub(1)
                        .filter(|prev_year| NaiveDate::from_ymd_opt(*prev_year, 1, 1).is_some())
                    {
                        Rc::new(Self {
                            year: prev_year,
                            month: Month::December,
                        })
                    } else {
                        self
                    }
                } else {
                    Rc::new(Self {
                        year: self.year,
                        month: self.month.pred(),
                    })
                }
            }
            Today => {
                let today = Local::now().date_naive();
                Rc::new(Self {
                    year: today.year(),
                    month: Month::try_from(today.month() as u8).unwrap(),
                })
            }
            Set(year, month) => Rc::new(Self { year, month }),
            SetYear(year) => {
                if NaiveDate::from_ymd_opt(year, 1, 1).is_some() {
                    Rc::new(Self {
                        year,
                        month: self.month,
                    })
                } else {
                    self
                }
            }
            SetMonth(month) => Rc::new(Self {
                year: self.year,
                month,
            }),
        }
    }
}

impl Reducible for ActiveDialog {
    type Action = Dialog;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        if self.0 == Some(action) {
            Rc::new(Self(None))
        } else {
            Rc::new(Self(Some(action)))
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let today = chrono::Local::now().date_naive();

    let year_month = use_reducer_eq(|| YearMonth {
        year: today.year(),
        month: Month::try_from(today.month() as u8).unwrap(),
    });
    let active_dialog = use_reducer_eq(|| ActiveDialog(None));

    let language = use_state_eq(|| Language::English);
    let enable_chinese = use_state_eq(|| false);
    let start_on_monday = use_state_eq(|| false);
    let highlight_today = use_state_eq(|| true);
    let options = Options {
        language: *language,
        enable_chinese: *enable_chinese,
        start_on_monday: *start_on_monday,
        color: false,
    };

    let calendar = use_memo(
        (today, *year_month, options, *highlight_today),
        |(today, year_month, options, highlight_today)| {
            Calendar::new(
                year_month.year,
                year_month.month,
                highlight_today.then_some(*today),
                *options,
            )
            .unwrap()
        },
    );

    let active_dialog_value = active_dialog.0;

    let year_month_dispatcher = year_month.dispatcher();
    let year_month_dispatcher1 = year_month.dispatcher();
    let year_month_dispatcher2 = year_month.dispatcher();
    let year_month_dispatcher3 = year_month.dispatcher();
    let year_month_dispatcher4 = year_month.dispatcher();
    let active_dialog_dispatcher = active_dialog.dispatcher();
    let active_dialog_dispatcher1 = active_dialog.dispatcher();
    let active_dialog_dispatcher2 = active_dialog.dispatcher();

    let language_setter = language.setter();
    let enable_chinese_setter = enable_chinese.setter();
    let start_on_monday_setter = start_on_monday.setter();
    let highlight_today_setter = highlight_today.setter();

    html! { <>
        <main>
            <div class="header">
                <div class="side left">{ year_month.month.name() }</div>
                <div class="year">{ year_month.year }</div>
                <div class="side right">
                    if *language != Language::English {
                        { year_month.month.translate_to_string(*language) }
                    }
                </div>
            </div>
            <div class="body">
                <table class="calendar">
                    <tr>{
                        for Weekdays(if *start_on_monday { Weekday::Mon } else { Weekday::Sun })
                            .take(7)
                            .map(|weekday| html! {
                                <th class={classes!(is_weekend(weekday).then_some("weekend"))}>
                                    { weekday.short().translate_to_string(*language) }
                                </th>
                            })
                    }</tr>
                    {
                        for calendar.iter().map(|row| html! {
                            <tr>
                                {
                                    for row.map(|cell| html! {
                                        if let Some(cell) = cell {
                                            <td class={ classes!(
                                                cell.today.then_some("today"),
                                                cell.weekend.then_some("weekend"),
                                            ) }>
                                                <div class="day">{ cell.date.day() }</div>
                                                if let Some(solar_term) = cell.solar_term {
                                                    <div class="chinese solar-term">{
                                                        solar_term.translate_to_string(*language)
                                                    }</div>
                                                } else if let Some(chinese) = cell.chinese_date {
                                                    <div class="chinese">{
                                                        chinese
                                                            .short()
                                                            .translate_to_string(*language)
                                                    }</div>
                                                }
                                            </td>
                                        } else {
                                            <td></td>
                                        }
                                    })
                                }
                            </tr>
                        })
                    }
                </table>
            </div>
        </main>
        <div class="corner">
            if let Some(dialog) = active_dialog_value {{
                match dialog {
                    Dialog::Jump => html! { <div class="dialog">
                        <div class="title">{"Jump"}</div>
                        <table>
                            <tr>
                                <td>{"Year"}</td>
                                <td>
                                    <input
                                        type="number"
                                        value={ year_month.year.to_string() }
                                        min="-262143"
                                        max="262142"
                                        onchange={ move |event: Event| {
                                            let Some(element) = event.target().and_then(|target| {
                                                target.dyn_into::<HtmlInputElement>().ok()
                                            }) else {
                                                return;
                                            };
                                            let Ok(year) = i32::from_str(&element.value()) else {
                                                return;
                                            };
                                            year_month_dispatcher3.dispatch(
                                                YearMonthAction::SetYear(year)
                                            );
                                        }}
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td>{"Month"}</td>
                                <td>
                                    <select
                                        onchange={ move |event: Event| {
                                            let Some(element) = event.target().and_then(|target| {
                                                target.dyn_into::<HtmlSelectElement>().ok()
                                            }) else {
                                                return;
                                            };
                                            year_month_dispatcher4.dispatch(
                                                YearMonthAction::SetMonth(
                                                    Month::try_from(
                                                        element.selected_index() as u8 + 1
                                                    ).unwrap()
                                                )
                                            );
                                        } }
                                    >{
                                        for Months(Month::January).take(12).map(|month| html! {
                                            <option selected={ month == year_month.month }>
                                                { month.translate_to_string(*language) }
                                            </option>
                                        })
                                    }</select>
                                </td>
                            </tr>
                        </table>
                    </div> },
                    Dialog::Styles => html! {<div class="dialog">
                        <div class="title">{"Styles"}</div>
                        <table></table>
                    </div> },
                    Dialog::Settings => html! { <div class="dialog">
                        <div class="title">{"Settings"}</div>
                        <table>
                            <tr>
                                <td>{"Language"}</td>
                                <td>
                                    <select
                                        onchange={ move |event: Event| {
                                            use Language::*;
                                            let Some(element) = event
                                                .target()
                                                .and_then(|target| {
                                                    target.dyn_into::<HtmlSelectElement>().ok()
                                                })
                                            else {
                                                return;
                                            };
                                            language_setter.set(
                                                [
                                                    English,
                                                    ChineseSimplified,
                                                    ChineseTraditional,
                                                ][element.selected_index() as usize]
                                            );
                                        }}
                                    >
                                        <option selected={ *language == Language::English }>
                                            {"English"}
                                        </option>
                                        <option
                                            selected={ *language == Language::ChineseSimplified }
                                        >
                                            {"简体中文"}
                                        </option>
                                        <option
                                            selected={ *language == Language::ChineseTraditional }
                                        >
                                            {"繁體中文"}
                                        </option>
                                    </select>
                                </td>
                            </tr>
                            <tr>
                                <td>{"Enable Chinese Caldendar"}</td>
                                <td>
                                    <input
                                        type="checkbox"
                                        checked={ *enable_chinese }
                                        onchange={ move |event: Event| {
                                            let Some(element) = event
                                                .target()
                                                .and_then(|target| {
                                                    target.dyn_into::<HtmlInputElement>().ok()
                                                })
                                            else {
                                                return;
                                            };
                                            enable_chinese_setter.set(element.checked());
                                        } }
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td>{"Start on Monday"}</td>
                                <td>
                                    <input
                                        type="checkbox"
                                        checked={ *start_on_monday }
                                        onchange={ move |event: Event| {
                                            let Some(element) = event
                                                .target()
                                                .and_then(|target| {
                                                    target.dyn_into::<HtmlInputElement>().ok()
                                                })
                                            else {
                                                return;
                                            };
                                            start_on_monday_setter.set(element.checked());
                                        } }
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td>{"Highlight Today"}</td>
                                <td>
                                    <input
                                        type="checkbox"
                                        checked={ *highlight_today }
                                        onchange={ move |event: Event| {
                                            let Some(element) = event
                                                .target()
                                                .and_then(|target| {
                                                    target.dyn_into::<HtmlInputElement>().ok()
                                                })
                                            else {
                                                return;
                                            };
                                            highlight_today_setter.set(element.checked());
                                        } }
                                    />
                                </td>
                            </tr>
                        </table>
                    </div> }
                } }
            }
        <div class="corner-buttons">
            <button
                title="Previous Month"
                class="material-symbols-outlined"
                disabled={
                    year_month.month == Month::January
                    && NaiveDate::from_ymd_opt(year_month.year - 1, 1, 1).is_none()
                }
                onclick={ move |_| year_month_dispatcher.dispatch(YearMonthAction::PrevMonth) }
            >
                {"arrow_back"}
            </button>
            <button
                title="Next Month"
                class="material-symbols-outlined"
                disabled={
                    year_month.month == Month::December
                    && NaiveDate::from_ymd_opt(year_month.year + 1, 1, 1).is_none()
                }
                onclick={ move |_| year_month_dispatcher1.dispatch(YearMonthAction::NextMonth) }
            >
                {"arrow_forward"}
            </button>
            <button
                title="Today"
                class="material-symbols-outlined"
                onclick={ move |_| year_month_dispatcher2.dispatch(YearMonthAction::Today) }
            >
                {"today"}
            </button>
            <button
                title="Jump"
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Jump)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher.dispatch(Dialog::Jump) }
            >
                {"calendar_month"}
            </button>
            <button
                title="Print"
                class="material-symbols-outlined"
                onclick={ |_| {
                    if let Some(window) = web_sys::window() {
                        let _ = window.print();
                    }
                } }
            >
                {"print"}
            </button>
            <button
                title="Styles"
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Styles)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher1.dispatch(Dialog::Styles) }
            >
                {"style"}
            </button>
            <button
                title="Settings"
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Settings)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher2.dispatch(Dialog::Settings) }
            >
                {"settings"}
            </button>
            </div>
        </div>
    </>}
}
