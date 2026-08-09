; Программа проверки внешнего флага с пульта
DEFINE PRESS_SENSOR 3
DEFINE VALVE_ACTUATOR 1

START:
    SEL_CH PRESS_SENSOR
    READ_ADC
    STORE 100

    ; Если тумблер П0 поднят, прыгаем на аварийный режим
    JPS 0 EMERGENCY_MODE

    ; Стандартный режим работы
    WRITE_DAC VALVE_ACTUATOR 100
    JUMP START

EMERGENCY_MODE:
    ; Аварийный режим: принудительно пишем ноль (закрываем клапан)
    WRITE_DAC VALVE_ACTUATOR ZERO_VAL
    JUMP START

ZERO_VAL: DATA 0.0
