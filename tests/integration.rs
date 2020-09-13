mod common;
use crate::common::{
    default_cs, destroy_i2c, destroy_spi, new_i2c, new_spi_accel, new_spi_mag, BitFlags as BF,
    Register, ACCEL_ADDR, MAG_ADDR,
};
use embedded_hal_mock::{
    i2c::Transaction as I2cTrans, pin::Mock as PinMock, spi::Transaction as SpiTrans,
};

#[test]
fn can_create_and_destroy_i2c() {
    let sensor = new_i2c(&[]);
    destroy_i2c(sensor);
}

#[test]
fn can_create_and_destroy_spi() {
    let sensor = new_spi_accel(&[], PinMock::new(&[]));
    destroy_spi(sensor);
}

#[test]
fn i2c_can_get_accel_id() {
    let accel_id = 0xAB;
    let mut sensor = new_i2c(&[I2cTrans::write_read(
        ACCEL_ADDR,
        vec![Register::WHO_AM_I_A],
        vec![accel_id],
    )]);
    let id = sensor.accelerometer_id().unwrap();
    assert_eq!(accel_id, id);
    destroy_i2c(sensor);
}

#[test]
fn i2c_can_get_mag_id() {
    let mag_id = 0xAB;
    let mut sensor = new_i2c(&[I2cTrans::write_read(
        MAG_ADDR,
        vec![Register::WHO_AM_I_M],
        vec![mag_id],
    )]);
    let id = sensor.magnetometer_id().unwrap();
    assert_eq!(mag_id, id);
    destroy_i2c(sensor);
}

#[test]
fn spi_can_get_accel_id() {
    let accel_id = 0xAB;
    let mut sensor = new_spi_accel(
        &[SpiTrans::transfer(
            vec![BF::SPI_RW | Register::WHO_AM_I_A, 0],
            vec![0, accel_id],
        )],
        default_cs(),
    );
    let id = sensor.accelerometer_id().unwrap();
    assert_eq!(accel_id, id);
    destroy_spi(sensor);
}

#[test]
fn spi_can_get_mag_id() {
    let mag_id = 0xAB;
    let mut sensor = new_spi_mag(
        &[SpiTrans::transfer(
            vec![BF::SPI_RW | Register::WHO_AM_I_M, 0],
            vec![0, mag_id],
        )],
        default_cs(),
    );
    let id = sensor.magnetometer_id().unwrap();
    assert_eq!(mag_id, id);
    destroy_spi(sensor);
}

#[test]
fn can_init() {
    let mut sensor = new_i2c(&[I2cTrans::write(
        ACCEL_ADDR,
        vec![Register::CTRL_REG4_A, BF::BDU],
    )]);
    sensor.init().unwrap();
    destroy_i2c(sensor);
}
