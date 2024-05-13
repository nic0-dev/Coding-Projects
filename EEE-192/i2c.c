#include "i2c.h"

uint16_t AHB_Prescaler[8] = {2, 4, 8, 16, 64, 128, 256, 512};
uint8_t APB1_Prescaler[4] = {2, 4, 8, 16};

void I2C_PeriClkControl(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi) {
    if (EnOrDi == ENABLE) {
		if (pI2Cx == I2C1) {
			I2C1_PCLK_EN();
		} else if (pI2Cx == I2C2) {
			I2C2_PCLK_EN();
		} else if (pI2Cx == I2C3) {
			I2C3_PCLK_EN();
		}
	} else {
		if (pI2Cx == I2C1) {
			I2C1_PCLK_DI();
		} else if (pI2Cx == I2C2) {
			I2C2_PCLK_DI();
		} else if (pI2Cx == I2C3) {
			I2C3_PCLK_DI();
		}
	}
}

uint32_t RCC_PCLK1Value(void) {
    uint32_t pclk1, SystemClk;
    uint8_t clksrc, temp, ahbp, apb1;
    clksrc = ((RCC->CFGR >> 2) & 0x3);
    if (clksrc == 0) {
        SystemClk = 16000000;
    } else if (clksrc == 1) {
        SystemClk = 8000000;
    } else if (clksrc == 2) {
        SystemClk = 72000000;
    }
    // AHB
	temp = ((RCC->CFGR >> RCC_CFGR_HPRE) & 0xF);
	if(temp < 8){
		ahbp = 1;
	} else {
		ahbp = AHB_Prescaler[temp-8];
	}

	// APB1
	temp = ((RCC->CFGR >> RCC_CFGR_PPRE1) & 0x7);
	if (temp < 4){
		apb1 = 1;
	} else {
		apb1 = APB1_Prescaler[temp-4];
	}
    pclk1 = (SystemClk / ahbp ) / apb1;
    return pclk1;
}

void I2C_Init(I2C_Handle_t *pI2CHandle) { // Peripheral must be disabled
	// Enable Peripheral Clock
	GPIO_PeriClkControl(pI2CHandle->pI2Cx, ENABLE);

    // Ack Control 
    uint32_t tempreg = 0;
    tempreg |= (pI2CHandle->I2C_Config.I2C_ACKControl << 10);
    pI2CHandle->pI2Cx->CR1 = tempreg;

    // Configure the speed of SCL
    tempreg = 0;
    tempreg |= RCC_PCLK1Value() / 1000000U;
    pI2CHandle->pI2Cx->CR2 = (tempreg & 0x3F);

	// Program the Device Own Address
	tempreg = 0;
	tempreg |= pI2CHandle->I2C_Config.I2C_DeviceAddress << 1;
	tempreg |= (1 << 14);
	pI2CHandle->pI2Cx->OAR1 = tempreg;

    // Configure CCR
	tempreg = 0;
	uint16_t ccr_value = 0;
	if(pI2CHandle->I2C_Config.I2C_SCLSpeed <= I2C_SCL_SPEED_SM) {
		// standard mode
		ccr_value = (RCC_GetPCLK1Value() / (2 * pI2CHandle->I2C_Config.I2C_SCLSpeed));
		tempreg |= (ccr_value & 0xFFF);
	} else {
		// fast mode
		tempreg |= (1 << 15);
		tempreg |= (pI2CHandle->I2C_Config.I2C_FMDutyCycle << 14);
		if(pI2CHandle->I2C_Config.I2C_FMDutyCycle == I2C_FM_DUTY_2) {
			ccr_value = (RCC_GetPCLK1Value() / (3 * pI2CHandle->I2C_Config.I2C_SCLSpeed));
		} else {
			ccr_value = (RCC_GetPCLK1Value() / (25 * pI2CHandle->I2C_Config.I2C_SCLSpeed));
		}
		tempreg |= (ccr_value & 0xFFF);
	}
	pI2CHandle->pI2Cx->CCR = tempreg;

	// Enable the peripheral
	pI2CHandle->pI2Cx->CR1 |= I2C_CR1_PE;
}


uint8_t I2C_GetFlagStatus(I2C_RegDef_t *pI2Cx, uint32_t FlagName) {
	if(pI2Cx->SR1 & FlagName){
		return FLAG_SET;
	}
	return FLAG_RESET;
}

static void I2C_GenerateStartCondition(I2C_RegDef_t *pI2Cx) {
	pI2Cx->CR1 |= (1 << I2C_CR1_START);
}

static void I2C_ExecuteAddressPhaseWrite(I2C_RegDef_t *pI2Cx, uint8_t SlaveAddr) {
	SlaveAddr = SlaveAddr << 1;
	SlaveAddr &= ~(1);			// SlaveAddr R/W bit + r/w bit = 0
	pI2Cx->DR = SlaveAddr;
}

static void I2C_ExecuteAddressPhaseRead(I2C_RegDef_t *pI2Cx, uint8_t SlaveAddr) {
	SlaveAddr = SlaveAddr << 1;
	SlaveAddr |= 1;			// SlaveAddr R/W bit + r/w bit = 1
	pI2Cx->DR = SlaveAddr;
}

static void I2C_ClearADDRFlag(I2C_RegDef_t *pI2Cx) {
	uint32_t dummyRead = pI2Cx->SR1;
	dummyRead = pI2Cx->SR2;
	(void)dummyRead;
}

static void I2C_GenerateStopCondition(I2C_RegDef_t *pI2Cx) {
	pI2Cx->CR1 |= (1 << I2C_CR1_STOP);
}



void I2C_MasterSendData(I2C_Handle_t *pI2CHandle, uint8_t *pTxBuffer, uint32_t Len, uint8_t SlaveAddr) {
	// Start Condition
	I2C_GenerateStartCondition(pI2CHandle->pI2Cx);
	// Check SB Flag in SR1
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_SB));
	// Send address of the slave with r/w bit set to w(0) (Total 8 bits)
	I2C_ExecuteAddressPhaseWrite(pI2CHandle->pI2Cx, SlaveAddr);
	// Confirm the address phase is completed
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_ADDR));
	// Clear ADDR flag
	I2C_ClearADDRFlag(pI2CHandle->pI2Cx);
	// Send the data until Len = 0
	while(Len > 0) {
		while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_TXE));
		pI2CHandle->pI2Cx->DR = *pTxBuffer;
		pTxBuffer++;
		Len--;
	}
	// wait for TX=1 and BTF=1 before generating STOP condition
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_TXE));
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_BTF));
	// Generate Stop Condition
	I2C_GenerateStopCondition(pI2CHandle->pI2Cx);
}

void I2C_MasterReceiveData(I2C_Handle_t *pI2CHandle, uint8_t *pRxBuffer, uint8_t Len, uint8_t SlaveAddr) {
	// Generate START condition
	I2C_GenerateStartCondition(pI2CHandle->pI2Cx);
	// Check SB Flag in SR1
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_SB));
	// Send address of the slave with r/w bit set to r(1) (Total 8 bits)
	I2C_ExecuteAddressPhaseRead(pI2CHandle->pI2Cx, SlaveAddr);
	// Confirm ADDR flag
	while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_ADDR));
	// Receive the data
	if (Len == 1){
		// Disable Acking
		I2C_ManageAcking(pI2CHandle->pI2Cx, I2C_ACK_DISABLE);
		// Clear ADDR Flag
		I2C_ClearADDRFlag(pI2CHandle->pI2Cx);
		// wait until RXNE becomes 1
		while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_RXNE));
		// Generate STOP Condition
		I2C_GenerateStopCondition(pI2CHandle->pI2Cx);
		// Read data from the buffer
		*pRxBuffer = pI2CHandle->pI2Cx->DR;
	}
	if (Len > 1) {
		// Clear ADDR Flag
		I2C_ClearADDRFlag(pI2CHandle->pI2Cx);
		// Read the data until Len = 0
		for (uint32_t i = Len; i > 0; i--) {
			// wait until RXNE becomes 1
			while(!I2C_GetFlagStatus(pI2CHandle->pI2Cx, I2C_FLAG_RXNE));
			if(i == 2) { // If last 2 bytes are remaining
				// Disable Acking
				I2C_ManageAcking(pI2CHandle->pI2Cx, I2C_ACK_DISABLE);
				// Generate STOP Condition
				I2C_GenerateStopCondition(pI2CHandle->pI2Cx);
			}
			// Read data from the buffer
			*pRxBuffer = pI2CHandle->pI2Cx->DR;
			pRxBuffer++;
		}
	}
	// Re-enable Acking
	if (pI2CHandle->I2C_Config.I2C_ACKControl == I2C_ACK_ENABLE) {
		I2C_ManageAcking(pI2CHandle->pI2Cx, I2C_ACK_ENABLE);
	}
}

void I2C_PeripheralControl(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi) {
	if(EnOrDi == ENABLE) {
		pI2Cx->CR1 |= (1 << I2C_CR1_PE);
	} else {
		pI2Cx->CR1 &= ~(1 << I2C_CR1_PE);
	}
}

void I2C_ManageAcking(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi) {
	if(EnOrDi == I2C_ACK_ENABLE) {
		pI2Cx->CR1 |= (1 << I2C_CR1_ACK);
	} else {
		pI2Cx->CR1 &= ~(1 << I2C_CR1_ACK);
	}
}