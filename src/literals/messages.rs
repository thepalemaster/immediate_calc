pub const BUFFER_ERROR: &str = if cfg!(feature = "lang_rus") {
    "Буфер обмена не доступен"
} else {
    "System clipboard unavailable"
};

pub const BUFFER_COPIED: &str = if cfg!(feature = "lang_rus") {
    "Скопированно в буфер обмена"
} else {
    "Copied to clipboard"
};

pub const BUFFER_FAIL: &str = if cfg!(feature = "lang_rus") {
    "Не удалось скопировать в буфер обмена"
} else {
    "Copy to clipboard failed"
};

pub const CALCULATION_ERR: &str = if cfg!(feature = "lang_rus") {
    "Ошибка вычисления"
} else {
    "Calculation error"
};

pub const INPUT_WRONG: &str = if cfg!(feature = "lang_rus") {
    "Некорректный ввод"
} else {
    "Incorrect input"
};

pub const PARSE_WRONG: &str = if cfg!(feature = "lang_rus") {
    "Ошибка преобразования в число"
} else {
    "Error converting to a digit"
};

pub const WRONG_FIELD: &str = if cfg!(feature = "lang_rus") {
    "Ошибка доступа у полю ввода"
} else {
    "Error accessing to the input firld"
};

pub const VIEW_FAIL: &str = if cfg!(feature = "lang_rus") {
    "Ошибка отображения результата"
} else {
    "Result display error"
};

pub const SHAPE_FAIL: &str = if cfg!(feature = "lang_rus") {
    "Неизвестная фигура"
} else {
    "Unknown shape"
};
