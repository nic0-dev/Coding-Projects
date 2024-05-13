/**
 ******************************************************************************
 * @file           : main.c
 * @author         : Mark Cagas
 * @brief          : Main program body
 ******************************************************************************
 */

#include <stdio.h>
#include <string.h>
#include "stm32f4xx.h"
#include "lcd.h"

#define MY_ADDR                 0x0
#define SLAVE_ADDR_LCD          0x4E // PCF8574
#define SYSTEM_CORE_CLOCK 		72000000  // 72 MHz
I2C_Handle_t i2c1Handle;
GPIO_Handle_t SCLHandle, SDAHandle;
int row = 0, col = 0;

void delay(uint32_t ms) {
	uint32_t i, j;
	// Loop takes 3 cycles, delay for ms milliseconds
	for (i = 0; i < ms; i++) {
		for (j = 0; j < (SYSTEM_CORE_CLOCK / 1000) / 3000; j++) {
			__asm("NOP");  // NOP has a known execution time of 1 cycle
		}
	}
}

void LCD_SendCmd (char cmd) {
    char data_u, data_l;
	uint8_t data_t[4];
	data_u = (cmd & 0xf0);
	data_l = ((cmd << 4) & 0xf0);
	data_t[0] = data_u | 0x0C;  //en=1, rs=0
	data_t[1] = data_u | 0x08;  //en=0, rs=0
	data_t[2] = data_l | 0x0C;  //en=1, rs=0
	data_t[3] = data_l | 0x08;  //en=0, rs=0
	// HAL_I2C_Master_Transmit (&hi2c1, SLAVE_ADDRESS_LCD,(uint8_t *) data_t, 4, 100);
    I2C_MasterSendData(&i2c1Handle, (uint8_t *) data_t, 4, SLAVE_ADDR_LCD);
}

void LCD_SendData (char data) {
	char data_u, data_l;
	uint8_t data_t[4];
	data_u = (data&0xf0);
	data_l = ((data<<4)&0xf0);
	data_t[0] = data_u | 0x0D;  //en=1, rs=0
	data_t[1] = data_u | 0x09;  //en=0, rs=0
	data_t[2] = data_l | 0x0D;  //en=1, rs=0
	data_t[3] = data_l | 0x09;  //en=0, rs=0
	// HAL_I2C_Master_Transmit (&hi2c1, SLAVE_ADDRESS_LCD,(uint8_t *) data_t, 4, 100);
    I2C_MasterSendData(&i2c1Handle, (uint8_t *) data_t, 4, SLAVE_ADDR_LCD);
}

void LCD_Clear (void){
	LCD_SendCmd (0x80);
	for (int i=0; i<70; i++) {
		LCD_SendData (' ');
	}
}

void LCD_SendString(char *str) {
    while (*str) LCD_SendData (*str++);
}

void LCD_PutCur(int row, int col) {
    switch (row) {
        case 0:
            col |= 0x80;
            break;
        case 1:
            col |= 0xC0;
            break;
    }
    LCD_SendCmd (col);
}


void LCD_Init(void) {
    // 4-bit initialization
    delay(50);
    LCD_SendCmd (0x30);
	delay(5);  // wait for >4.1ms
	LCD_SendCmd (0x30);
	delay(1);  // wait for >100us
	LCD_SendCmd (0x30);
	delay(10);
	LCD_SendCmd (0x20);  // 4bit mode
	delay(10);

    // display initialisation
	LCD_SendCmd (0x28); // Function set --> DL=0 (4 bit mode), N = 1 (2 line display) F = 0 (5x8 characters)
	delay(1);
	LCD_SendCmd (0x08); //Display on/off control --> D=0,C=0, B=0  ---> display off
	delay(1);
	LCD_SendCmd (0x01);  // clear display
	delay(1);
	delay(1);
	LCD_SendCmd (0x06); //Entry mode set --> I/D = 1 (increment cursor) & S = 0 (no shift)
	delay(1);
	LCD_SendCmd (0x0C); //Display on/off control --> D = 1, C and B = 0. (Cursor and blink, last two bits)
}

void SystemClock_Config(void){
    RCC->CR &= ~(0x1 << RCC_CR_HSEON);
    RCC->CR |= (1 << RCC_CR_HSEON);

    // Check, not sure
    //    while(!((RCC->CR >> RCC_CR_HSERDY) & 0x1));

    RCC->PLLCFGR &= ~(0x1 << RCC_PLLCFGR_SRC);  // Clearing
    RCC->PLLCFGR |= (1 << RCC_PLLCFGR_SRC);     // Setting

    RCC->PLLCFGR &= ~(0x1F << RCC_PLLCFGR_M);  // Clearing
    RCC->PLLCFGR |= (4 << RCC_PLLCFGR_M);     // Setting

    RCC->PLLCFGR &= ~(0x1FF << RCC_PLLCFGR_N);  // Clearing
    RCC->PLLCFGR |= (192 << RCC_PLLCFGR_N);     // Setting

    RCC->PLLCFGR &= ~(0x3 << RCC_PLLCFGR_P);  // Clearing
    RCC->PLLCFGR |= (2 << RCC_PLLCFGR_P);     // Setting

    RCC->PLLCFGR &= ~(0xF << RCC_PLLCFGR_Q);  // Clearing
    RCC->PLLCFGR |= (4 << RCC_PLLCFGR_Q);     // Setting

    RCC->CR &= ~(0x1 << RCC_CR_PLLON);
    RCC->CR |= (1 << RCC_CR_PLLON);
    // while(!(RCC->CR & RCC_CR_PLLRDY));
    // Check, not sure
    //    while(!((RCC->CR >> RCC_CR_PLLRDY) & 0x1));

    RCC->CFGR &= ~(0x3 << RCC_CFGR_SW);
    RCC->CFGR |= (2 << RCC_CFGR_SW);

    RCC->CFGR &= ~(0x1F << RCC_CFGR_HPRE);
    RCC->CFGR |= (0 << RCC_CFGR_SW);

    RCC->CFGR &= ~(0x7 << RCC_CFGR_PPRE1);
    RCC->CFGR |= (4 << RCC_CFGR_PPRE1);

    RCC->CFGR &= ~(0x7 << RCC_CFGR_PPRE2);
    RCC->CFGR |= (0 << RCC_CFGR_PPRE2);
}

void GPIOB_Init(void) {
    SCLHandle.pGPIOx = GPIOB;
    SDAHandle.pGPIOx = GPIOB;

    SCLHandle.GPIO_PinConfig.GPIO_PinNumber = GPIO_PIN_NO_8;    // SCL
    SDAHandle.GPIO_PinConfig.GPIO_PinNumber = GPIO_PIN_NO_9;    // SDA

    SCLHandle.GPIO_PinConfig.GPIO_PinMode = GPIO_MODE_ALTFN;
    SDAHandle.GPIO_PinConfig.GPIO_PinMode = GPIO_MODE_ALTFN;

    SCLHandle.GPIO_PinConfig.GPIO_PinOPType = 1; // Open-Drain
    SDAHandle.GPIO_PinConfig.GPIO_PinOPType = 1;

    // Check, not sure
    SCLHandle.GPIO_PinConfig.GPIO_PinSpeed = GPIO_SPEED_HIGH;
    SDAHandle.GPIO_PinConfig.GPIO_PinSpeed = GPIO_SPEED_HIGH;

    // Check, not sure
    SCLHandle.GPIO_PinConfig.GPIO_PinPuPdControl = GPIO_PUPDR_NONE; // Disable pull-up/down
    SDAHandle.GPIO_PinConfig.GPIO_PinPuPdControl = GPIO_PUPDR_NONE;

    SCLHandle.GPIO_PinConfig.GPIO_AltFuncMode = 4;
    SDAHandle.GPIO_PinConfig.GPIO_AltFuncMode = 4;

    GPIO_Init(&SCLHandle);
    GPIO_Init(&SDAHandle);
}

void I2C1_Init(void){
    i2c1Handle.I2C_Config.I2C_ACKControl = I2C_ACK_ENABLE;
    i2c1Handle.I2C_Config.I2C_DeviceAddress = SLAVE_ADDR_LCD;
    i2c1Handle.I2C_Config.I2C_FMDutyCycle = I2C_FM_DUTY_2;
    i2c1Handle.I2C_Config.I2C_SCLSpeed = I2C_SCL_SPEED_SM; // 100kHz

    I2C_Init(&i2c1Handle);
}

int main() {
    // System Clock Config
    SystemClock_Config();
    // Initialize GPIOB
    GPIOB_Init();
    // Initialize I2C1
    I2C1_Init();

    LCD_Init();
//    LCD_Clear();
    LCD_SendString("192adas HEHE");
    LCD_PutCur(1, 0);
    LCD_SendString("STM3ajfsa2F4");
    while(1) {

    }
    return 0;
}
