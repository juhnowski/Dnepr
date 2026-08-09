pub const WORD_MASK: u32 = 0x03FF_FFFF;
pub const SIGN_BIT: u32 = 0x0200_0000; // 26-й бит (знаковый)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DneprWord(pub u32);

impl DneprWord {
    /// Создание слова из сырого 32-битного значения с маскированием
    pub fn from_raw(val: u32) -> Self {
        DneprWord(val & WORD_MASK)
    }

    /// Преобразование вещественного числа [-1.0, 1.0) в 26-битный формат «Днепр» с фиксированной запятой
    pub fn from_float(val: f64) -> Self {
        let clamped = val.clamp(-1.0, 0.999_999_94);
        if clamped >= 0.0 {
            DneprWord((clamped * ((1 << 25) as f64)) as u32 & WORD_MASK)
        } else {
            let abs_scaled = (clamped.abs() * ((1 << 25) as f64)) as u32;
            let twos_complement = ((!abs_scaled).wrapping_add(1)) & WORD_MASK;
            DneprWord(twos_complement | SIGN_BIT)
        }
    }

    /// Преобразование 26-битного формата «Днепр» в вещественное число f64
    pub fn to_float(self) -> f64 {
        if (self.0 & SIGN_BIT) != 0 {
            let inverted = (!self.0 & WORD_MASK).wrapping_add(1) & WORD_MASK;
            -(inverted as f64) / ((1 << 25) as f64)
        } else {
            (self.0 as f64) / ((1 << 25) as f64)
        }
    }

    /// Расширение знака с 26-битного целого до 32-битного знакового целого (i32)
    pub fn sign_extend(self) -> i32 {
        if (self.0 & SIGN_BIT) != 0 {
            (self.0 | 0xFC00_0000) as i32
        } else {
            self.0 as i32
        }
    }

    /// Сложение двух слов «Днепр»
    pub fn add(self, other: Self) -> Self {
        let val1 = self.sign_extend();
        let val2 = other.sign_extend();
        DneprWord((val1.wrapping_add(val2) as u32) & WORD_MASK)
    }

    /// Вычитание двух слов «Днепр»
    pub fn sub(self, other: Self) -> Self {
        let val1 = self.sign_extend();
        let val2 = other.sign_extend();
        DneprWord((val1.wrapping_sub(val2) as u32) & WORD_MASK)
    }

    /// Умножение двух слов «Днепр»
    pub fn multiply(self, other: Self) -> Self {
        let val1 = self.sign_extend() as i64;
        let val2 = other.sign_extend() as i64;
        let shifted = (val1 * val2) >> 25;
        DneprWord((shifted as u32) & WORD_MASK)
    }
}
