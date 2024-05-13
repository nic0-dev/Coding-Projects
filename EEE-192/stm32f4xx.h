#ifndef STM32F4XX_H_
#define STM32F4XX_H_

#include <stdint.h>

#define __vo volatile

// #define FLASH_ADDR  0x08000000U
#define SRAM1_ADDR  0x20000000U	//112KB
#define SRAM2_ADDR  0x20001C00U
#define ROM_ADDR    0x1FFF0000U
#define SRAM        SRAM1_ADDR	

// Bus Peripheral Base Address
#define PERIPH_BASE 0x40000000U
#define APB1_ADDR   PERIPH_BASE
#define APB2_ADDR   0x40010000U
#define AHB1_ADDR   0x40020000U
#define AHB2_ADDR   0x50000000U

// AHB1 bus Peripheral Base Address
#define GPIOA_ADDR  (AHB1_ADDR + 0x0000)
#define GPIOB_ADDR  (AHB1_ADDR + 0x0400)
#define GPIOC_ADDR  (AHB1_ADDR + 0x0800)
#define GPIOD_ADDR  (AHB1_ADDR + 0x0C00)
#define GPIOE_ADDR  (AHB1_ADDR + 0x1000)
#define GPIOH_ADDR  (AHB1_ADDR + 0x1C00)
#define CRC_ADDR    (AHB1_ADDR + 0x3000)
#define RCC_ADDR    (AHB1_ADDR + 0x3800)
#define FLASH_ADDR  (AHB1_ADDR + 0x3C00)

// APB1 bus Peripheral Base Address
#define RTC_ADDR    (APB1_ADDR + 0x2800)
#define SPI2_ADDR   (APB1_ADDR + 0x3800)
#define SPI3_ADDR   (APB1_ADDR + 0x3C00)
#define I2C1_ADDR   (APB1_ADDR + 0x5400)
#define I2C2_ADDR   (APB1_ADDR + 0x5800)
#define I2C3_ADDR   (APB1_ADDR + 0x5C00)
#define PWR_ADDR    (APB1_ADDR + 0x7000)

// APB2 bus Peripheral Base Address
#define USART1_ADDR (APB2_ADDR + 0x1000)
#define USART6_ADDR (APB2_ADDR + 0x1400)
#define SPI1_ADDR   (APB2_ADDR + 0x3000)
#define SPI4_ADDR   (APB2_ADDR + 0x3400)
#define SPI5_ADDR   (APB2_ADDR + 0x5000)
#define SYSCFG_ADDR (APB2_ADDR + 0x3800)
#define EXTI_ADDR   (APB2_ADDR + 0x3C00)

/*FLASH INTERFACE REGISTER*/
typedef struct {
    __vo uint32_t ACR;
    __vo uint32_t KEYR;
    __vo uint32_t OPTKEYR;
    __vo uint32_t SR;
    __vo uint32_t CR;
    __vo uint32_t OPTCR;
} FLASH_RegDef_t;

// Flash Configuration 
#define FLASH_ACR_LATENCY   0
#define FLASH_ACR_PRFTEN    8
#define FLASH_ACR_ICEN      9
#define FLASH_ACR_DCEN      10

/*RCC REGISTER*/
typedef struct{
	__vo uint32_t CR;
	__vo uint32_t PLLCFGR;
	__vo uint32_t CFGR;
	__vo uint32_t CIR;
	__vo uint32_t AHB1RSTR;
	__vo uint32_t AHB2RSTR;
	uint32_t RESERVED0[2];
	__vo uint32_t APB1RSTR;
	__vo uint32_t APB2RSTR;
	uint32_t RESERVED2[2];
	__vo uint32_t AHB1ENR;
	__vo uint32_t AHB2ENR;
	uint32_t RESERVED4[2];
	__vo uint32_t APB1ENR;
	__vo uint32_t APB2ENR;
	uint32_t RESERVED6[2];
	__vo uint32_t AHB1LPENR;
	__vo uint32_t AHB2LPENR;
	uint32_t RESERVED8[2];
	__vo uint32_t APB1LPENR;
	__vo uint32_t APB2LPENR;
	uint32_t RESERVED10[2];
	__vo uint32_t BDCR;
	__vo uint32_t CSR;
	uint32_t RESERVED12[2];
	__vo uint32_t SSCGR;
	__vo uint32_t PLLI2SCFGR;
	__vo uint32_t DCKCFGR;
}RCC_RegDef_t;

// RCC Configuration
#define RCC_CR_HSION    0
#define RCC_CR_HSIRDY   1
#define RCC_CR_HSEON    16
#define RCC_CR_HSERDY   17
#define RCC_CR_HSEBYP   18
#define RCC_CR_CSSON    19
#define RCC_CR_PLLON    24
#define RCC_CR_PLLRDY   25

#define RCC_PLLCFGR_Q   24
#define RCC_PLLCFGR_SRC 22
#define RCC_PLLCFGR_P   16
#define RCC_PLLCFGR_N   6
#define RCC_PLLCFGR_M   0

#define RCC_CFGR_SW     0
#define RCC_CFGR_HPRE   4
#define RCC_CFGR_PPRE1  10
#define RCC_CFGR_PPRE2  13

/*GPIO REGISTER*/
typedef struct {
	__vo uint32_t MODER;
	__vo uint32_t OTYPER;
	__vo uint32_t OSPEEDR;
	__vo uint32_t PUPDR;
	__vo uint32_t IDR;
	__vo uint32_t ODR;
	__vo uint32_t BSRR;
	__vo uint32_t LCKR;
	__vo uint32_t AFR[2];
}GPIO_RegDef_t;

/*I2C REGISTER*/
typedef struct {
	__vo uint32_t CR1;
	__vo uint32_t CR2;
	__vo uint32_t OAR1;
	__vo uint32_t OAR2;
	__vo uint32_t DR;
	__vo uint32_t SR1;
	__vo uint32_t SR2;
	__vo uint32_t CCR;
	__vo uint32_t TRISE;
	__vo uint32_t FLTR;
} I2C_RegDef_t;

// I2C Configuration
#define I2C_CR1_PE          0
#define I2C_CR1_NOSTRETCH   7
#define I2C_CR1_START       8
#define I2C_CR1_STOP        9
#define I2C_CR1_ACK         10
#define I2C_CR1_SWRST       15

#define I2C_CR2_FREQ		0
#define I2C_CR2_ITERREN		8
#define I2C_CR2_ITEVTEN		9
#define I2C_CR2_ITBUFEN		10

#define I2C_SR1_SB			0
#define I2C_SR1_ADDR		1
#define I2C_SR1_BTF			2
#define I2C_SR1_ADD10		3
#define I2C_SR1_STOPF		4
#define I2C_SR1_RXNE		6
#define I2C_SR1_TXE			7
#define I2C_SR1_BERR		8
#define I2C_SR1_ARLO		9
#define I2C_SR1_AF			10
#define I2C_SR1_OVR			11
#define I2C_SR1_TIMEOUT		12

#define I2C_SR2_MSL			0
#define I2C_SR2_BUSY		1
#define I2C_SR2_TRA			2
#define I2C_SR2_GENCALL		4
#define I2C_SR2_DUALF		7

#define I2C_CCR_CCR			0
#define I2C_CCR_DUTY		14
#define I2C_CCR_FS			15

// Peripheral Definitions (Peripheral base addresses type casted to xxx_RegDef_t
#define GPIOA				((GPIO_RegDef_t*)GPIOA_ADDR)
#define GPIOB				((GPIO_RegDef_t*)GPIOB_ADDR)
#define GPIOC				((GPIO_RegDef_t*)GPIOC_ADDR)
#define GPIOD				((GPIO_RegDef_t*)GPIOD_ADDR)
#define GPIOE				((GPIO_RegDef_t*)GPIOE_ADDR)
#define GPIOH				((GPIO_RegDef_t*)GPIOH_ADDR)

#define RCC					((RCC_RegDef_t*)RCC_ADDR)
#define FLASH               ((FLASH_RegDef_t*)FLASH_ADDR)

#define I2C1				((I2C_RegDef_t*)I2C1_ADDR)
#define I2C2				((I2C_RegDef_t*)I2C2_ADDR)
#define I2C3				((I2C_RegDef_t*)I2C3_ADDR)

// Clock Enable Macros for Peripheral Clock

#define GPIOA_PCLK_EN()		(RCC->AHB1ENR |= (1 << 0))
#define GPIOB_PCLK_EN()		(RCC->AHB1ENR |= (1 << 1))
#define GPIOC_PCLK_EN()		(RCC->AHB1ENR |= (1 << 2))
#define GPIOD_PCLK_EN()		(RCC->AHB1ENR |= (1 << 3))
#define GPIOE_PCLK_EN()		(RCC->AHB1ENR |= (1 << 4))
#define GPIOH_PCLK_EN()		(RCC->AHB1ENR |= (1 << 7))

#define I2C1_PCLK_EN()		(RCC->APB1ENR |= (1 << 21))
#define I2C2_PCLK_EN()		(RCC->APB1ENR |= (1 << 22))
#define I2C3_PCLK_EN()		(RCC->APB1ENR |= (1 << 23))

// Clock Disable Macros for Peripheral Clock
#define GPIOA_PCLK_DI() 	(RCC->AHB1ENR &= ~(1 << 0))
#define GPIOB_PCLK_DI()		(RCC->AHB1ENR &= ~(1 << 1))
#define GPIOC_PCLK_DI()		(RCC->AHB1ENR &= ~(1 << 2))
#define GPIOD_PCLK_DI()		(RCC->AHB1ENR &= ~(1 << 3))
#define GPIOE_PCLK_DI()		(RCC->AHB1ENR &= ~(1 << 4))
#define GPIOH_PCLK_DI()		(RCC->AHB1ENR &= ~(1 << 7))

#define I2C1_PCLK_DI()		(RCC->APB1ENR &= ~(1 << 21))
#define I2C2_PCLK_DI()		(RCC->APB2ENR &= ~(1 << 22))
#define I2C3_PCLK_DI()		(RCC->APB2ENR &= ~(1 << 23))

// Reset Peripherals
#define GPIOA_REG_RESET() 	do{ (RCC->AHB1RSTR |= (1 << 0)); (RCC->AHB1RSTR &= ~(1 << 0)); }while(0)
#define GPIOB_REG_RESET()	do{ (RCC->AHB1RSTR |= (1 << 1)); (RCC->AHB1RSTR &= ~(1 << 1)); }while(0)
#define GPIOC_REG_RESET()	do{ (RCC->AHB1RSTR |= (1 << 2)); (RCC->AHB1RSTR &= ~(1 << 2)); }while(0)
#define GPIOD_REG_RESET()	do{ (RCC->AHB1RSTR |= (1 << 3)); (RCC->AHB1RSTR &= ~(1 << 3)); }while(0)
#define GPIOE_REG_RESET()	do{ (RCC->AHB1RSTR |= (1 << 4)); (RCC->AHB1RSTR &= ~(1 << 4)); }while(0)
#define GPIOH_REG_RESET()	do{ (RCC->AHB1RSTR |= (1 << 7)); (RCC->AHB1RSTR &= ~(1 << 7)); }while(0)

#define ENABLE				1
#define DISABLE				0
#define SET					ENABLE
#define RESET				DISABLE
#define GPIO_PIN_SET		SET
#define GPIO_PIN_RESET		RESET
#define FLAG_RESET			RESET
#define FLAG_SET			SET

#include "gpio.h"
#include "i2c.h"

#endif /* STM32F4XX_H_ */