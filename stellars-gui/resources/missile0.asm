        processor 6502
        include "vcs.h"

        seg
        org $F000

reset:  ldx #0
        lda #0

clear:  sta 0,x
        inx
        bne clear

        ; Initializiation
        lda #$96
        sta COLUBK
        lda #$C0
        sta COLUP0
        lda #$40
        sta COLUP1
        lda #%00010110
        sta NUSIZ0
        lda #%00110110
        sta NUSIZ1
        lda #$C0
        sta HMM0
        lda #2
        sta ENAM0
        sta ENAM1

start:  lda #0
        sta VBLANK

        lda #2
        sta VSYNC

        sta WSYNC
        sta WSYNC
        sta WSYNC

        lda #0
        sta VSYNC

        ldx #0
        
vblank: sta WSYNC
        inx         ; 6 CLK
        cpx #37     ; 6 CLK
        bne vblank  ; 6 CLK

        ldx #0      ; 6 CLK
        
pic:    sta WSYNC   ; 9 CLK
        sta HMOVE   ; 9 CLK
        nop         ; 6 CLK
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        nop
        sta RESM0
        sta RESM1
        inx
        cpx #192
        bne pic

        ldx #0
ovscan: sta WSYNC
        inx
        cpx #30
        bne ovscan

        jmp start


        org $FFFA

        .word reset
        .word reset
        .word reset

        end
