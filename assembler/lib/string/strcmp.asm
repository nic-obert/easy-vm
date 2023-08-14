# strcmp
# Compare two null-terminated strings stored in r1 and r2 registers
# Return 0 if strings are not equal
# Return 1 if strings are equal
# Return value is stored in r1


.text:

@@ strcmp

    # Initialize the character index register
    mov8 r8 0

    # Move the strings into r3 and r4 registers
    mov r3 r1
    mov r4 r2

    # Initialize the return value to 0 (strings are not equal)
    mov1 r7 0

    @ loop

        # Calculate the address of the char of s1
        mov r2 r8
        add

        # Store the char of s1 to compare
        mov1 r5 [r1]

        # Calculate the address of the char from s2
        mov r1 r4
        # r2 is still the char index
        add

        # Deref the char
        mov1 r1 [r1]

        # Compare the chars
        cmp r5 r1
        
        # If the chars are different, return
        jmpnz endloop

        # The chars are equal, check if they are null and finish
        cmp1 r1 0
        jmpz equal

        # If the chars are equal but not null, continue
        inc r8
        jmp loop


    @ equal
        # Set return value to 1
        mov1 r7 1


    @ endloop

    # Set return value
    mov r1 r7

    ret
