# Based on FreeRTOS-Kernel/examples/cmake_example/CMakeLists.txt

cmake_minimum_required(VERSION 3.15)

project(freertos LANGUAGES C ASM)

set(FREERTOS_KERNEL_PATH "FreeRTOS-Kernel")

# Cross compile
set(CMAKE_C_COMPILER arm-none-eabi-gcc)
set(CMAKE_ASM_COMPILER arm-none-eabi-gcc)

# Pick target
#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mcpu=cortex-m3 -mthumb") # Cortex M3 without FPU
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16") # Cortex M4 with FPU

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} --specs=nosys.specs --specs=nano.specs")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O2 -ggdb")

# Add the freertos_config for FreeRTOS-Kernel
add_library(freertos_config INTERFACE)
target_include_directories(freertos_config INTERFACE ".")

# Select the heap port. values between 1-4 will pick a heap.
# No heap when not set
# set(FREERTOS_HEAP "4" CACHE STRING "" FORCE)

# Select the native compile PORT
#set(FREERTOS_PORT "GCC_ARM_CM3_MPU" CACHE STRING "" FORCE) # Cortex M3 without FPU
set(FREERTOS_PORT "GCC_ARM_CM4_MPU" CACHE STRING "" FORCE) # Cortex M4 with FPU


# Adding the FreeRTOS-Kernel subdirectory
add_subdirectory(${FREERTOS_KERNEL_PATH} FreeRTOS-Kernel)

########################################################################
# Overall Compile Options
# Note the compile option strategy is to error on everything and then
# Per library opt-out of things that are warnings/errors.
# This ensures that no matter what strategy for compilation you take, the
# builds will still occur.
#
# Only tested with GNU and Clang.
# Other options are https://cmake.org/cmake/help/latest/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID
# Naming of compilers translation map:
#
#   FreeRTOS    | CMake
#   -------------------
#   CCS         | ?TBD?
#   GCC         | GNU, Clang, *Clang Others?
#   IAR         | IAR
#   Keil        | ARMCC
#   MSVC        | MSVC # Note only for MinGW?
#   Renesas     | ?TBD?

target_compile_options(freertos_kernel PRIVATE
    ### Gnu/Clang C Options
    $<$<COMPILE_LANG_AND_ID:C,GNU>:-fdiagnostics-color=always>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-fcolor-diagnostics>

    $<$<COMPILE_LANG_AND_ID:C,Clang,GNU>:-Wall>
    $<$<COMPILE_LANG_AND_ID:C,Clang,GNU>:-Wextra>
    $<$<COMPILE_LANG_AND_ID:C,Clang,GNU>:-Wpedantic>
    $<$<COMPILE_LANG_AND_ID:C,Clang,GNU>:-Werror>
    $<$<COMPILE_LANG_AND_ID:C,Clang,GNU>:-Wconversion>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Weverything>

    # Suppressions required to build clean with clang.
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Wno-unused-macros>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Wno-padded>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Wno-missing-variable-declarations>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Wno-covered-switch-default>
    $<$<COMPILE_LANG_AND_ID:C,Clang>:-Wno-cast-align> )
