// VivaldiKeyboardTester.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include "keyboard.h"


int main()
{
    VivaldiTester test;

    KEYBOARD_INPUT_DATA testData[2];
    RtlZeroMemory(testData, sizeof(testData)); //Reset test data

    /*testData[0].MakeCode = K_LCTRL;
    printf("Ctrl\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("Ctrl Repeat\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = VIVALDI_MUTE;
    testData[0].Flags = KEY_E0;
    printf("Mute\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].Flags |= KEY_BREAK;
    printf("Mute Release\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LCTRL;
    testData[0].Flags = 0;
    printf("Ctrl Repeat\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].Flags |= KEY_BREAK;
    printf("Ctrl Release\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = VIVALDI_MUTE;
    testData[0].Flags = KEY_E0;
    printf("Mute\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].Flags |= KEY_BREAK;
    printf("Mute Release\n");
    SubmitKeys_Guarded(&test, testData, 1);

    RtlZeroMemory(testData, sizeof(testData)); //Reset test data

    testData[0].MakeCode = 0x1E;
    testData[0].Flags = 0;
    printf("A Press\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = 0x1F;
    testData[0].Flags = 0;
    printf("S Press + A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("S + A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("S + A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("S + A Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = 0x1E;
    testData[0].Flags = KEY_BREAK;
    printf("A Release\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = 0x1F;
    testData[0].Flags = 0;
    printf("S Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    printf("S Hold\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = 0x1F;
    testData[0].Flags = KEY_BREAK;
    printf("S Release\n");
    SubmitKeys_Guarded(&test, testData, 1);

    RtlZeroMemory(testData, sizeof(testData)); //Reset test data

    testData[0].MakeCode = K_LCTRL;
    printf("Ctrl\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LALT;
    printf("Alt\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_BACKSP;
    printf("Backspace\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LCTRL;
    testData[0].Flags = KEY_BREAK;
    printf("Release Ctrl\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LALT;
    testData[0].Flags = KEY_BREAK;
    printf("Release Alt\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_BACKSP;
    testData[0].Flags = KEY_BREAK;
    printf("Release Backspace\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_BACKSP;
    testData[0].Flags = 0;
    printf("Backspace\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_BACKSP;
    testData[0].Flags = KEY_BREAK;
    printf("Release Backspace\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LCTRL;
    testData[0].Flags = 0;
    printf("Ctrl\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = VIVALDI_BRIGHTNESSUP;
    testData[0].Flags = KEY_E0;
    printf("Brightness Up\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = VIVALDI_BRIGHTNESSUP;
    testData[0].Flags = KEY_E0 | KEY_BREAK;
    printf("Release Brightness Up\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = K_LCTRL;
    testData[0].Flags = KEY_BREAK;
    printf("Release Ctrl\n");
    SubmitKeys_Guarded(&test, testData, 1);*/

    testData[0].MakeCode = VIVALDI_VOLUP;
    testData[0].Flags = KEY_E0;
    printf("Volume Up\n");
    SubmitKeys_Guarded(&test, testData, 1);

    testData[0].MakeCode = VIVALDI_VOLUP;
    testData[0].Flags = KEY_E0 | KEY_BREAK;
    printf("Release Volume Up\n");
    SubmitKeys_Guarded(&test, testData, 1);
}