#ifndef I2C_H_
#define I2C_H_

#include "stm32f4xx.h"

typedef struct {
	uint32_t I2C_SCLSpeed;
	uint8_t I2C_DeviceAddress;
	uint8_t I2C_ACKControl;
	uint16_t I2C_FMDutyCycle;
} I2C_Config_t;

// Handle Structure for I2Cx Peripheral

typedef struct {
	I2C_RegDef_t *pI2Cx;
	I2C_Config_t I2C_Config;
} I2C_Handle_t;

//@I2C_SCLSpeed
#define I2C_SCL_SPEED_SM 	100000
#define I2C_SCL_SPEED_FM4k 	400000
#define I2C_SCL_SPEED_FM2k	200000

//@I2C_ACKCONTROL
#define I2C_ACK_ENABLE		1
#define I2C_ACK_DISABLE		0

//@I2C_
#define I2C_FM_DUTY_2		0
#define I2C_FM_DUTY_16_9	1

// I2C Related status flags
#define I2C_FLAG_SB							(1 << I2C_SR1_SB)
#define I2C_FLAG_ADDR						(1 << I2C_SR1_ADDR)
#define I2C_FLAG_BTF						(1 << I2C_SR1_BTF)
#define I2C_FLAG_ADD10						(1 << I2C_SR1_ADD10)
#define I2C_FLAG_STOPF						(1 << I2C_SR1_STOPF)
#define I2C_FLAG_TXE						(1 << I2C_SR1_TXE)
#define I2C_FLAG_RXNE						(1 << I2C_SR1_RXNE)
#define I2C_FLAG_BERR						(1 << I2C_SR1_BERR)
#define I2C_FLAG_ARLO						(1 << I2C_SR1_ARLO)
#define I2C_FLAG_AF							(1 << I2C_SR1_AF)
#define I2C_FLAG_OVR						(1 << I2C_SR1_OVR)
#define I2C_FLAG_TIMEOUT					(1 << I2C_SR1_TIMEOUT)


/*APIs for I2C Driver*/

// Peripheral Clock setup
void I2C_PeriClkControl(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi);
// Initialize and DeInitialize I2C
void I2C_Init(I2C_Handle_t *pI2CHandle);
void I2C_DeInit(I2C_RegDef_t *pI2Cx);

// Data Send and Receive
void I2C_MasterSendData(I2C_Handle_t *pI2CHandle, uint8_t *pTxBuffer, uint32_t Len, uint8_t SlaveAddr);
void I2C_MasterReceiveData(I2C_Handle_t *pI2CHandle, uint8_t *pRxBuffer, uint8_t Len, uint8_t SlaveAddr);

// Other Peripheral Control APIs
void I2C_PeripheralControl(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi);
uint8_t I2C_GetFlagStatus(I2C_RegDef_t *pI2Cx, uint32_t FlagName);
void I2C_ManageAcking(I2C_RegDef_t *pI2Cx, uint8_t EnOrDi);

// Application Callback
void I2C_ApplicationEventCallback(I2C_Handle_t *pI2CHandle, uint8_t AppEv);

#endif /* I2C_H_ */
