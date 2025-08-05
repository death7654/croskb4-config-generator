// wrapper.hpp
#pragma once

#include "keyboard.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* VivaldiTesterHandle;

VivaldiTesterHandle VivaldiTester_Create();
void VivaldiTester_Destroy(VivaldiTesterHandle tester);
void VivaldiTester_SubmitKeys(VivaldiTesterHandle tester, KEYBOARD_INPUT_DATA* start, uint32_t count);

#ifdef __cplusplus
}
#endif
