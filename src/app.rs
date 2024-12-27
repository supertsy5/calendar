use std::{ops::Deref, rc::Rc};

use chrono::{Datelike, Local, Month, NaiveDate, Weekday};
use nongli::{
    calendar::{Calendar, Options},
    is_weekend,
    iter::{Months, Weekdays},
    language::{Language, ShortTranslate, Translate},
};
use yew::prelude::*;

use crate::form::{CheckboxInput, Form, IntInput, Select, SelectOption, StringInput};

const LANGUAGES: [Language; 3] = [
    Language::English,
    Language::ChineseSimplified,
    Language::ChineseTraditional,
];

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

    let color_text = use_state_eq(|| Rc::<str>::from("#111111"));
    let color_theme = use_state_eq(|| Rc::<str>::from("#0000ff"));
    let color_title = use_state_eq(|| Rc::<str>::from("#808080"));
    let color_today_text = use_state_eq(|| Rc::<str>::from("#ffffff"));
    let color_weekend = use_state_eq(|| Rc::<str>::from("#ff0000"));
    let color_solar_term = use_state_eq(|| Rc::<str>::from("#008000"));

    let language_index = use_state_eq(|| 0u32);
    let enable_chinese = use_state_eq(|| false);
    let start_on_monday = use_state_eq(|| false);
    let highlight_today = use_state_eq(|| true);

    let language = LANGUAGES
        .get(*language_index as usize)
        .copied()
        .unwrap_or(Language::English);
    let options = Options {
        language,
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

    let color_text_setter = color_text.setter();
    let color_theme_setter = color_theme.setter();
    let color_title_setter = color_title.setter();
    let color_today_text_setter = color_today_text.setter();
    let color_weekend_setter = color_weekend.setter();
    let color_solar_term_setter = color_solar_term.setter();

    let language_setter = language_index.setter();
    let enable_chinese_setter = enable_chinese.setter();
    let start_on_monday_setter = start_on_monday.setter();
    let highlight_today_setter = highlight_today.setter();

    html! { <>
        <style>{
            format!("
            :root {{
                --color-text: {color_text};
                --color-theme: {color_theme};
                --color-title: {color_title};
                --color-today-text: {color_today_text};
                --color-weekend: {color_weekend};
                --color-solar-term: {color_solar_term};
                --size-cell-height: 96px;
                --size-cell-width: 96px;
                --size-header-height: 96px;
                --size-text: 24px;
                --size-text-title: 24px;
                --size-text-year: 32px;
                --size-year-padding: 32px;
            }}",
            color_text = color_text.deref(),
            color_theme = color_theme.deref(),
            color_title = color_title.deref(),
            color_today_text = color_today_text.deref(),
            color_weekend = color_weekend.deref(),
            color_solar_term = color_solar_term.deref(),
        )
        }</style>
        <main>
            <div class="header">
                <div class="side left">{ year_month.month.name() }</div>
                <div class="year">{ year_month.year }</div>
                <div class="side right">
                    if language != Language::English {
                        { year_month.month.translate_to_string(language) }
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
                                    { weekday.short().translate_to_string(language) }
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
                                                        solar_term.translate_to_string(language)
                                                    }</div>
                                                } else if let Some(chinese) = cell.chinese_date {
                                                    <div class="chinese">{
                                                        chinese
                                                            .short()
                                                            .translate_to_string(language)
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
                        <Form>
                            <IntInput
                                name="Year"
                                min={ Some(-262143) }
                                max={ Some(262142) }
                                value={ year_month.year }
                                onchange={ move |value| {
                                    year_month_dispatcher3
                                        .dispatch(YearMonthAction::SetYear(value));
                            } }/>
                            <Select
                                name="Month"
                                value={ year_month.month as u32 }
                                onchange={move |value| {
                                    if let Ok(month) = Month::try_from(value as u8 + 1) {
                                        year_month_dispatcher4.dispatch(
                                            YearMonthAction::SetMonth(month)
                                        );
                                    }
                                }}
                            >
                            {
                                for Months(Month::January).take(12).map(|month| html_nested! {
                                    <SelectOption>
                                        { month.translate_to_string(language) }
                                    </SelectOption>
                                })
                            }
                            </Select>
                        </Form>
                    </div> },
                    Dialog::Styles => html! {<div class="dialog">
                        <div class="title">{"Styles"}</div>
                        <Form>
                            <StringInput
                                name="Text Color"
                                value={ color_text.deref().clone() }
                                onchange={ move |value| color_text_setter.set(Rc::from(value)) }
                            />
                            <StringInput
                                name="Theme Color"
                                value={ color_theme.deref().clone() }
                                onchange={ move |value| color_theme_setter.set(Rc::from(value)) }
                            />
                            <StringInput
                                name="Title Color"
                                value={ color_title.deref().clone() }
                                onchange={ move |value| color_title_setter.set(Rc::from(value)) }
                            />
                            <StringInput
                                name="Today Text Color"
                                value={ color_today_text.deref().clone() }
                                onchange={move |value| {
                                    color_today_text_setter.set(Rc::from(value))
                                } }
                            />
                            <StringInput
                                name="Weekend Color"
                                value={ color_weekend.deref().clone() }
                                onchange={ move |value| color_weekend_setter.set(Rc::from(value)) }
                            />
                            <StringInput
                                name="Solar Term Color"
                                value={ color_solar_term.deref().clone() }
                                onchange={ move |value| {
                                    color_solar_term_setter.set(Rc::from(value))
                                } }
                            />
                        </Form>
                    </div> },
                    Dialog::Settings => html! { <div class="dialog">
                        <div class="title">{"Settings"}</div>
                        <Form>
                            <Select
                                name="Language"
                                value={ *language_index }
                                onchange={ move |value| language_setter.set(value) }
                            >
                                <SelectOption>{"English"}</SelectOption>
                                <SelectOption>{"简体中文"}</SelectOption>
                                <SelectOption>{"繁體中文"}</SelectOption>
                            </Select>
                            <CheckboxInput
                                name="Enable Chinese"
                                checked={ *enable_chinese }
                                onchange={ move |checked| enable_chinese_setter.set(checked) }
                            />
                            <CheckboxInput
                                name="Start on Monday"
                                checked={ *start_on_monday }
                                onchange={ move |checked| start_on_monday_setter.set(checked) }
                            />
                            <CheckboxInput
                                name="Highlight Today"
                                checked={ *highlight_today }
                                onchange={ move |checked| highlight_today_setter.set(checked) }
                            />
                        </Form>
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
