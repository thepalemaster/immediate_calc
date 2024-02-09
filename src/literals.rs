pub mod messages;

pub const STEP: f32 = 50.;

pub const APP_TITLE: &str = if cfg!(feature = "lang_rus") {
    "Калькулятор площади"
} else {
    "Area calculator"
};

pub const INPUT_UNITS: &str = if cfg!(feature = "lang_rus") {
    "Единицы ввода"
} else {
    "Input units"
};

pub const OUTPUT_UNITS: &str = if cfg!(feature = "lang_rus") {
    "Единицы вывода"
} else {
    "Output units"
};

pub const FACTOR: &str = if cfg!(feature = "lang_rus") {
    "Коэфицент"
} else {
    "Factor"
};

pub const CALCULATE: &str = if cfg!(feature = "lang_rus") {
    "Рассчитать"
} else {
    "Calculate"
};

pub const CLEAR: &str = if cfg!(feature = "lang_rus") {
    "Очистить"
} else {
    "Clear"
};

pub const COPY: &str = if cfg!(feature = "lang_rus") {
    "Скопировать"
} else {
    "Copy to buffer"
};

pub const TOTAL: &str = if cfg!(feature = "lang_rus") {
    "Итого:"
} else {
    "Total:"
};

pub const EDIT: &str = if cfg!(feature = "lang_rus") {
    "Редактировать"
} else {
    "Edit shape"
};

pub const CANCEL: &str = if cfg!(feature = "lang_rus") {
    "Отмена"
} else {
    "Cancel"
};

pub const SAVE: &str = if cfg!(feature = "lang_rus") {
    "Сохранить"
} else {
    "Save"
};

pub const MM: &str = if cfg!(feature = "lang_rus") {
    "мм"
} else {
    "mm"
};

pub const SM: &str = if cfg!(feature = "lang_rus") {
    "см"
} else {
    "sm"
};

pub const DM: &str = if cfg!(feature = "lang_rus") {
    "дм"
} else {
    "dm"
};

pub const M: &str = if cfg!(feature = "lang_rus") {
    "м"
} else {
    "m"
};

pub const MM2: &str = if cfg!(feature = "lang_rus") {
    "мм²"
} else {
    "mm²"
};

pub const SM2: &str = if cfg!(feature = "lang_rus") {
    "см²"
} else {
    "sm²"
};

pub const DM2: &str = if cfg!(feature = "lang_rus") {
    "дм²"
} else {
    "dm²"
};

pub const M2: &str = if cfg!(feature = "lang_rus") {
    "м²"
} else {
    "m²"
};

pub const PARSE_ERROR: &str = if cfg!(feature = "lang_rus") {
    "Ввод некорректных данных"
} else {
    "Wrong input"
};
