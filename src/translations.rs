use std::fmt::{Display, Formatter, Result as FmtResult};

use nongli::language::{
    Language::{self as Language0, *},
    StaticTranslate, Translate,
};

macro_rules! translate {
    ($(($ident: ident, $en: literal, $ch: literal)),* $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $ident;

            impl StaticTranslate for $ident {
                fn static_translate(&self, language: Language0) -> &'static str {
                    match language {
                        English => $en,
                        _ => $ch,
                    }
                }
            }

            impl Translate for $ident {
                fn translate(&self, language: Language0, f: &mut Formatter) -> FmtResult {
                    self.static_translate(language).fmt(f)
                }
            }
        )*
    };

    ($(($ident: ident, $en: literal, $ch_s: literal, $ch_t: literal)),* $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $ident;

            impl StaticTranslate for $ident {
                fn static_translate(&self, language: Language0) -> &'static str {
                    match language {
                        English => $en,
                        ChineseSimplified => $ch_s,
                        ChineseTraditional => $ch_t,
                    }
                }
            }

            impl Translate for $ident {
                fn translate(&self, language: Language0, f: &mut Formatter) -> FmtResult {
                    self.static_translate(language).fmt(f)
                }
            }
        )*
    };
}

translate!(
    (PrevMonth, "Previous Month", "上一月"),
    (NextMonth, "Next Month", "下一月"),
    (Today, "Today", "今日"),
    (Year, "Year", "年"),
    (Month, "Month", "月"),
    (HighlightToday, "Highlight Today", "高亮今日"),
);

translate!(
    (Jump, "Jump", "跳转", "跳轉"),
    (Print, "Print", "打印", "列印"),
    (Styles, "Styles", "样式", "樣式"),
    (Settings, "Settings", "设置", "設定"),
    (TextColor, "Text Color", "文字颜色", "文字顏色"),
    (ThemeColor, "Theme Color", "主题颜色", "主題顏色"),
    (YearColor, "Year Color", "年颜色", "年顏色"),
    (MonthColor, "Month Color", "月颜色", "月顏色"),
    (TodayTextColor, "Today Text Color", "今日文字颜色", "今日文字顏色"),
    (WeekendColor, "Weekend Color", "周末颜色", "週末顏色"),
    (WeekNumberColor, "Week Number Color", "周数字号", "週數顏色"),
    (SolarTermColor, "Solar Term Color", "节气颜色", "節氣顏色"),
    (Font, "Font", "字体", "字體"),
    (CellWidth, "Cell Width", "单元格宽度", "單元格寬度"),
    (CellHeight, "Cell Height", "单元格高度", "單元格高度"),
    (HeaderHeight, "Header Height", "标题高度", "標題高度"),
    (YearMargin, "Year Margin", "年边距", "年邊距"),
    (TextSize, "Text Size", "字号", "字號"),
    (WeekdayTextSize, "Weekday Text Size", "星期字号", "星期字號"),
    (WeekNumberTextSize, "Week Number Text Size", "周数字号", "週數字號"),
    (ChineseTextSize, "Chinese Date Text Size", "农历字号", "農曆字號"),
    (YearTextSize, "Year Text Size", "年字号", "年字號"),
    (MonthTextSize, "Month Text Size", "月字号", "月字號"),
    (Language, "Language", "语言", "語言"),
    (EnableChineseCalendar, "Enable Chinese Calendar", "启用农历", "啟用農曆"),
    (StartOnMonday, "Start on Monday", "从星期一开始", "從星期一開始"),
    (ShowWeekNumbers, "Show Week Numbers", "显示周数", "顯示週數"),
);
