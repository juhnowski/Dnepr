; Программа сглаживания и калибровки датчика
DEFINE PRESS_SENSOR 3
DEFINE VALVE_ACTUATOR 1

; Секция кода
START:
    SEL_CH PRESS_SENSOR
    READ_ADC
    STORE RAW_VAL        ; Сохраняем сырое значение в переменную ОЗУ

    ; Масштабируем сигнал: умножаем на калибровочный коэффициент
    MULT RAW_VAL CAL_FACT
    STORE SCALE_VAL

    ; Вычитаем аппаратное смещение (погрешность нуля)
    SUB SCALE_VAL OFFSET_VAL
    STORE FINAL_VAL

    ; Выводим чистый результат на клапан ЦАП
    WRITE_DAC VALVE_ACTUATOR FINAL_VAL

    JZ EXIT_PROG         ; Если на выходе чистый ноль — останавливаемся
    JUMP START           ; Иначе продолжаем цикл

EXIT_PROG:
    HALT

; Секция данных (выделение ячеек под переменные и константы)
RAW_VAL:    DATA 0.0     ; Сюда запишется сырой сигнал
SCALE_VAL:  DATA 0.0     ; Промежуточный результат
FINAL_VAL:  DATA 0.0     ; Финальный результат

CAL_FACT:   DATA 0.80    ; Коэффициент усиления датчика (80%)
OFFSET_VAL: DATA 0.10    ; Постоянное смещение датчика (-0.10)
