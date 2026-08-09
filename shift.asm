; Тестирование побитовых сдвигов ЭВМ Днепр
START:
    SHR NUM_CELL 2   ; Сдвиг вправо на 2 бита (деление 0.80 на 4 = 0.20)
    STORE RESULT_1

    SHL RESULT_1 1   ; Сдвиг влево на 1 бит (умножение 0.20 на 2 = 0.40)
    STORE RESULT_2
    HALT

NUM_CELL: DATA 0.80
RESULT_1: DATA 0.0
RESULT_2: DATA 0.0
