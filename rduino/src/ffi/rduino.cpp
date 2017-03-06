#include "rduino.hpp"

#include <Arduino.h>

extern "C" {

// Pins
const uint8_t RDUINO_ANALOG_INPUT_PINS[RDUINO_NUM_ANALOG_INPUTS] = {
#if RDUINO_NUM_ANALOG_INPUTS > 0
    A0,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 1
    A1,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 2
    A2,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 3
    A3,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 4
    A4,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 5
    A5,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 6
    A6,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 7
    A7,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 8
    A8,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 9
    A9,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 10
    A10,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 11
    A11,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 12
    A12,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 13
    A13,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 14
    A14,
#endif
#if RDUINO_NUM_ANALOG_INPUTS > 15
    A15,
#endif
};

const uint8_t RDUINO_ANALOG_OUTPUT_PINS[RDUINO_NUM_ANALOG_OUTPUTS] = {
#ifdef ARDUINO_SAM_DUE
    DAC0, DAC1
#else
#   if RDUINO_NUM_ANALOG_OUTPUTS > 0
        // FIXME: Workaround for boards that are missing PIN_DACx defines (such as MKRZero).
        // This might not be correct. Please fix your shit Arduino.
#       ifdef PIN_DAC0
            DAC0,
#       else
            A0,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 1
#       ifdef PIN_DAC1
            DAC1,
#       else
            A1,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 2
#       ifdef PIN_DAC2
            DAC2,
#       else
            A2,
#           endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 3
#       ifdef PIN_DAC3
            DAC3,
#       else
            A3,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 4
#       ifdef PIN_DAC4
            DAC4,
#       else
            A4,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 5
#       ifdef PIN_DAC5
            DAC5,
#       else
            A5,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 6
#       ifdef PIN_DAC6
            DAC6,
#       else
            A6,
#       endif
#   endif
#   if RDUINO_NUM_ANALOG_OUTPUTS > 7
#       ifdef PIN_DAC7
            DAC7,
#       else
            A7,
#       endif
#   endif
#endif
};

bool rduino_digital_pin_has_pwm(uint8_t pin) {
    return digitalPinHasPWM(pin);
}

int rduino_digital_pin_to_interrupt(uint8_t pin) {
    int interrupt = digitalPinToInterrupt(pin);
    if (interrupt == NOT_AN_INTERRUPT) {
        return -1;
    }
    return interrupt;
}

// Digital I/O
void rduino_pin_mode(uint8_t pin, RduinoPinMode mode) {
    unsigned int real_mode;
    switch (mode) {
        case RduinoPinMode::Input:
            real_mode = INPUT;
            break;
        case RduinoPinMode::InputPullup:
            real_mode = INPUT_PULLUP;
            break;
        case RduinoPinMode::Output:
            real_mode = OUTPUT;
            break;
#ifdef ARDUINO_ARCH_SAMD
        case RduinoPinMode::InputPulldown:
            real_mode = INPUT_PULLDOWN;
            break;
#endif
        default:
            return;
    }
    pinMode(pin, real_mode);
}

void rduino_digital_write(uint8_t pin, RduinoPinLevel value) {
    digitalWrite(pin, value == RduinoPinLevel::High ? HIGH : LOW);
}

RduinoPinLevel rduino_digital_read(uint8_t pin) {
    return digitalRead(pin) == HIGH ? RduinoPinLevel::High : RduinoPinLevel::Low;
}


// Analog I/O
#ifdef ARDUINO_ARCH_AVR
void rduino_analog_reference(RduinoAnalogReference type) {
    uint8_t real_type;
    switch (type) {
        case RduinoAnalogReference::Default:
            real_type = DEFAULT;
            break;
        case RduinoAnalogReference::External:
            real_type = EXTERNAL;
            break;
        case RduinoAnalogReference::Internal:
// Fall-through if INTERNAL is not defined
#ifdef INTERNAL
            real_type = INTERNAL;
            break;
#endif
#ifdef INTERNAL1V1
        case RduinoAnalogReference::Internal1v1:
            real_type = INTERNAL1V1;
            break;
#endif
#ifdef INTERNAL2V56
        case RduinoAnalogReference::Internal2v56:
            real_type = INTERNAL2V56;
            break;
#endif
        default:
            return;
    }
    analogReference(real_type);
}
#endif

#ifdef ARDUINO_ARCH_SAMD
void rduino_analog_reference(RduinoAnalogReference type) {
    eAnalogReference real_type;
    switch (type) {
        case RduinoAnalogReference::Default:
            real_type = eAnalogReference::AR_DEFAULT;
            break;
        case RduinoAnalogReference::External:
            real_type = eAnalogReference::AR_EXTERNAL;
            break;
        case RduinoAnalogReference::Internal:
            real_type = eAnalogReference::AR_INTERNAL;
            break;

        case RduinoAnalogReference::Internal1v0:
            real_type = eAnalogReference::AR_INTERNAL1V0;
            break;
        case RduinoAnalogReference::Internal1v65:
            real_type = eAnalogReference::AR_INTERNAL1V65;
            break;
        case RduinoAnalogReference::Internal2v23:
            real_type = eAnalogReference::AR_INTERNAL2V23;
            break;
        default:
            return;
    }
    analogReference(real_type);
}
#endif

uint16_t rduino_analog_read(uint8_t pin) {
    return analogRead(pin);
}

void rduino_analog_write(uint8_t pin, uint16_t value) {
    analogWrite(pin, value);
}

void rduino_analog_read_resolution(uint8_t res) {
    analogReadResolution(res);
}

void rduino_analog_write_resolution(uint8_t res) {
    analogWriteResolution(res);
}


// Advanced I/O
void rduino_tone(uint8_t pin, unsigned int frequency, unsigned long duration) {
    // Disabled for Arduino Due for whatever reason.
#ifdef ARDUINO_SAM_DUE
    (void)pin;
    (void)frequency;
    (void)duration;
#else
    tone(pin, frequency, duration);
#endif
}

void rduino_no_tone(uint8_t pin) {
#ifdef ARDUINO_SAM_DUE
    (void)pin;
#else
    noTone(pin);
#endif
}

void rduino_shift_out(uint8_t data_pin, uint8_t clock_pin, RduinoBitOrder bit_order, uint8_t value) {
    shiftOut(data_pin, clock_pin, bit_order == RduinoBitOrder::LsbFirst ? LSBFIRST : MSBFIRST, value);
}

uint8_t rduino_shift_in(uint8_t data_pin, uint8_t clock_pin, RduinoBitOrder bit_order) {
    return shiftIn(data_pin, clock_pin, bit_order == RduinoBitOrder::LsbFirst ? LSBFIRST : MSBFIRST);
}

unsigned long rduino_pulse_in(uint8_t pin, RduinoPinLevel value, unsigned long timeout) {
    return pulseIn(pin, value == RduinoPinLevel::High ? HIGH : LOW, timeout);
}


// Time
unsigned long rduino_millis(void) {
    return millis();
}

unsigned long rduino_micros(void) {
    return micros();
}

void rduino_delay(unsigned long ms) {
    delay(ms);
}

void rduino_delay_microseconds(unsigned int us) {
    delayMicroseconds(us);
}


// Random
void rduino_random_seed(unsigned long seed) {
    randomSeed(seed);
}

void rduino_random(long min, long max) {
    random(min, max);
}


// External interrupts
typedef void (* RduinoIsr)(void);

void rduino_attach_interrupt(uint8_t interrupt, RduinoIsr isr, RduinoInterruptMode mode) {
    int real_mode;
    switch (mode) {
        case RduinoInterruptMode::Low:
            real_mode = LOW;
            break;
        case RduinoInterruptMode::Change:
            real_mode = CHANGE;
            break;
        case RduinoInterruptMode::Rising:
            real_mode = RISING;
            break;
        case RduinoInterruptMode::Falling:
            real_mode = FALLING;
            break;
#ifdef ARDUINO_ARCH_SAMD
        case RduinoInterruptMode::High:
            real_mode = HIGH;
            break;
#endif
        default:
            return;
    }
    attachInterrupt(interrupt, isr, real_mode);
}

void rduino_detach_interrupt(uint8_t interrupt) {
    detachInterrupt(interrupt);
}


// Communication
#if defined(ARDUINO_ARCH_SAMD)
typedef Uart RduinoSerialHardware;
#elif defined(ARDUINO_ARCH_SAM)
typedef UARTClass RduinoSerialHardware;
#else
typedef HardwareSerial RduinoSerialHardware;
#endif
typedef Serial_ RduinoSerialUsb;

struct RduinoSerial {
    enum class Type { Uart, Usart, Usb } type;
    union {
        RduinoSerialHardware *hw;
        RduinoSerialUsb *usb;
    };

#ifdef ARDUINO_ARCH_SAM
    RduinoSerial(USARTClass *hw) : type(Type::Usart), hw(hw) {}
#endif
    RduinoSerial(RduinoSerialHardware *hw) : type(Type::Uart), hw(hw) {}
    RduinoSerial(RduinoSerialUsb *usb) : type(Type::Usb), usb(usb) {}

    inline Stream *stream() {
        if (type == Type::Usb) {
            return usb;
        } else {
            return hw;
        }
    }
};

RduinoSerial *rduino_serial_default() {
    static RduinoSerial serial(&Serial);
    return &serial;
}

RduinoSerial *rduino_serial_usbvirtual() {
#ifdef SERIAL_PORT_USBVIRTUAL
    static RduinoSerial serial(&SERIAL_PORT_USBVIRTUAL);
    return &serial;
#else
    return nullptr;
#endif
}

RduinoSerial *rduino_serial_monitor() {
#ifdef SERIAL_PORT_MONITOR
    static RduinoSerial serial(&SERIAL_PORT_MONITOR);
    return &serial;
#else
    return nullptr;
#endif
}

RduinoSerial *rduino_serial_linuxbridge() {
#ifdef SERIAL_PORT_LINUXBRIDGE
    static RduinoSerial serial(&SERIAL_PORT_LINUXBRIDGE);
    return &serial;
#else
    return nullptr;
#endif
}

RduinoSerial *rduino_serial_hardware() {
#ifdef SERIAL_PORT_HARDWARE
    static RduinoSerial serial(&SERIAL_PORT_HARDWARE);
    return &serial;
#else
    return nullptr;
#endif
}

RduinoSerial *rduino_serial_hardware_open() {
#ifdef SERIAL_PORT_HARDWARE_OPEN
    static RduinoSerial serial(&SERIAL_PORT_HARDWARE_OPEN);
    return &serial;
#else
    return nullptr;
#endif
}


bool rduino_serial_ready(RduinoSerial *serial) {
    if (serial->type == RduinoSerial::Type::Usb) {
        return !!(*serial->usb);
    } else {
        return !!(*serial->hw);
    }
}

#ifdef ARDUINO_ARCH_SAM
bool rduino_serial_begin(RduinoSerial *serial, unsigned long speed, RduinoSerialConfig config) {
    bool is_uart_config = false;
    UARTClass::UARTModes uart_config = SERIAL_8N1;
    USARTClass::USARTModes usart_config = SERIAL_5N1;
    switch (config) {
        case RduinoSerialConfig::Serial8N1:
            uart_config = SERIAL_8N1;
            is_uart_config = true;
            break;
        case RduinoSerialConfig::Serial8E1:
            uart_config = SERIAL_8E1;
            is_uart_config = true;
            break;
        case RduinoSerialConfig::Serial8O1:
            uart_config = SERIAL_8O1;
            is_uart_config = true;
            break;

        case RduinoSerialConfig::Serial5N1:
            usart_config = SERIAL_5N1;
            break;
        case RduinoSerialConfig::Serial6N1:
            usart_config = SERIAL_6N1;
            break;
        case RduinoSerialConfig::Serial7N1:
            usart_config = SERIAL_7N1;
            break;
        case RduinoSerialConfig::Serial5N2:
            usart_config = SERIAL_5N2;
            break;
        case RduinoSerialConfig::Serial6N2:
            usart_config = SERIAL_6N2;
            break;
        case RduinoSerialConfig::Serial7N2:
            usart_config = SERIAL_7N2;
            break;
        case RduinoSerialConfig::Serial8N2:
            usart_config = SERIAL_8N2;
            break;
        case RduinoSerialConfig::Serial5E1:
            usart_config = SERIAL_5E1;
            break;
        case RduinoSerialConfig::Serial6E1:
            usart_config = SERIAL_6E1;
            break;
        case RduinoSerialConfig::Serial7E1:
            usart_config = SERIAL_7E1;
            break;
        case RduinoSerialConfig::Serial5E2:
            usart_config = SERIAL_5E2;
            break;
        case RduinoSerialConfig::Serial6E2:
            usart_config = SERIAL_6E2;
            break;
        case RduinoSerialConfig::Serial7E2:
            usart_config = SERIAL_7E2;
            break;
        case RduinoSerialConfig::Serial8E2:
            usart_config = SERIAL_8E2;
            break;
        case RduinoSerialConfig::Serial5O1:
            usart_config = SERIAL_5O1;
            break;
        case RduinoSerialConfig::Serial6O1:
            usart_config = SERIAL_6O1;
            break;
        case RduinoSerialConfig::Serial7O1:
            usart_config = SERIAL_7O1;
            break;
        case RduinoSerialConfig::Serial5O2:
            usart_config = SERIAL_5O2;
            break;
        case RduinoSerialConfig::Serial6O2:
            usart_config = SERIAL_6O2;
            break;
        case RduinoSerialConfig::Serial7O2:
            usart_config = SERIAL_7O2;
            break;
        case RduinoSerialConfig::Serial8O2:
            usart_config = SERIAL_8O2;
            break;
        default:
            return false;
    }

    switch (serial->type) {
        case RduinoSerial::Type::Usb:
            if (is_uart_config) {
                serial->usb->begin(speed, uart_config);
            } else {
                serial->usb->begin(speed, usart_config);
            }
            break;
        case RduinoSerial::Type::Usart:
            {
                USARTClass *usart = static_cast<USARTClass *>(serial->hw);
                if (is_uart_config) {
                    usart->begin(speed, uart_config);
                } else {
                    usart->begin(speed, usart_config);
                }
            }
            break;
        case RduinoSerial::Type::Uart:
            if (is_uart_config) {
                serial->hw->begin(speed, uart_config);
            } else {
                return false;
            }
            break;
    }
    return true;
}
#else
bool rduino_serial_begin(RduinoSerial *serial, unsigned long speed, RduinoSerialConfig config) {
    unsigned int real_config;
    switch (config) {
        case RduinoSerialConfig::Serial5N1:
            real_config = SERIAL_5N1;
            break;
        case RduinoSerialConfig::Serial6N1:
            real_config = SERIAL_6N1;
            break;
        case RduinoSerialConfig::Serial7N1:
            real_config = SERIAL_7N1;
            break;
        case RduinoSerialConfig::Serial8N1:
            real_config = SERIAL_8N1;
            break;
        case RduinoSerialConfig::Serial5N2:
            real_config = SERIAL_5N2;
            break;
        case RduinoSerialConfig::Serial6N2:
            real_config = SERIAL_6N2;
            break;
        case RduinoSerialConfig::Serial7N2:
            real_config = SERIAL_7N2;
            break;
        case RduinoSerialConfig::Serial8N2:
            real_config = SERIAL_8N2;
            break;
        case RduinoSerialConfig::Serial5E1:
            real_config = SERIAL_5E1;
            break;
        case RduinoSerialConfig::Serial6E1:
            real_config = SERIAL_6E1;
            break;
        case RduinoSerialConfig::Serial7E1:
            real_config = SERIAL_7E1;
            break;
        case RduinoSerialConfig::Serial8E1:
            real_config = SERIAL_8E1;
            break;
        case RduinoSerialConfig::Serial5E2:
            real_config = SERIAL_5E2;
            break;
        case RduinoSerialConfig::Serial6E2:
            real_config = SERIAL_6E2;
            break;
        case RduinoSerialConfig::Serial7E2:
            real_config = SERIAL_7E2;
            break;
        case RduinoSerialConfig::Serial8E2:
            real_config = SERIAL_8E2;
            break;
        case RduinoSerialConfig::Serial5O1:
            real_config = SERIAL_5O1;
            break;
        case RduinoSerialConfig::Serial6O1:
            real_config = SERIAL_6O1;
            break;
        case RduinoSerialConfig::Serial7O1:
            real_config = SERIAL_7O1;
            break;
        case RduinoSerialConfig::Serial8O1:
            real_config = SERIAL_8O1;
            break;
        case RduinoSerialConfig::Serial5O2:
            real_config = SERIAL_5O2;
            break;
        case RduinoSerialConfig::Serial6O2:
            real_config = SERIAL_6O2;
            break;
        case RduinoSerialConfig::Serial7O2:
            real_config = SERIAL_7O2;
            break;
        case RduinoSerialConfig::Serial8O2:
            real_config = SERIAL_8O2;
            break;
        default:
            return false;
    }

    if (serial->type == RduinoSerial::Type::Usb) {
        serial->usb->begin(speed, real_config);
    } else {
        serial->hw->begin(speed, real_config);
    }
    return true;
}
#endif

void rduino_serial_end(RduinoSerial *serial) {
    if (serial->type == RduinoSerial::Type::Usb) {
        serial->usb->end();
    } else {
        serial->hw->end();
    }
}

void rduino_serial_set_timeout(RduinoSerial *serial, unsigned long timeout) {
    if (serial->type == RduinoSerial::Type::Usb) {
        serial->usb->setTimeout(timeout);
    } else {
        serial->hw->setTimeout(timeout);
    }
}


size_t rduino_serial_available(RduinoSerial *serial) {
    return serial->stream()->available();
}

int rduino_serial_read(RduinoSerial *serial) {
    return serial->stream()->read();
}

size_t rduino_serial_read_bytes(RduinoSerial *serial, uint8_t *buffer, size_t length) {
    return serial->stream()->readBytes(buffer, length);
}

int rduino_serial_peek(RduinoSerial *serial) {
    return serial->stream()->peek();
}


size_t rduino_serial_available_for_write(RduinoSerial *serial) {
    if (serial->type == RduinoSerial::Type::Usb) {
        return serial->usb->availableForWrite();
    } else {
        return serial->hw->availableForWrite();
    }
}

// int rduino_serial_get_write_error(RduinoSerial *serial) {
//     int code = serial->stream()->getWriteError();
//     serial->stream()->clearWriteError();
//     return code;
// }

bool rduino_serial_write(RduinoSerial *serial, uint8_t value) {
    return serial->stream()->write(value) > 0;
}

size_t rduino_serial_write_bytes(RduinoSerial *serial, const uint8_t *buffer, size_t length) {
    return serial->stream()->write(buffer, length);
}

void rduino_serial_flush(RduinoSerial *serial) {
    serial->stream()->flush();
}

}
