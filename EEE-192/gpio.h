#ifndef GPIO_H_
#define GPIO_H_

#include "stm32f4xx.h"

typedef struct{
	uint8_t GPIO_PinNumber;
	uint8_t GPIO_PinMode;
	uint8_t GPIO_PinOPType;
	uint8_t GPIO_PinSpeed;
	uint8_t GPIO_PinPuPdControl;
	uint8_t GPIO_AltFuncMode;
} GPIO_PinConfig_t;

typedef struct{
	// pointer to hold the base address of the GPIO Peripheral
	GPIO_RegDef_t *pGPIOx;
	GPIO_PinConfig_t GPIO_PinConfig;
} GPIO_Handle_t;

//@GPIO Pin Numbers
#define GPIO_PIN_NO_0		0
#define GPIO_PIN_NO_1		1
#define GPIO_PIN_NO_2		2
#define GPIO_PIN_NO_3		3
#define GPIO_PIN_NO_4		4
#define GPIO_PIN_NO_5		5
#define GPIO_PIN_NO_6		6
#define GPIO_PIN_NO_7		7
#define GPIO_PIN_NO_8		8
#define GPIO_PIN_NO_9		9
#define GPIO_PIN_NO_10		10
#define GPIO_PIN_NO_11		11
#define GPIO_PIN_NO_12		12
#define GPIO_PIN_NO_13		13
#define GPIO_PIN_NO_14		14
#define GPIO_PIN_NO_15		15

//@GPIO Pin Possible Modes
#define GPIO_MODE_IN 		0
#define GPIO_MODE_OUT		1
#define GPIO_MODE_ALTFN		2
#define GPIO_MODE_ANALOG	3

// #define GPIO_MODE_IT_FT		4
// #define GPIO_MODE_IT_RT		5
// #define GPIO_MODE_IT_RFT	6

//@GPIO Pin Possible Output Speeds
#define GPIO_SPEED_LOW		0
#define GPIO_SPEED_MEDIUM	1
#define GPIO_SPEED_FAST		2
#define GPIO_SPEED_HIGH		3

//@GPIO Pin Pull Up AND Pull Down Configuration
#define GPIO_PUPDR_NONE		0
#define GPIO_PUPDR_PU		1
#define GPIO_PUPDR_PD		2



/*APIs for GPIO Driver*/

// Peripheral Clock setup
void GPIO_PeriClkControl(GPIO_RegDef_t *pGPIOx, uint8_t EnOrDi);
// Initialize and DeInitialize GPIO
void GPIO_Init(GPIO_Handle_t *pGPIOHandle);
void GPIO_DeInit(GPIO_RegDef_t *pGPIOx);
// Data Read and Write
uint8_t GPIO_ReadFromInputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber);
void GPIO_WriteToOutputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber, uint8_t Value);
void GPIO_ToggleOutputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber);

#endif /* GPIO_H_ */