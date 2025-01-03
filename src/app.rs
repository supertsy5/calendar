use std::{ops::Deref, rc::Rc};

use chrono::{Datelike, Local, Month, NaiveDate, Weekday};
use nongli::{
    calendar::{Calendar, Options},
    is_weekend,
    iter::{Months, Weekdays},
    language::{Language, ShortTranslate, StaticTranslate, Translate},
};
use yew::prelude::*;

use crate::{
    form::{CheckboxInput, ColorInput, Form, IntInput, Select, SelectOption, StringInput},
    translations,
};

const LANGUAGES: &[Language] = &[
    Language::English,
    Language::ChineseSimplified,
    Language::ChineseTraditional,
];

const GENERIC_FONTS: &[&str] = &[
    "cursive",
    "emoji",
    "fangsong",
    "fantasy",
    "inherit",
    "initial",
    "kai",
    "math",
    "monospace",
    "nastaliq",
    "revert",
    "revert-layer",
    "sans-serif",
    "serif",
    "system-ui",
    "ui-monospace",
    "ui-rounded",
    "ui-sans-serif",
    "ui-serif",
    "unset",
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
    #[allow(dead_code)]
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
    let color_year = use_state_eq(|| Rc::<str>::from("#808080"));
    let color_month = use_state_eq(|| Rc::<str>::from("#808080"));
    let color_today_text = use_state_eq(|| Rc::<str>::from("#ffffff"));
    let color_weekend = use_state_eq(|| Rc::<str>::from("#ff0000"));
    let color_week_number = use_state(|| Rc::<str>::from("#808080"));
    let color_festival = use_state(|| Rc::<str>::from("inherit"));
    let color_solar_term = use_state_eq(|| Rc::<str>::from("inherit"));
    let font = use_state_eq(|| Rc::<str>::from("sans-serif"));
    let size_cell_width = use_state_eq(|| Rc::<str>::from("96px"));
    let size_cell_height = use_state_eq(|| Rc::<str>::from("96px"));
    let size_header_height = use_state_eq(|| Rc::<str>::from("96px"));
    let size_text = use_state_eq(|| Rc::<str>::from("24px"));
    let size_text_year = use_state_eq(|| Rc::<str>::from("48px"));
    let size_text_month = use_state_eq(|| Rc::<str>::from("32px"));
    let size_text_weekday = use_state_eq(|| Rc::<str>::from("inherit"));
    let size_text_week_number = use_state_eq(|| Rc::<str>::from("inherit"));
    let size_text_chinese = use_state_eq(|| Rc::<str>::from("inherit"));
    let size_year_margin = use_state_eq(|| Rc::<str>::from("64px"));

    let language_index = use_state_eq(|| 0u32);
    let enable_chinese = use_state_eq(|| false);
    let start_on_monday = use_state_eq(|| false);
    let show_week_numbers = use_state_eq(|| false);
    let highlight_today = use_state_eq(|| true);

    let language = LANGUAGES
        .get(*language_index as usize)
        .copied()
        .unwrap_or(Language::English);
    let options = Options {
        language,
        enable_chinese: *enable_chinese,
        start_on_monday: *start_on_monday,
        week_number: *show_week_numbers,
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
    let css_import = use_memo(font.deref().clone(), |font| {
        let fonts = font
            .split(',')
            .map(|s| s.trim().trim_matches('"'))
            .filter(|s| !GENERIC_FONTS.contains(&s.to_lowercase().as_str()))
            .flat_map(|s| {
                "&family="
                    .chars()
                    .chain(s.chars().map(|ch| if ch == ' ' { '+' } else { ch }))
            })
            .collect::<String>();
        if fonts.is_empty() {
            String::new()
        } else {
            format!("@import url(\"https://fonts.googleapis.com/css2?display=swap{fonts}\");")
        }
    });

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
    let color_year_setter = color_year.setter();
    let color_month_setter = color_month.setter();
    let color_today_text_setter = color_today_text.setter();
    let color_weekend_setter = color_weekend.setter();
    let color_week_number_setter = color_week_number.setter();
    let color_festival_setter = color_festival.setter();
    let color_solar_term_setter = color_solar_term.setter();
    let font_setter = font.setter();
    let size_cell_width_setter = size_cell_width.setter();
    let size_cell_height_setter = size_cell_height.setter();
    let size_header_height_setter = size_header_height.setter();
    let size_text_setter = size_text.setter();
    let size_text_year_setter = size_text_year.setter();
    let size_text_month_setter = size_text_month.setter();
    let size_text_weekday_setter = size_text_weekday.setter();
    let size_text_week_number_setter = size_text_week_number.setter();
    let size_text_chinese_setter = size_text_chinese.setter();
    let size_year_margin_setter = size_year_margin.setter();

    let language_setter = language_index.setter();
    let enable_chinese_setter = enable_chinese.setter();
    let start_on_monday_setter = start_on_monday.setter();
    let show_week_numbers_setter = show_week_numbers.setter();
    let highlight_today_setter = highlight_today.setter();

    html! { <>
        if !css_import.is_empty() {
            <style>{ css_import }</style>
        }
        <style>{
            format!("
            :root {{
                --color-text: {color_text};
                --color-theme: {color_theme};
                --color-year: {color_year};
                --color-month: {color_month};
                --color-today-text: {color_today_text};
                --color-weekend: {color_weekend};
                --color-week-number: {color_week_number};
                --color-festival: {color_festival};
                --color-solar-term: {color_solar_term};
                --size-cell-width: {size_cell_width};
                --size-cell-height: {size_cell_height};
                --size-header-height: {size_header_height};
                --size-text: {size_text};
                --size-text-weekday: {size_text_weekday};
                --size-text-week-number: {size_text_week_number};
                --size-text-chinese: {size_text_chinese};
                --size-text-month: {size_text_month};
                --size-text-year: {size_text_year};
                --size-year-margin: {size_year_margin};
            }}
            
            body {{
                font-family: {font};
            }}
            ",
            color_text = color_text.deref(),
            color_theme = color_theme.deref(),
            color_year = color_year.deref(),
            color_month = color_month.deref(),
            color_today_text = color_today_text.deref(),
            color_weekend = color_weekend.deref(),
            color_week_number = color_week_number.deref(),
            color_festival = color_festival.deref(),
            color_solar_term = color_solar_term.deref(),
            font = font.deref(),
            size_cell_width = size_cell_width.deref(),
            size_cell_height = size_cell_height.deref(),
            size_header_height = size_header_height.deref(),
            size_text = size_text.deref(),
            size_text_weekday = size_text_weekday.deref(),
            size_text_week_number = size_text_week_number.deref(),
            size_text_chinese = size_text_chinese.deref(),
            size_text_month = size_text_month.deref(),
            size_text_year = size_text_year.deref(),
            size_year_margin = size_year_margin.deref(),
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
                    <tr>
                        if *show_week_numbers { <th></th> }
                        {
                            for Weekdays(if *start_on_monday { Weekday::Mon } else { Weekday::Sun })
                                .take(7)
                                .map(|weekday| html! {
                                    <th class={classes!(
                                        "weekday",
                                        is_weekend(weekday).then_some("weekend"),
                                    )}>
                                        { weekday.short().translate_to_string(language) }
                                    </th>
                                })
                        }
                    </tr>
                    {
                        for calendar.iter().map(|(week_number, row)| html! {
                            <tr>
                                if *show_week_numbers {
                                    <th class="week-number">{ week_number }</th>
                                }
                                {
                                    for row.map(|cell| html! {
                                        if let Some(cell) = cell {
                                            <td class={ classes!(
                                                cell.today.then_some("today"),
                                                cell.weekend.then_some("weekend"),
                                            ) }>
                                                <div class="day">{ cell.date.day() }</div>
                                                if let Some(festival) = cell.festival {
                                                    <div class="chinese festival">{
                                                        festival.static_translate(language)
                                                    }</div>
                                                } else if let Some(solar_term) = cell.solar_term {
                                                    <div class="chinese solar-term">{
                                                        solar_term.static_translate(language)
                                                    }</div>
                                                } else if let Some(chinese) = cell.chinese_date {
                                                    <div class="chinese">{
                                                        chinese
                                                            .short()
                                                            .translate_to_string(language)
                                                    }</div>
                                                } else if *enable_chinese {
                                                    <div class="chinese"></div>
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
        if let Some(dialog) = active_dialog_value {{
            match dialog {
                Dialog::Jump => html! { <div class="dialog">
                    <div class="title">{ translations::Jump.static_translate(language) }</div>
                    <Form>
                        <IntInput
                            name={ translations::Year.static_translate(language) }
                            min={ Some(-262143) }
                            max={ Some(262142) }
                            value={ year_month.year }
                            onchange={ move |value| {
                                year_month_dispatcher3
                                    .dispatch(YearMonthAction::SetYear(value));
                        } }/>
                        <Select
                            name={ translations::Month.static_translate(language) }
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
                    <div class="title">{ translations::Styles.static_translate(language) }</div>
                    <Form>
                        <ColorInput
                            name={ translations::TextColor.static_translate(language) }
                            value={ color_text.deref().clone() }
                            onchange={ move |value| color_text_setter.set(Rc::from(value)) }
                        />
                        <ColorInput
                            name={ translations::ThemeColor.static_translate(language) }
                            value={ color_theme.deref().clone() }
                            onchange={ move |value| color_theme_setter.set(Rc::from(value)) }
                        />
                        <ColorInput
                            name={ translations::YearColor.static_translate(language) }
                            value={ color_year.deref().clone() }
                            onchange={ move |value| color_year_setter.set(Rc::from(value)) }
                        />
                        <ColorInput
                            name={ translations::MonthColor.static_translate(language) }
                            value={ color_month.deref().clone() }
                            onchange={ move |value| color_month_setter.set(Rc::from(value)) }
                        />
                        <ColorInput
                            name={ translations::TodayTextColor.static_translate(language) }
                            value={ color_today_text.deref().clone() }
                            onchange={move |value| {
                                color_today_text_setter.set(Rc::from(value))
                            } }
                        />
                        <ColorInput
                            name={ translations::WeekendColor.static_translate(language) }
                            value={ color_weekend.deref().clone() }
                            onchange={ move |value| color_weekend_setter.set(Rc::from(value)) }
                        />
                        <ColorInput
                            name={ translations::WeekNumberColor.static_translate(language) }
                            value={ color_week_number.deref().clone() }
                            onchange={ move |value| {
                                color_week_number_setter.set(Rc::from(value))
                            } }
                        />
                        <ColorInput
                            name={ translations::FestivalColor.static_translate(language) }
                            value={ color_festival.deref().clone() }
                            onchange={ move |value| {
                                color_festival_setter.set(Rc::from(value))
                            } }
                        />
                        <ColorInput
                            name={ translations::SolarTermColor.static_translate(language) }
                            value={ color_solar_term.deref().clone() }
                            onchange={ move |value| {
                                color_solar_term_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::Font.static_translate(language) }
                            value={ font.deref().clone() }
                            onchange={ move |value| font_setter.set(Rc::from(value)) }
                        />
                        <StringInput
                            name={ translations::CellWidth.static_translate(language) }
                            value={ size_cell_width.deref().clone() }
                            onchange={ move |value| {
                                size_cell_width_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::CellHeight.static_translate(language) }
                            value={ size_cell_height.deref().clone() }
                            onchange={ move |value| {
                                size_cell_height_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::HeaderHeight.static_translate(language) }
                            value={ size_header_height.deref().clone() }
                            onchange={ {
                                move |value| size_header_height_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::YearMargin.static_translate(language) }
                            value={ size_year_margin.deref().clone() }
                            onchange={ move |value| {
                                size_year_margin_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::TextSize.static_translate(language) }
                            value={ size_text.deref().clone() }
                            onchange={ move |value| size_text_setter.set(Rc::from(value)) }
                        />
                        <StringInput
                            name={ translations::WeekdayTextSize.static_translate(language) }
                            value={ size_text_weekday.deref().clone() }
                            onchange={ move |value| size_text_weekday_setter.set(Rc::from(value)) }
                        />
                        <StringInput
                            name={ translations::WeekNumberTextSize.static_translate(language) }
                            value={ size_text_week_number.deref().clone() }
                            onchange={ move |value| {
                                size_text_week_number_setter.set(Rc::from(value))
                            } }
                        />
                        <StringInput
                            name={ translations::ChineseTextSize.static_translate(language) }
                            value={ size_text_chinese.deref().clone() }
                            onchange={ move |value| size_text_chinese_setter.set(Rc::from(value)) }
                        />
                        <StringInput
                            name={ translations::YearTextSize.static_translate(language) }
                            value={ size_text_year.deref().clone() }
                            onchange={ move |value| size_text_year_setter.set(Rc::from(value)) }
                        />
                        <StringInput
                            name={ translations::MonthTextSize.static_translate(language) }
                            value={ size_text_month.deref().clone() }
                            onchange={ move |value| {
                                size_text_month_setter.set(Rc::from(value))
                            } }
                        />
                    </Form>
                </div> },
                Dialog::Settings => html! { <div class="dialog">
                    <div class="title">{ translations::Settings.static_translate(language) }</div>
                    <Form>
                        <Select
                            name={ translations::Language.static_translate(language) }
                            value={ *language_index }
                            onchange={ move |value| language_setter.set(value) }
                        >
                            <SelectOption>{"English"}</SelectOption>
                            <SelectOption>{"简体中文"}</SelectOption>
                            <SelectOption>{"繁體中文"}</SelectOption>
                        </Select>
                        <CheckboxInput
                            name={ translations::EnableChineseCalendar.static_translate(language) }
                            checked={ *enable_chinese }
                            onchange={ move |checked| enable_chinese_setter.set(checked) }
                        />
                        <CheckboxInput
                            name={ translations::StartOnMonday.static_translate(language) }
                            checked={ *start_on_monday }
                            onchange={ move |checked| start_on_monday_setter.set(checked) }
                        />
                        <CheckboxInput
                            name={ translations::ShowWeekNumbers.static_translate(language) }
                            checked={ *show_week_numbers }
                            onchange={ move |checked| show_week_numbers_setter.set(checked) }
                        />
                        <CheckboxInput
                            name={ translations::HighlightToday.static_translate(language) }
                            checked={ *highlight_today }
                            onchange={ move |checked| highlight_today_setter.set(checked) }
                        />
                    </Form>
                </div> }
            } }
        }
        <div class="corner-buttons">
            <button
                title={ translations::PrevMonth.static_translate(language) }
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
                title={ translations::NextMonth.static_translate(language) }
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
                title={ translations::Today.static_translate(language) }
                class="material-symbols-outlined"
                onclick={ move |_| year_month_dispatcher2.dispatch(YearMonthAction::Today) }
            >
                {"today"}
            </button>
            <button
                title={ translations::Jump.static_translate(language) }
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Jump)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher.dispatch(Dialog::Jump) }
            >
                {"calendar_month"}
            </button>
            <button
                title={ translations::Print.static_translate(language) }
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
                title={ translations::Styles.static_translate(language) }
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Styles)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher1.dispatch(Dialog::Styles) }
            >
                {"style"}
            </button>
            <button
                title={ translations::Settings.static_translate(language) }
                class={ classes!(
                    "material-symbols-outlined",
                    (active_dialog_value == Some(Dialog::Settings)).then_some("active"),
                ) }
                onclick={ move |_| active_dialog_dispatcher2.dispatch(Dialog::Settings) }
            >
                {"settings"}
            </button>
            <a href="https://github.com/supertsy5/calendar">
                <button title="GitHub" class="material-symbols-outlined">{"code"}</button>
            </a>
        </div>
    </>}
}
