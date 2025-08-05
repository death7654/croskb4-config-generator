#include "keyboard.h"
#include "wrapper.h"

extern "C" {

VivaldiTesterHandle VivaldiTester_Create() {
    return new VivaldiTester();
}

void VivaldiTester_Destroy(VivaldiTesterHandle tester) {
    delete static_cast<VivaldiTester*>(tester);
}

void VivaldiTester_SubmitKeys(VivaldiTesterHandle tester, KEYBOARD_INPUT_DATA* start, uint32_t count) {
    SubmitKeys_Guarded(static_cast<VivaldiTester*>(tester), start, count);
}

}
