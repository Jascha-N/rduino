#pragma once

#include <Arduino.h>
#include <cstdlib>
#include <cstdint>

extern "C" {

// Pins
#define RDUINO_NUM_DIGITAL_PINS NUM_DIGITAL_PINS

#ifdef NUM_ANALOG_INPUTS
#   if NUM_ANALOG_INPUTS > 16
#       warning "More than 16 analog input pins"
#       define RDUINO_NUM_ANALOG_INPUTS 16
#   else
#       define RDUINO_NUM_ANALOG_INPUTS NUM_ANALOG_INPUTS
#   endif
#else
#   define RDUINO_NUM_ANALOG_INPUTS 0
#endif

#ifdef NUM_ANALOG_OUTPUTS
#   if NUM_ANALOG_OUTPUTS > 8
#       warning "More than 8 analog output pins"
#       define RDUINO_NUM_ANALOG_OUTPUTS 8
#   else
#       define RDUINO_NUM_ANALOG_OUTPUTS NUM_ANALOG_OUTPUTS
#   endif
#elif defined(ARDUINO_SAM_DUE)
// Workaround: Arduino Due does not define NUM_ANALOG_OUTPUTS, because of reasons.
#   define RDUINO_NUM_ANALOG_OUTPUTS 2
#else
#   define RDUINO_NUM_ANALOG_OUTPUTS 0
#endif

extern const uint8_t RDUINO_ANALOG_INPUT_PINS[RDUINO_NUM_ANALOG_INPUTS];
extern const uint8_t RDUINO_ANALOG_OUTPUT_PINS[RDUINO_NUM_ANALOG_OUTPUTS];

bool rduino_digital_pin_has_pwm(uint8_t pin);
int rduino_digital_pin_to_interrupt(uint8_t pin);

// Digital I/O
enum class RduinoPinLevel {
    Low,
    High
};

enum class RduinoPinMode {
    Input,
    InputPullup,
    Output,

    // SAMD only
    InputPulldown,
};

void rduino_pin_mode(uint8_t pin, RduinoPinMode mode);
void rduino_digital_write(uint8_t pin, RduinoPinLevel value);
RduinoPinLevel rduino_digital_read(uint8_t pin);

// Analog I/O
enum class RduinoAnalogReference {
    Default,
    External,
    Internal,

    // AVR only
    Internal1v1,
    Internal2v56,

    // SAMD only
    Internal1v0,
    Internal1v65,
    Internal2v23
};

void rduino_analog_reference(RduinoAnalogReference type);
uint16_t rduino_analog_read(uint8_t pin);
void rduino_analog_write(uint8_t pin, uint16_t value);

void rduino_analog_read_resolution(uint8_t res);
void rduino_analog_write_resolution(uint8_t res);

// Advanced I/O
enum class RduinoBitOrder {
    MsbFirst,
    LsbFirst
};

void rduino_tone(uint8_t pin, unsigned int frequency, unsigned long duration);
void rduino_no_tone(uint8_t pin);
void rduino_shift_out(uint8_t data_pin, uint8_t clock_pin, RduinoBitOrder bit_order, uint8_t value);
uint8_t rduino_shift_in(uint8_t data_pin, uint8_t clock_pin, RduinoBitOrder bit_order);
unsigned long rduino_pulse_in(uint8_t pin, RduinoPinLevel value, unsigned long timeout);

// Time
unsigned long rduino_millis(void);
unsigned long rduino_micros(void);
void rduino_delay(unsigned long ms);
void rduino_delay_microseconds(unsigned int us);

// Random
void rduino_random_seed(unsigned long seed);
void rduino_random(long min, long max);

// External interrupts
enum class RduinoInterruptMode {
    Low,
    Change,
    Rising,
    Falling,

    // SAMD only
    High
};

typedef void(* RduinoIsr)(void);

void rduino_attach_interrupt(uint8_t interrupt, RduinoIsr isr, RduinoInterruptMode mode);
void rduino_detach_interrupt(uint8_t interrupt);

// Communication
struct RduinoSerial;

enum class RduinoSerialConfig {
    Serial5N1,
    Serial6N1,
    Serial7N1,
    Serial8N1,
    Serial5N2,
    Serial6N2,
    Serial7N2,
    Serial8N2,
    Serial5E1,
    Serial6E1,
    Serial7E1,
    Serial8E1,
    Serial5E2,
    Serial6E2,
    Serial7E2,
    Serial8E2,
    Serial5O1,
    Serial6O1,
    Serial7O1,
    Serial8O1,
    Serial5O2,
    Serial6O2,
    Serial7O2,
    Serial8O2
};

RduinoSerial *rduino_serial_default();
RduinoSerial *rduino_serial_usbvirtual();
RduinoSerial *rduino_serial_monitor();
RduinoSerial *rduino_serial_linuxbridge();
RduinoSerial *rduino_serial_hardware();
RduinoSerial *rduino_serial_hardware_open();

bool rduino_serial_ready(RduinoSerial *serial);
bool rduino_serial_begin(RduinoSerial *serial, unsigned long speed, RduinoSerialConfig config);
void rduino_serial_end(RduinoSerial *serial);
void rduino_serial_set_timeout(RduinoSerial *serial, unsigned long timeout);

size_t rduino_serial_available(RduinoSerial *serial);
int rduino_serial_read(RduinoSerial *serial);
size_t rduino_serial_read_bytes(RduinoSerial *serial, uint8_t *buffer, size_t length);
int rduino_serial_peek(RduinoSerial *serial);

size_t rduino_serial_available_for_write(RduinoSerial *serial);
//int rduino_serial_get_write_error(RduinoSerial *serial);
bool rduino_serial_write(RduinoSerial *serial, uint8_t value);
size_t rduino_serial_write_bytes(RduinoSerial *serial, const uint8_t *buffer, size_t length);
void rduino_serial_flush(RduinoSerial *serial);

}
