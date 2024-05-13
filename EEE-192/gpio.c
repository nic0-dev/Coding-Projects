#include "gpio.h"

// Enables or Disables Peripheral Clock for the given GPIO Port
void GPIO_PeriClkControl(GPIO_RegDef_t *pGPIOx, uint8_t EnOrDi){
	if (EnOrDi == ENABLE){
		if (pGPIOx == GPIOA){
			GPIOA_PCLK_EN();
		} else if (pGPIOx == GPIOB){
			GPIOB_PCLK_EN();
		} else if (pGPIOx == GPIOC){
			GPIOC_PCLK_EN();
		} else if (pGPIOx == GPIOD){
			GPIOD_PCLK_EN();
		} else if (pGPIOx == GPIOE){
			GPIOE_PCLK_EN();
		} else if (pGPIOx == GPIOH){
			GPIOH_PCLK_EN();
		}
	} else{
		if (pGPIOx == GPIOA){
			GPIOA_PCLK_DI();
		} else if (pGPIOx == GPIOB){
			GPIOB_PCLK_DI();
		} else if (pGPIOx == GPIOC){
			GPIOC_PCLK_DI();
		} else if (pGPIOx == GPIOD){
			GPIOD_PCLK_DI();
		} else if (pGPIOx == GPIOE){
			GPIOE_PCLK_DI();
		} else if (pGPIOx == GPIOH){
			GPIOH_PCLK_DI();
		}
	}
}

void GPIO_Init(GPIO_Handle_t *pGPIOHandle){
	// Enable Peripheral Clock
	GPIO_PeriClkControl(pGPIOHandle->pGPIOx, ENABLE);

    uint32_t temp = 0;
    // Pin Mode
    temp = (pGPIOHandle->GPIO_PinConfig.GPIO_PinMode << (2 * pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber));
    pGPIOHandle->pGPIOx->MODER &= ~(0x3 << pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber);     // Clearing
    pGPIOHandle->pGPIOx->MODER |= temp;                                                     // Setting
	temp = 0;
    // Pin Speed
	temp = (pGPIOHandle->GPIO_PinConfig.GPIO_PinSpeed << (2 * pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber));
	pGPIOHandle->pGPIOx->OSPEEDR &= ~(0x3 << pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber);
	pGPIOHandle->pGPIOx->OSPEEDR |= temp;
	temp = 0;
    // Pull Up Pull Down Configuration
	temp = (pGPIOHandle->GPIO_PinConfig.GPIO_PinPuPdControl << (2 * pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber));
	pGPIOHandle->pGPIOx->PUPDR &= ~(0x3 << pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber);
	pGPIOHandle->pGPIOx->PUPDR |= temp;
	temp = 0;
    // Output Type
	temp = (pGPIOHandle->GPIO_PinConfig.GPIO_PinOPType << pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber);
	pGPIOHandle->pGPIOx->OTYPER &= ~(0x1 << pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber);
	pGPIOHandle->pGPIOx->OTYPER |= temp;
	temp = 0;

	if ((pGPIOHandle->GPIO_PinConfig.GPIO_PinMode == GPIO_MODE_ALTFN)) {
		uint8_t idx, bit;
		idx = pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber / 8;
		bit = pGPIOHandle->GPIO_PinConfig.GPIO_PinNumber % 8;

		pGPIOHandle->pGPIOx->AFR[idx] &= ~(0xF << (4 * bit));                                 			// Clearing
		pGPIOHandle->pGPIOx->AFR[idx] |= (pGPIOHandle->GPIO_PinConfig.GPIO_AltFuncMode << (4 * bit));   // Setting
	}
}

void GPIO_DeInit(GPIO_RegDef_t *pGPIOx){
	if(pGPIOx == GPIOA){
		GPIOA_REG_RESET();
	}else if(pGPIOx == GPIOB){
		GPIOB_REG_RESET();
	}else if(pGPIOx == GPIOC){
		GPIOC_REG_RESET();
	}else if(pGPIOx == GPIOD){
		GPIOD_REG_RESET();
	}else if(pGPIOx == GPIOE){
		GPIOE_REG_RESET();
	}else if(pGPIOx == GPIOH){
		GPIOH_REG_RESET();
	}
}

// Data read and write
uint8_t GPIO_ReadFromInputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber){
	uint8_t value;
	value = (uint8_t)((pGPIOx->IDR >> PinNumber) & 0x00000001);
	return value;
}

void GPIO_WriteToOutputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber, uint8_t Value){
	if(Value == GPIO_PIN_SET) {
		// Write 1 to Output Data Register at Bit Field of Pin Number
		pGPIOx->ODR |= (1 << PinNumber);
	} else {
		// Write 0
		pGPIOx->ODR &= ~(1 << PinNumber);
	}
} 

void GPIO_ToggleOutputPin(GPIO_RegDef_t *pGPIOx, uint8_t PinNumber){
	pGPIOx->ODR ^= (1 << PinNumber);
}