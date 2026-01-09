                            * = $0000
0000   A5 89                LDA $89			;Load $0x0089 into A
0002   85 F7                STA $F7			;Store A into $0x00F7
0004   A9 01                LDA #$01		;Load 0x01 into A
0006   85 27                STA VDELBL		;Store A into VDELBL			
0008   85 25                STA VDELP0		;Store A into VDELP0
000A   A6 91                LDX $91			;Load $0x0091 into X
000C   E8                   INX				;Increment X
000D   85 02                STA WSYNC		;Wait for scanline to finish    => 11 scanlines
000F   E8                   INX				;Increment X
0010   86 F6                STX $F6			;Store X into $0x00F6
0012   A2 00                LDX #$00		;Load 0x00 into X
0014   A9 04                LDA #$04		;Load 0x04 into A
0016   86 1B                STX GRP0		;Store X into GRP0
0018   86 1C                STX GRP1		;Store X into GRP1
001A   86 0E                STX PF1			;Store X into PF1
001C   86 0F                STX PF2			;Store X into PF2
001E   86 2C                STX CXCLR		;Store X into CXCLR
0020   85 D7                STA $D7			;Store A into $0x00D7
0022   A5 85                LDA $85			;Load $0x0085 into A
0024   85 D5                STA $D5			;Store A into $0x00D5
0026   A5 86                LDA $86			;Load $0x0086 into A
0028   85 D6                STA $D6			;Store A into $0x00D6
002A   A5 87                LDA $87			;Load $0x0087 into A
002C   85 D8                STA $D8			;Store A into $0x00D8
002E   A5 88                LDA $88			;Load $0x0088 into A
0030   85 D9                STA $D9			;Store A into $0x00D9
0032   A5 DB                LDA $DB			;Load $0x00DB into A
0034   85 D4                STA $D4			;Store A into $0x00D4
0036   4C 5B F0             JMP $F05B

0039   EA          L0039    NOP
003A   A9 00                LDA #$00        ;Load 0x00 into A
003C   4C 65 F0             JMP $F065

003F   EA          L003F    NOP
0040   A9 00                LDA #$00
0042   4C 8D F0             JMP $F08D

0045   B4 A4                LDY $A4,X       ;Load ($0x00A4 + X) into Y
0047   84 0E                STY PF1         ;Store Y into PF1
0049   B4 A5                LDY $A5,X       ;Load ($0x00A5 + X) into Y
004B   84 0F                STY PF2         ;Store Y into PF2
004D   B4 A7                LDY $A7,X       ;Load ($0x00A7 + X) into Y
004F   84 0E                STY PF1         ;Store Y into PF1
0051   B4 A6                LDY $A6,X       ;Load ($0x00A6 + X) into Y
0053   84 0F                STY PF2         ;Store Y into PF2
0055   C7 89                DCP $89         ;Decrement value in $0x0089 and compare the result to A
0057   2A                   ROL A           ;Rotate 1 bit of A left
0058   2A                   ROL A           ;Rotate 1 bit of A left
0059   85 1F                STA ENABL       ;Store A into ENABL

005B   A5 8E                LDA $8E         ;Load $0x008E into A
005D   C7 85                DCP $85         ;Decrement value in $0x0085 and compare the result to A
005F   90 D8                BCC L0039       ;Branch on C = 0
0061   A4 85                LDY $85         ;Load $0x0085 into Y
0063   B1 8A                LDA ($8A),Y     ;Load (address located at $0x008A) + Y into A
0065   85 1B                STA GRP0        ;Store A into GRP0
0067   A5 90                LDA $90         ;Load $0x0090 into A
0069   C7 87                DCP $87         ;Decrement value in $0x0087 and compare the result to A
006B   2A                   ROL A           ;Rotate 1 bit of A left
006C   2A                   ROL A           ;Rotate 1 bit of A left
006D   85 1D                STA ENAM0       ;Store A into ENAM0
006F   B5 A4                LDA $A4,X       ;Load $(0x00A4 + X) into A
0071   85 0E                STA PF1         ;Store A into PF1
0073   B5 A5                LDA $A5,X       ;Load $(0x00A5 + X) into A
0075   85 0F                STA PF2         ;Store A into PF2
0077   B5 A7                LDA $A7,X       ;Load $(0x00A7 + X) into A
0079   85 0E                STA PF1         ;Store A into PF1
007B   B5 A6                LDA $A6,X       ;Load$(0x00A6 + X) into A
007D   85 0F                STA PF2         ;Store A into PF2
007F   C6 D4                DEC $D4         ;Decrement value in $0x00D4
0081   F0 19                BEQ L009C       ;Branch if Z = 1
0083   A5 8F                LDA $8F         ;Load $0x008F into A
0085   C7 86                DCP $86         ;Decrement value in $0x0086 and compare the result to A
0087   90 B6                BCC L003F       ;Branch on C = 0
0089   A4 86                LDY $86         ;Load $0x0086 into Y
008B   B1 8C                LDA ($8C),Y     ;Load (address located at $0x008C) + Y into A
008D   85 1C                STA GRP1        ;Store A into GRP1
008F   A5 91                LDA $91         ;Load $0x0091 into A
0091   C7 88                DCP $88         ;Decrement value in $0x0088 and compare the result to A
0093   E5 F6                SBC $F6         ;A = A - $0x00F6 - !C
0095   85 1E                STA ENAM1       ;Store A into ENAM1
0097   A5 92                LDA $92         ;Load $0x0092 into A
0099   4C 45 F0             JMP $F045

009C   A5 8F      L009C     LDA $8F         ;Load $0x008F into A
009E   C7 86                DCP $86         ;Decrement value in $0x0086 and compare the result to A
00A0   90 33                BCC L00D5       ;Branch C = 0
00A2   A4 86                LDY $86         ;Load $0x0086 into Y
00A4   B1 8C                LDA ($8C),Y     ;Load (address located at $0x008C) + Y into A
00A6   85 1C                STA GRP1        ;Store A into GRP1
00A8   A5 91                LDA $91         ;Load $0x0091 into A
00AA   C7 88                DCP $88         ;Decrement value in $0x0088 and compare the result to A
00AC   E5 F6                SBC $F6         ;A = A - $0x00F6 - !C
00AE   85 1E                STA ENAM1       ;Store A into ENAM1
00B0   A9 00                LDA #$00        ;Load 0x00 into A
00B2   85 0E                STA PF1         ;Store A into PF1
00B4   85 0F                STA PF2         ;Store A into PF2
00B6   8A                   TXA             ;Transfer X into A
00B7   69 04                ADC #$04        ;A = A + 0x04 + C
00B9   AA                   TAX             ;Transfer A into X
00BA   C9 2C                CMP #$2C        ;Compare A with 0xAC
00BC   D0 0C                BNE L00CA       ;Branch on Z = 0
00BE   A9 09                LDA #$09        ;Load 0x09 into A
00C0   E5 DB                SBC L00DB       ;A = A - $0x00DB - !C
00C2   85 D7                STA $D7         ;Store A into $0x00D7
00C4   AD 92 00             LDA $0092       ;Load $0x0092 into A
00C7   4C F1 F0             JMP $F0F1

00CA   85 D7      L00CA     STA $D7         ;Store A into $0x00D7
00CC   A9 08                LDA #$08        ;Load 0x08 into A
00CE   85 D4                STA $D4         ;Store A into $0x00D4
00D0   A5 92                LDA $92         ;Load $0x0092 into A
00D2   4C 55 F0             JMP $F055

00D5   EA         L00D5     NOP
00D6   A9 00                LDA #$00
00D8   4C A6 F0             JMP $F0A6

00DB   EA         L00DB     NOP
00DC   A9 00                LDA #$00        ;Load 0x00 into A
00DE   4C 01 F1             JMP $F101

00E1   B4 A4                LDY $A4,X
00E3   84 0E                STY $0E
00E5   B4 A5                LDY $A5,X
00E7   84 0F                STY $0F
00E9   B4 A7                LDY $A7,X
00EB   84 0E                STY $0E
00ED   B4 A6                LDY $A6,X
00EF   84 0F                STY $0F

00F1   C7 89                DCP $89         ;Decrement value in $0x0089 and compare the result to A
00F3   2A                   ROL A           ;Rotate 1 bit of A left
00F4   2A                   ROL A           ;Rotate 1 bit of A left
00F5   85 1F                STA ENABL       ;Store A into ENABL
00F7   A5 8E                LDA $8E         ;Load $0x008E into A
00F9   C7 85                DCP $85         ;Decrement value in $0x0085 and compare the result to A
00FB   90 DE                BCC L00DB       ;Branch on C = 0
00FD   A4 85                LDY $85         ;Load $0x0085 into Y
00FF   B1 8A                LDA ($8A),Y     ;Load (address located at $0x008A) + Y into A
0101   85 1B                STA GRP0        ;Store A into GRP0
0103   A5 90                LDA $90         ;Load $0x0090 into A
0105   C7 87                DCP $87         ;Decrement value in $0x0087 and compare the result to A
0107   C6 D7                DEC $D7         ;Decrement value at $0x00D7
0109   F0 33                BEQ L013E       ;Branch on Z = 1
010B   B4 A4                LDY $A4,X
010D   84 0E                STY PF1
010F   B4 A5                LDY $A5,X
0111   84 0F                STY PF2
0113   B4 A7                LDY $A7,X
0115   84 0E                STY PF1
0117   B4 A6                LDY $A6,X
0119   84 0F                STY PF2
011B   2A                   ROL A
011C   2A                   ROL A
011D   85 1D                STA ENAM0
011F   A5 8F                LDA $8F
0121   C7 86                DCP $86
0123   90 13                BCC L0138
0125   A4 86                LDY $86
0127   B1 8C                LDA ($8C),Y
0129   85 1C                STA GRP1
012B   A5 91                LDA $91
012D   C7 88                DCP $88
012F   E5 F6                SBC $F6
0131   85 1E                STA ENAM1
0133   A5 92                LDA $92
0135   4C E1 F0             JMP $F0E1

0138   EA         L0138     NOP
0139   A9 00                LDA #$00
013B   4C 29 F1             JMP $F129

013E   A9 00      L013E     LDA #$00        ;Load 0x00 into A
0140   85 0E                STA PF1         ;Store A into PF1
0142   85 0F                STA PF2         ;Store A into PF2
0144   85 02                STA WSYNC       ;Wait for next scanline
0146   85 0B                STA REFP0       ;Store A into REFP0
0148   85 0C                STA REFP1       ;Store A into REFP1
014A   85 1B                STA GRP0        ;Store A into GRP0
014C   85 1C                STA GRP1        ;Store A into GRP1
014E   85 1D                STA ENAM0       ;Store A into ENAM0
0150   85 1E                STA ENAM1       ;Store A into ENAM1
0152   85 1F                STA ENABL       ;Store A into ENABL
0154   85 02                STA WSYNC       ;Wait for next scanline
0156   EA                   NOP
0157   85 1B                STA GRP0        ;Store A into GRP0
0159   85 1C                STA GRP1        ;Store A into GRP1
015B   A0 07                LDY #$07        ;Load 0x07 into Y
015D   84 D4                STY $D4         ;Store Y into $0x00D4
015F   A9 03                LDA #$03        ;Load 0x03 into A
0161   85 04                STA NUSIZ0      ;Store A into NUSIZ0
0163   85 05                STA NUSIZ1      ;Store A into NUSIZ1
0165   85 25                STA VDELP0      ;Stora A into VDELP0
0167   85 26                STA VDELP1      ;Store A into VDELP1
0169   A9 F0                LDA #$F0        ;Load 0xF0 into A
016B   85 20                STA HMP0        ;Store A into HMP0
016D   A5 A3                LDA $A3         ;Load $0x00A3 into A
016F   EA                   NOP
0170   85 10                STA RESP0       ;Store A into RESP0
0172   85 11                STA RESP1       ;Store A into RESP1
0174   85 06                STA COLUP0      ;Store A into COLUP0
0176   85 07                STA COLUP1      ;Store A into COLUP1
0178   85 02                STA WSYNC       ;Wait for next scanline
017A   85 2A                STA HMOVE       ;Move all objects
017C   A4 D4      L017C     LDY $D4         ;Load $0x00D4 into Y
017E   B1 96                LDA ($96),Y     ;Load (address located at $0x0096) + Y into A
0180   85 1B                STA GRP0        ;Store A into GRP0
0182   85 02                STA WSYNC       ;Wait for next scanline
0184   B1 98                LDA ($98),Y     ;Load (address located at $0x0098) + Y into A
0186   85 1C                STA GRP1        ;Store A into GRP1
0188   B1 9A                LDA ($9A),Y     ;Load (address located at $0x009A) + Y into A
018A   85 1B                STA GRP0        ;Store A into GRP0
018C   B1 9C                LDA (L009C),Y   ;Load (address located at $0x009C) + Y into A
018E   85 D7                STA $D7         ;Store A into 0xD7
0190   B1 9E                LDA ($9E),Y     ;Load (address located at $0x009E) + Y into A
0192   AA                   TAX             ;Transfer A into X
0193   B1 A0                LDA ($A0),Y     ;Load (address located at $0x00A0) + Y into A
0195   A8                   TAY             ;Transfer A into Y
0196   A5 D7                LDA $D7         ;Load $0x00D7 into A
0198   85 1C                STA GRP1        ;Store A into GRP1
019A   86 1B                STX GRP0        ;Store A into GRP0
019C   84 1C                STY GRP1        ;Store Y into GRP1
019E   85 1B                STA GRP0        ;Store A into GRP0
01A0   C6 D4                DEC $D4         ;Decrement value at $0x00D4
01A2   10 D8                BPL L017C       ;Branch N = 0
01A4   85 02                STA WSYNC       ;Wait for next scanline
01A6   A9 00                LDA #$00        ;Load 0x00 into A
01A8   85 1B                STA GRP0        ;Store A into GRP0
01AA   85 1C                STA GRP1        ;Store A into GRP1
01AC   85 25                STA VDELP0      ;Store A into VDELP0
01AE   85 26                STA VDELP1      ;Store A into VDELP1
01B0   85 04                STA NUSIZ0      ;Store A into NUSIZ0
01B2   85 05                STA NUSIZ1      ;Store A into NUSIZ1
01B4   A5 D5                LDA $D5         ;Load $0x00D5 into A
01B6   85 85                STA $85         ;Store A into $0x0085
01B8   A5 D6                LDA $D6         ;Load $0x00D6 into A
01BA   85 86                STA $86         ;Store A into $0x0086
01BC   A5 D8                LDA $D8         ;Load $0x00D8 into A
01BE   85 87                STA $87         ;Store A into $0x0087
01C0   A5 D9                LDA $D9         ;Load $0x00D9 into A
01C2   85 88                STA $88         ;Store A into $0x0088
01C4   A5 F7                LDA $F7         ;Load $0x00F7 into A
01C6   85 89                STA $89         ;Store A into $0x0089
01C8   A9 2B                LDA #$2B        ;Load 0x2B into A
01CA   8D 96 02             STA TIM64T      ;Store A into TIM64T
01CD   A9 42                LDA #$42        ;Load 0x42 into A
01CF   85 02                STA WSYNC       ;Wait for next scanline
01D1   85 01                STA VBLANK      ;Enable VBLANK
01D3   60                   RTS


01D4   78                   SEI				;Set Interrupt Enable
01D5   D8                   CLD				;Clear Decimal Flag
01D6   A2 00                LDX #$00		;Load 0x00 into X
01D8   8A                   TXA				;Transfer X into A

/* Reset memory between 0x00 and 0xFF */
01D9   E8         res_stack INX				;Increment X
01DA   9A                   TXS				;Transfer X to SP
01DB   48                   PHA				;Push A to stack
01DC   D0 FB                BNE res_stack

01DE   A9 08                LDA #$08		;Load 0x08 into A
01E0   85 DB                STA $DB			;Store 0x08 in $0x00DB
01E2   A2 0B                LDX #$0B		;Load 0x0B into X

/*
    This setup a portion of the RAM from 0x96 to 0xA1 values alternating between 0xFF and 0xAC, that is:
    - 0xA1 = 0xFF
    - 0xA0 = 0xAC
    - 0x9F = 0xFF
    - 0x9E = 0xAC
    - etc...
*/
01E4   A9 FF      L01E4     LDA #$FF		;Load 0xFF into A
01E6   95 96                STA $96,X		;Store A into $(0x0096 + X)
01E8   CA                   DEX				;Decrement X
01E9   A9 AC                LDA #$AC		;Load 0xAC into A
01EB   95 96                STA $96,X		;Store 0xAC into $(0x0096 + X)
01ED   CA                   DEX				;Decrement X
01EE   10 F4                BPL L01E4		;Branch if X is still positive

01F0   A9 01                LDA #$01		;Load 0x01 into A
01F2   85 0A                STA CTRLPF		;Store 0x01 into CTRLPF
01F4   0D 84 02             ORA INTIM		;OR 0x01 with INTIM
01F7   85 A2                STA $A2			;Store A into $0x00A2
01F9   4C 1B F4             JMP $F41B

; Coming from 0xF218
01FC   86 D5                STX $D5			;Store X into $0x00D5
01FE   AA                   TAX				;Transfer A into X   	
01FF   4A                   LSR A			;Shift A 1 bit to right	
0200   4A                   LSR A			;Shift A 1 bit to right	
0201   4A                   LSR A			;Shift A 1 bit to right	
0202   85 D4                STA $D4			;Store A into 0x00D4
0204   98                   TYA				;Transfer Y to A			
0205   0A                   ASL A			;Shift A 1 bit to left	
0206   0A                   ASL A			;Shift A 1 bit to left	
0207   18                   CLC				;Clear Carry
0208   65 D4                ADC $D4			;A = A + $0x00D4 + C
020A   A8                   TAY				;Transfer A to Y
020B   A5 D5                LDA $D5			;Load $0x00D5 into A
020D   60                   RTS				

020E   20 FC F1             JSR $F1FC
0211   BD 76 F2             LDA $F276,X
0214   39 A4 00             AND $00A4,Y
0217   60                   RTS

; Coming from 0xF43C
0218   20 FC F1             JSR $F1FC
021B   4C 4F F2             JMP $F24F

021E   20 FC F1             JSR $F1FC
0221   4C 2B F2             JMP $F22B
0224   E8         L0224     INX
0225   8A                   TXA
0226   29 07                AND #$07
0228   D0 01                BNE L022B
022A   C8                   INY
022B   20 4F F2   L022B     JSR $F24F
022E   E4 D6                CPX $D6
0230   30 F2                BMI L0224
0232   60                   RTS

0233   20 FC F1             JSR $F1FC
0236   84 D4                STY $D4
0238   E6 D6                INC $D6
023A   A5 D6                LDA $D6
023C   0A                   ASL A
023D   0A                   ASL A
023E   18                   CLC
023F   65 D4                ADC $D4
0241   85 D6                STA $D6
0243   20 4F F2   L0243     JSR $F24F
0246   C8                   INY
0247   C8                   INY
0248   C8                   INY
0249   C8                   INY
024A   C4 D6                CPY $D6
024C   30 F5                BMI L0243
024E   60                   RTS

024F   A5 D5                LDA $D5			;Load $0x00D5 into A
0251   F0 0D                BEQ L0260		;Branch when Z = 1
0253   4A                   LSR A
0254   B0 14                BCS L026A
0256   B9 A4 00             LDA $00A4,Y
0259   5D 76 F2             EOR $F276,X
025C   99 A4 00             STA $00A4,Y
025F   60                   RTS

0260   B9 A4 00   L0260     LDA $00A4,Y		;Load $(0x00A4 + Y) into A
0263   1D 76 F2             ORA $F276,X     ;A = A | $(0xF276 + X)
0266   99 A4 00             STA $00A4,Y     ;Store A into $(0x00A4 + Y)
0269   60                   RTS

026A   BD 76 F2   L026A     LDA $F276,X
026D   49 FF                EOR #$FF
026F   39 A4 00             AND $00A4,Y
0272   99 A4 00             STA $00A4,Y
0275   60                   RTS

0276   80                   ???                ;%10000000
0277   40                   RTI
0278   20 10 08             JSR $0810
027B   04                   ???                ;%00000100
027C   02                   ???                ;%00000010
027D   01 01                ORA ($01,X)
027F   02                   ???                ;%00000010
0280   04                   ???                ;%00000100
0281   08                   PHP
0282   10 20                BPL L02A4
0284   40                   RTI
0285   80                   ???                ;%10000000
0286   80                   ???                ;%10000000
0287   40                   RTI
0288   20 10 08             JSR $0810
028B   04                   ???                ;%00000100
028C   02                   ???                ;%00000010
028D   01 01                ORA ($01,X)
028F   02                   ???                ;%00000010
0290   04                   ???                ;%00000100
0291   08                   PHP
0292   10 20                BPL L02B4
0294   40                   RTI
0295   80                   ???                ;%10000000
0296   D0 14                BNE L02AC
0298   A2 30                LDX #$30
029A   B5 A3      L029A     LDA $A3,X
029C   4A                   LSR A
029D   36 A2                ROL $A2,X
029F   76 A1                ROR $A1,X
02A1   36 A0                ROL $A0,X
02A3   76 A3                ROR $A3,X
02A5   CA                   DEX
02A6   CA                   DEX
02A7   CA                   DEX
02A8   CA                   DEX
02A9   D0 EF                BNE L029A
02AB   60                   RTS
02AC   4A         L02AC     LSR A
02AD   90 13                BCC L02C2
02AF   A2 30                LDX #$30
02B1   B5 A0                LDA $A0,X
02B3   4A                   LSR A
02B4   36 A1      L02B4     ROL $A1,X
02B6   76 A2                ROR $A2,X
02B8   36 A3                ROL $A3,X
02BA   76 A0                ROR $A0,X
02BC   8A                   TXA
02BD   CA                   DEX
02BE   CA                   DEX
02BF   CA                   DEX
02C0   CA                   DEX
02C1   60                   RTS
02C2   4A         L02C2     LSR A
02C3   90 33                BCC L02F8
02C5   C6 DB                DEC $DB
02C7   D0 65                BNE L032E
02C9   A9 08                LDA #$08
02CB   85 DB                STA $DB
02CD   A2 04                LDX #$04
02CF   B5 A3      L02CF     LDA $A3,X
02D1   95 D3                STA $D3,X
02D3   CA                   DEX
02D4   D0 F9                BNE L02CF
02D6   B5 A8      L02D6     LDA $A8,X
02D8   95 A4                STA $A4,X
02DA   B5 A9                LDA $A9,X
02DC   95 A5                STA $A5,X
02DE   B5 AA                LDA $AA,X
02E0   95 A6                STA $A6,X
02E2   B5 AB                LDA $AB,X
02E4   95 A7                STA $A7,X
02E6   E8                   INX
02E7   E8                   INX
02E8   E8                   INX
02E9   E8                   INX
02EA   E0 2C                CPX #$2C
02EC   D0 E8                BNE L02D6
02EE   A2 04                LDX #$04
02F0   B5 D3      L02F0     LDA $D3,X
02F2   95 CF                STA $CF,X
02F4   CA                   DEX
02F5   D0 F9                BNE L02F0
02F7   60                   RTS

02F8   E6 DB      L02F8     INC $DB
02FA   A5 DB                LDA $DB
02FC   C9 09                CMP #$09
02FE   D0 2E                BNE L032E
0300   A9 01                LDA #$01
0302   85 DB                STA $DB
0304   A2 04                LDX #$04
0306   B5 CF      L0306     LDA $CF,X
0308   95 D3                STA $D3,X
030A   CA                   DEX
030B   D0 F9                BNE L0306
030D   A2 2C                LDX #$2C
030F   B5 A3      L030F     LDA $A3,X
0311   95 A7                STA $A7,X
0313   B5 A2                LDA $A2,X
0315   95 A6                STA $A6,X
0317   B5 A1                LDA $A1,X
0319   95 A5                STA $A5,X
031B   B5 A0                LDA $A0,X
031D   95 A4                STA $A4,X
031F   CA                   DEX
0320   CA                   DEX
0321   CA                   DEX
0322   CA                   DEX
0323   D0 EA                BNE L030F
0325   A2 04                LDX #$04
0327   B5 D3      L0327     LDA $D3,X
0329   95 A3                STA $A3,X
032B   CA                   DEX
032C   D0 F9                BNE L0327
032E   60         L032E     RTS

032F   A9 01                LDA #$01
0331   2C 82 02             BIT SWCHB
0334   60                   RTS

0335   A9 02                LDA #$02
0337   2C 82 02             BIT SWCHB
033A   60                   RTS

033B   A9 40                LDA #$40
033D   2C 82 02             BIT SWCHB
0340   60                   RTS

0341   A9 80                LDA #$80
0343   2C 82 02             BIT SWCHB
0346   60                   RTS

0347   A9 08                LDA #$08
0349   2C 82 02             BIT SWCHB
034C   60                   RTS

034D   A9 10                LDA #$10
034F   2C 80 02             BIT SWCHA
0352   60                   RTS

0353   A9 20                LDA #$20
0355   2C 80 02             BIT SWCHA
0358   60                   RTS

0359   A9 40                LDA #$40
035B   2C 80 02             BIT SWCHA
035E   60                   RTS

035F   A9 80                LDA #$80
0361   2C 80 02             BIT SWCHA
0364   60                   RTS

0365   A9 80                LDA #$80
0367   24 0C                BIT INPT4
0369   60                   RTS

/* HORIZONTAL MOTION REGISTERS SETUP */
/*
	This routine works by computing the new value of the HM[P0,P1,M0,M1,BL] registers in A and then shifting the 4
	rightmost bits to the leftmost bits.
	Y is never used here even though it is being set before every call to this routine.
	It uses two scanlines to perform this operation.
*/
036A   38                   SEC				;Set Carry
036B   85 02                STA WSYNC		;Wait for scanline to finish
036D   E9 0F      L036D     SBC #$0F		;Subtract with carry				e.g: A=0x00 - 0x0F - 0 -> A=0xF1, C=1
036F   B0 FC                BCS L036D		;Until carry flag is set
0371   49 07                EOR #$07		;Exclusive OR Memory with A		    e.g: A=0xF1 ^ 0x07 -> A=0xF6
0373   0A                   ASL A			;Shift A 1 bit left				    e.g: A=0xF6 << 1 −> A=0xEC, C=1
0374   0A                   ASL A			;Shift A 1 bit left				    e.g: A=0xEC << 1 -> A=0xD8, C=1
0375   0A                   ASL A			;Shift A 1 bit left				    e.g: A=0xD8 << 1 -> A=0xB0, C=1
0376   0A                   ASL A			;Shift A 1 bit left				    e.g: A=0xB0 << 1 -> A=0x60, C=1
0377   95 10                STA $10,X		;Store A into RES[P0,P1,M0,M1,BL]
0379   95 20                STA $20,X		;Store A into HM[P0,P1,M0,M1,BL]
037B   85 02                STA WSYNC		;Wait for scanline to finish
037D   85 2A                STA HMOVE		;Execute objects movement
037F   EA                   NOP
0380   EA                   NOP
0381   EA                   NOP
0382   EA                   NOP
0383   EA                   NOP
0384   EA                   NOP
0385   EA                   NOP
0386   EA                   NOP
0387   EA                   NOP
0388   EA                   NOP
0389   EA                   NOP
038A   EA                   NOP
038B   85 2B                STA HMCLR		;Clear all horizontal motion registers
038D   60                   RTS

038E   20 6A F3             JSR $F36A
0391   94 85                STY $85,X		;Store Y into $(0x0085 + X)
0393   60                   RTS

0394   A5 A2                LDA $A2
0396   4A                   LSR A
0397   90 02                BCC L039B
0399   49 B2                EOR #$B2
039B   85 A2      L039B     STA $A2
039D   60                   RTS

/*
    The routine seems to set specific addresses of the RAM to specific values using A.
    At the beginning because the RAM is almost entirely empty (so full of 0x00), this does not change anything since
    the values have already been setup (0xAC) by the routine at 0xF1E4.
*/
039E   AA                   TAX				;Transfer A into X
039F   29 0F                AND #$0F		;A = A & 0x0F
03A1   0A                   ASL A			;Shift A 1 bit to left
03A2   0A                   ASL A			;Shift A 1 bit to left
03A3   0A                   ASL A			;Shift A 1 bit to left
03A4   69 AC                ADC #$AC		;Add Memory to A with C
03A6   A8                   TAY				;Transfer A into Y
03A7   8A                   TXA				;Transfer X into A
03A8   29 F0                AND #$F0		;A = A & 0xF0
03AA   4A                   LSR A			;Shift A 1 bit to right
03AB   69 AC                ADC #$AC		;Add Memory to A with C
03AD   AA                   TAX				;Transfer A into X
03AE   60                   RTS

03AF   AD 84 02   chkTimer  LDA INTIM		;Load INTIM value into A
03B2   D0 FB                BNE chkTimer	;Branch if Z = 0

/* VSYNC HANDLING */
03B4   A9 02                LDA #$02		;Load 0x02 into A
03B6   85 02                STA WSYNC		;Wait for scanline to finish
03B8   85 00                STA VSYNC		;Enable VSYNC
03BA   85 02                STA WSYNC		;Wait for scanline to finish
03BC   85 02                STA WSYNC		;Wait for scanline to finish
03BE   A9 00                LDA #$00		;Load 0x00 into A
03C0   85 02                STA WSYNC		;Wait for scanline to finish
03C2   85 00                STA VSYNC		;Disable VSYNC

/* VBLANK HANDLING */
/*
    At the beginning most of the RAM has been reset to 0x00 and all the address that are accessed in here are basically
    reading 0, hence after setting the horizontal motion registers, some values in the are left unchanged
    Then...
*/
03C4   85 01                STA VBLANK		;Disable VBLANK
03C6   A9 25                LDA #$25		;Load 0x25 into A
03C8   8D 96 02             STA TIM64T		;Store 0x25 into TIM64T
03CB   A5 80                LDA $80			;Load $0x80 into A
03CD   A2 00                LDX #$00		;Load 0x00 into X
03CF   A4 85                LDY $85			;Load $0x85 into Y
03D1   20 8E F3             JSR $F38E		;Setup RESP0, HMP0      => 2 scanlines
03D4   A5 81                LDA $81
03D6   A2 01                LDX #$01
03D8   A4 86                LDY $86
03DA   20 8E F3             JSR $F38E		;Setup RESP1, HMP1      => 4 scanlines
03DD   A5 82                LDA $82
03DF   A2 02                LDX #$02
03E1   A4 87                LDY $87
03E3   20 8E F3             JSR $F38E		;Setup RESM0, HMM0      => 6 scanlines
03E6   A5 83                LDA $83
03E8   A2 03                LDX #$03
03EA   A4 88                LDY $88
03EC   20 8E F3             JSR $F38E		;Setup RESM1, HMM1      => 8 scanlines
03EF   A5 84                LDA $84
03F1   A2 04                LDX #$04
03F3   A4 89                LDY $89         ;Load $0x0089 into Y
03F5   20 8E F3             JSR $F38E		;Setup RESBL, HMBL      => 10 scanlines
03F8   A5 95                LDA $95			;Load $0x0095 into A
03FA   20 9E F3             JSR $F39E		
03FD   84 A0                STY $A0 		;Store Y into $0x00A0
03FF   86 9E                STX $9E			;Store X into $0x009E
0401   A5 94                LDA $94			;Load $0x0094 into A
0403   20 9E F3             JSR $F39E
0406   84 9C                STY L009C		;Store Y into $0x009C
0408   86 9A                STX $9A			;Store X into $0x009A
040A   A5 93                LDA $93			;Load $0x0093 into A
040C   20 9E F3             JSR $F39E
040F   84 98                STY $98			;Store Y into $0x0098
0411   86 96                STX $96			;Store X into $0x0096
0413   AD 84 02   chkTimer  LDA INTIM		;Load INTIM into A
0416   D0 FB                BNE chkTimer	;Branch if Z = 0
0418   4C 00 F0             JMP $F000

/*
    Setting up some registers and some values in the RAM
    I think that some of this code has been generated by the assembler because it seems to be a complicated way
    to load values in the RAM...
*/
041B   A9 0A                LDA #$0A		;Load 0x0A into A
041D   85 E0                STA $E0			;Store A into 0x00E0
041F   A5 E0                LDA $E0			;Load 0x0A into A
0421   38                   SEC				;Set Carry Flag
0422   E9 01                SBC #$01		;A = A - 0x01 - !C
0424   85 E0                STA $E0			;Store A into 0x00E0
0426   A9 0C                LDA #$0C			
0428   85 19                STA AUDV0		;Store 0x0C into AUDV0 
042A   A9 06                LDA #$06			
042C   85 15                STA AUDC0		;Store 0x06 into AUDC0 
042E   A9 0B                LDA #$0B
0430   85 17                STA AUDF0		;Store 0x0B into AUDF0
0432   A9 82                LDA #$82
0434   85 08                STA COLUPF		;Store 0x82 into COLUPF
0436   A9 10                LDA #$10		;Load 0x10 into A
0438   A0 00                LDY #$00		;Load 0x00 into Y
043A   A2 00                LDX #$00		;Load 0x00 into X
043C   20 18 F2             JSR $F218
043F   A9 11                LDA #$11		;Load 0x11 into A
0441   A0 00                LDY #$00		;Load 0x00 into Y
0443   A2 00                LDX #$00		;Load 0x00 into X
0445   20 18 F2             JSR $F218
0448   A9 12                LDA #$12		;Load 0x12 into A
044A   A0 00                LDY #$00		;Load 0x00 into Y
044C   A2 00                LDX #$00		;Load 0x00 into X
044E   20 18 F2             JSR $F218
0451   A9 13                LDA #$13		;Load 0x13 into A
0453   A0 00                LDY #$00		;Load 0x00 into Y
0455   A2 00                LDX #$00		;Load 0x00 into X
0457   20 18 F2             JSR $F218
045A   A9 15                LDA #$15
045C   A0 00                LDY #$00
045E   A2 00                LDX #$00
0460   20 18 F2             JSR $F218
0463   A9 17                LDA #$17
0465   A0 00                LDY #$00
0467   A2 00                LDX #$00
0469   20 18 F2             JSR $F218
046C   A9 19                LDA #$19
046E   A0 00                LDY #$00
0470   A2 00                LDX #$00
0472   20 18 F2             JSR $F218
0475   A9 1D                LDA #$1D
0477   A0 00                LDY #$00
0479   A2 00                LDX #$00
047B   20 18 F2             JSR $F218
047E   A9 13                LDA #$13
0480   A0 01                LDY #$01
0482   A2 00                LDX #$00
0484   20 18 F2             JSR $F218
0487   A9 15                LDA #$15
0489   A0 01                LDY #$01
048B   A2 00                LDX #$00
048D   20 18 F2             JSR $F218
0490   A9 17                LDA #$17
0492   A0 01                LDY #$01
0494   A2 00                LDX #$00
0496   20 18 F2             JSR $F218
0499   A9 1A                LDA #$1A
049B   A0 01                LDY #$01
049D   A2 00                LDX #$00
049F   20 18 F2             JSR $F218
04A2   A9 1C                LDA #$1C
04A4   A0 01                LDY #$01
04A6   A2 00                LDX #$00
04A8   20 18 F2             JSR $F218
04AB   A9 12                LDA #$12
04AD   A0 02                LDY #$02
04AF   A2 00                LDX #$00
04B1   20 18 F2             JSR $F218
04B4   A9 15                LDA #$15
04B6   A0 02                LDY #$02
04B8   A2 00                LDX #$00
04BA   20 18 F2             JSR $F218
04BD   A9 17                LDA #$17
04BF   A0 02                LDY #$02
04C1   A2 00                LDX #$00
04C3   20 18 F2             JSR $F218
04C6   A9 1B                LDA #$1B
04C8   A0 02                LDY #$02
04CA   A2 00                LDX #$00
04CC   20 18 F2             JSR $F218
04CF   A9 11                LDA #$11
04D1   A0 03                LDY #$03
04D3   A2 00                LDX #$00
04D5   20 18 F2             JSR $F218
04D8   A9 16                LDA #$16
04DA   A0 03                LDY #$03
04DC   A2 00                LDX #$00
04DE   20 18 F2             JSR $F218
04E1   A9 1B                LDA #$1B
04E3   A0 03                LDY #$03
04E5   A2 00                LDX #$00
04E7   20 18 F2             JSR $F218
04EA   A9 10                LDA #$10
04EC   A0 04                LDY #$04
04EE   A2 00                LDX #$00
04F0   20 18 F2             JSR $F218
04F3   A9 16                LDA #$16
04F5   A0 04                LDY #$04
04F7   A2 00                LDX #$00
04F9   20 18 F2             JSR $F218
04FC   A9 1A                LDA #$1A
04FE   A0 04                LDY #$04
0500   A2 00                LDX #$00
0502   20 18 F2             JSR $F218
0505   A9 1C                LDA #$1C
0507   A0 04                LDY #$04
0509   A2 00                LDX #$00
050B   20 18 F2             JSR $F218
050E   A9 10                LDA #$10
0510   A0 05                LDY #$05
0512   A2 00                LDX #$00
0514   20 18 F2             JSR $F218
0517   A9 11                LDA #$11
0519   A0 05                LDY #$05
051B   A2 00                LDX #$00
051D   20 18 F2             JSR $F218
0520   A9 12                LDA #$12
0522   A0 05                LDY #$05
0524   A2 00                LDX #$00
0526   20 18 F2             JSR $F218
0529   A9 13                LDA #$13
052B   A0 05                LDY #$05
052D   A2 00                LDX #$00
052F   20 18 F2             JSR $F218
0532   A9 16                LDA #$16
0534   A0 05                LDY #$05
0536   A2 00                LDX #$00
0538   20 18 F2             JSR $F218
053B   A9 19                LDA #$19
053D   A0 05                LDY #$05
053F   A2 00                LDX #$00
0541   20 18 F2             JSR $F218
0544   A9 1D                LDA #$1D
0546   A0 05                LDY #$05
0548   A2 00                LDX #$00
054A   20 18 F2             JSR $F218
/*
	After the initial RAM setup the following addresses are set with the following content:
	0x00A6 = 0xF5
	0x00A7 = 0x22
	0x00AA = 0x15
	0x00AB = 0x14
	0x00AE = 0x25
	0x00AF = 0x08
	0x00B2 = 0x42
	0x00B3 = 0x08
	0x00B6 = 0x82
	0x00B7 = 0x14
	0x00BA = 0xF2
	0x00BB = 0x22
*/
054D   20 AF F3             JSR $F3AF
0550   A5 E0                LDA $E0
0552   C9 00                CMP #$00
0554   D0 03                BNE L0559
0556   4C 64 F5             JMP $F564
0559   20 65 F3   L0559     JSR $F365
055C   D0 03                BNE L0561
055E   4C 6E F6             JMP $F66E
0561   4C 1F F4   L0561     JMP $F41F
0564   A9 0A                LDA #$0A
0566   85 E0                STA $E0
0568   A5 E0                LDA $E0
056A   38                   SEC
056B   E9 01                SBC #$01
056D   85 E0                STA $E0
056F   A9 B4                LDA #$B4
0571   85 08                STA $08
0573   20 AF F3             JSR $F3AF
0576   A5 E0                LDA $E0
0578   C9 00                CMP #$00
057A   D0 03                BNE L057F
057C   4C 8A F5             JMP $F58A
057F   20 65 F3   L057F     JSR $F365
0582   D0 03                BNE L0587
0584   4C 6E F6             JMP $F66E
0587   4C 8E F5   L0587     JMP $F58E
058A   A9 0A                LDA #$0A
058C   85 E0                STA $E0
058E   A5 E0                LDA $E0
0590   38                   SEC
0591   E9 01                SBC #$01
0593   85 E0                STA $E0
0595   A9 D4                LDA #$D4
0597   85 08                STA $08
0599   20 AF F3             JSR $F3AF
059C   A5 E0                LDA $E0
059E   C9 00                CMP #$00
05A0   D0 03                BNE L05A5
05A2   4C B0 F5             JMP $F5B0
05A5   20 65 F3   L05A5     JSR $F365
05A8   D0 03                BNE L05AD
05AA   4C 6E F6             JMP $F66E
05AD   4C 8E F5   L05AD     JMP $F58E
05B0   A9 0A                LDA #$0A
05B2   85 E0                STA $E0
05B4   A5 E0                LDA $E0
05B6   38                   SEC
05B7   E9 01                SBC #$01
05B9   85 E0                STA $E0
05BB   A9 F6                LDA #$F6
05BD   85 08                STA $08
05BF   20 AF F3             JSR $F3AF
05C2   A5 E0                LDA $E0
05C4   C9 00                CMP #$00
05C6   D0 03                BNE L05CB
05C8   4C D6 F5             JMP $F5D6
05CB   20 65 F3   L05CB     JSR $F365
05CE   D0 03                BNE L05D3
05D0   4C 6E F6             JMP $F66E
05D3   4C B4 F5   L05D3     JMP $F5B4
05D6   A9 0A                LDA #$0A
05D8   85 E0                STA $E0
05DA   A5 E0                LDA $E0
05DC   38                   SEC
05DD   E9 01                SBC #$01
05DF   85 E0                STA $E0
05E1   A9 1E                LDA #$1E
05E3   85 08                STA $08
05E5   20 AF F3             JSR $F3AF
05E8   A5 E0                LDA $E0
05EA   C9 00                CMP #$00
05EC   D0 03                BNE L05F1
05EE   4C FC F5             JMP $F5FC
05F1   20 65 F3   L05F1     JSR $F365
05F4   D0 03                BNE L05F9
05F6   4C 6E F6             JMP $F66E
05F9   4C DA F5   L05F9     JMP $F5DA
05FC   A9 0A                LDA #$0A
05FE   85 E0                STA $E0
0600   A5 E0                LDA $E0
0602   38                   SEC
0603   E9 01                SBC #$01
0605   85 E0                STA $E0
0607   A9 36                LDA #$36
0609   85 08                STA $08
060B   20 AF F3             JSR $F3AF
060E   A5 E0                LDA $E0
0610   C9 00                CMP #$00
0612   D0 03                BNE L0617
0614   4C 22 F6             JMP $F622
0617   20 65 F3   L0617     JSR $F365
061A   D0 03                BNE L061F
061C   4C 6E F6             JMP $F66E
061F   4C 00 F6   L061F     JMP $F600
0622   A9 0A                LDA #$0A
0624   85 E0                STA $E0
0626   A5 E0                LDA $E0
0628   38                   SEC
0629   E9 01                SBC #$01
062B   85 E0                STA $E0
062D   A9 46                LDA #$46
062F   85 08                STA $08
0631   20 AF F3             JSR $F3AF
0634   A5 E0                LDA $E0
0636   C9 00                CMP #$00
0638   D0 03                BNE L063D
063A   4C 48 F6             JMP $F648
063D   20 65 F3   L063D     JSR $F365
0640   D0 03                BNE L0645
0642   4C 6E F6             JMP $F66E
0645   4C 26 F6   L0645     JMP $F626
0648   A9 0A                LDA #$0A
064A   85 E0                STA $E0
064C   A5 E0                LDA $E0
064E   38                   SEC
064F   E9 01                SBC #$01
0651   85 E0                STA $E0
0653   A9 68                LDA #$68
0655   85 08                STA $08
0657   20 AF F3             JSR $F3AF
065A   A5 E0                LDA $E0
065C   C9 00                CMP #$00
065E   D0 03                BNE L0663
0660   4C 1B F4             JMP $F41B
0663   20 65 F3   L0663     JSR $F365
0666   D0 03                BNE L066B
0668   4C 6E F6             JMP $F66E
066B   4C 4C F6   L066B     JMP $F64C

066E   A9 00                LDA #$00
0670   85 19                STA AUDV0
0672   A9 28                LDA #$28
0674   85 F3                STA $F3
0676   A9 46                LDA #$46
0678   85 F4                STA $F4
067A   A9 28                LDA #$28
067C   85 DC                STA $DC
067E   A9 02                LDA #$02
0680   85 DD                STA $DD
0682   A9 00                LDA #$00
0684   85 E1                STA LOGO_POS
0686   A9 44                LDA #$44
0688   85 A3                STA $A3
068A   A9 01                LDA #$01
068C   85 95                STA $95
068E   A9 00                LDA #$00
0690   85 94                STA $94
0692   A9 00                LDA #$00
0694   85 93                STA $93

0696   A9 42                LDA #$42
0698   85 06                STA COLUP0
069A   A9 1E                LDA #$1E
069C   85 07                STA COLUP1
069E   A5 E1                LDA LOGO_POS
06A0   18                   CLC
06A1   69 01                ADC #$01            ;Increment LOGO_POS by 1
06A3   85 E1                STA LOGO_POS
06A5   4C BD F6             JMP $F6BD

06A8   F8                   SED
06A9   18                   CLC
06AA   A5 95                LDA $95
06AC   69 01                ADC #$01
06AE   85 95                STA $95
06B0   A5 94                LDA $94
06B2   69 00                ADC #$00
06B4   85 94                STA $94
06B6   A5 93                LDA $93
06B8   69 00                ADC #$00
06BA   85 93                STA $93
06BC   D8                   CLD

06BD   A9 10                LDA #$10
06BF   85 09                STA COLUBK
06C1   A9 CC                LDA #$CC
06C3   85 8A                STA $8A
06C5   A9 F6                LDA #$F6
06C7   85 8B                STA $8B
06C9   4C D5 F6             JMP $F6D5

06CC   00                   BRK
06CD   00                   BRK
06CE   70 50                BVS L0720
06D0   00                   BRK
06D1   20 00 50             JSR $5000
06D4   00                   BRK

06D5   A9 08                LDA #$08
06D7   85 8E                STA $8E
06D9   A9 E4                LDA #$E4
06DB   85 8C                STA $8C
06DD   A9 F6                LDA #$F6
06DF   85 8D                STA $8D
06E1   4C EB F6             JMP $F6EB

06E4   00                   BRK
06E5   60                   RTS
06E6   80                   ??
06E7   80                   ??
06E8   80                   ??
06E9   80                   ??
06EA   70                   ??

06EB   A9 06                LDA #$06
06ED   85 8F                STA $8F
06EF   A5 DD                LDA $DD
06F1   C9 4D                CMP #$4D
06F3   90 41                BCC L0736
06F5   20 94 F3             JSR $F394
06F8   85 DC                STA $DC
06FA   A5 DC                LDA $DC
06FC   C9 55                CMP #$55
06FE   90 0E                BCC L070E
0700   A9 4D                LDA #$4D
0702   C5 DD                CMP $DD
0704   B0 08                BCS L070E
0706   A9 35                LDA #$35
0708   85 DC                STA $DC
070A   A9 00                LDA #$00
070C   85 EE                STA $EE
070E   A5 DC      L070E     LDA $DC
0710   C9 AA                CMP #$AA
0712   90 0E                BCC L0722
0714   A9 4D                LDA #$4D
0716   C5 DD                CMP $DD
0718   B0 08                BCS L0722
071A   A9 3B                LDA #$3B
071C   85 DC                STA $DC
071E   A9 00                LDA #$00
0720   85 EE      L0720     STA $EE
0722   A5 DC      L0722     LDA $DC
0724   C9 FF                CMP #$FF
0726   90 0E                BCC L0736
0728   A9 4D                LDA #$4D
072A   C5 DD                CMP $DD
072C   B0 08                BCS L0736
072E   A9 3F                LDA #$3F
0730   85 DC                STA $DC
0732   A9 00                LDA #$00
0734   85 EE                STA $EE
0736   A5 F3      L0736     LDA $F3
0738   85 80                STA $80
073A   A5 F4                LDA $F4
073C   85 85                STA $85
073E   A5 DC                LDA $DC
0740   85 81                STA $81
0742   A5 DD                LDA $DD
0744   85 86                STA $86
0746   A5 DD                LDA $DD
0748   18                   CLC
0749   69 01                ADC #$01
074B   85 DD                STA $DD
074D   20 AF F3             JSR $F3AF
0750   20 59 F3             JSR $F359
0753   D0 07                BNE L075C
0755   A5 F3                LDA $F3
0757   38                   SEC
0758   E9 01                SBC #$01
075A   85 F3                STA $F3
075C   A5 F3      L075C     LDA $F3
075E   C9 05                CMP #$05
0760   B0 04                BCS L0766
0762   A9 05                LDA #$05
0764   85 F3                STA $F3
0766   A9 50      L0766     LDA #$50
0768   C5 F3                CMP $F3
076A   B0 04                BCS L0770
076C   A9 50                LDA #$50
076E   85 F3                STA $F3
0770   20 5F F3   L0770     JSR $F35F
0773   D0 07                BNE L077C
0775   A5 F3                LDA $F3
0777   18                   CLC
0778   69 01                ADC #$01
077A   85 F3                STA $F3
077C   A5 F3      L077C     LDA $F3
077E   C9 05                CMP #$05
0780   B0 04                BCS L0786
0782   A9 05                LDA #$05
0784   85 F3                STA $F3
0786   A9 50      L0786     LDA #$50
0788   C5 F3                CMP $F3
078A   B0 04                BCS L0790
078C   A9 50                LDA #$50
078E   85 F3                STA $F3
0790   A9 04      L0790     LDA #$04
0792   20 96 F2             JSR $F296
0795   A5 DD                LDA $DD
0797   C9 46                CMP #$46
0799   D0 07                BNE CHECK_LOGO_POS
079B   24 07                BIT $07
079D   10 03                BPL CHECK_LOGO_POS
079F   4C A8 F6             JMP $F6A8

/* ZYX LOGO POSITION CHECK */
07A2   A5 E1 CHECK_LOGO_POS LDA LOGO_POS
07A4   C9 04                CMP #$04
07A6   D0 03                BNE L07AB       ;If logo is NOT in final position
07A8   4C AE F7             JMP $F7AE       ;If logo IS in final position

07AB   4C 96 F6   L07AB     JMP $F696

07AE   20 AF F3             JSR $F3AF
07B1   A9 D4                LDA #$D4
07B3   85 09                STA COLUBK
07B5   20 2F F3             JSR $F32F
07B8   F0 03                BEQ L07BD
07BA   4C AB F7             JMP $F7AB
07BD   FF         L07BD     ???                ;%11111111
07BE   FF                   ???                ;%11111111
07BF   FF                   ???                ;%11111111
07C0   FF                   ???                ;%11111111
07C1   FF                   ???                ;%11111111
07C2   FF                   ???                ;%11111111
07C3   FF                   ???                ;%11111111
07C4   FF                   ???                ;%11111111
07C5   FF                   ???                ;%11111111
07C6   FF                   ???                ;%11111111
07C7   FF                   ???                ;%11111111
07C8   FF                   ???                ;%11111111
07C9   FF                   ???                ;%11111111
07CA   FF                   ???                ;%11111111
07CB   FF                   ???                ;%11111111
07CC   FF                   ???                ;%11111111
07CD   FF                   ???                ;%11111111
07CE   FF                   ???                ;%11111111
07CF   FF                   ???                ;%11111111
07D0   FF                   ???                ;%11111111
07D1   FF                   ???                ;%11111111
07D2   FF                   ???                ;%11111111
07D3   FF                   ???                ;%11111111
07D4   FF                   ???                ;%11111111
07D5   FF                   ???                ;%11111111
07D6   FF                   ???                ;%11111111
07D7   FF                   ???                ;%11111111
07D8   FF                   ???                ;%11111111
07D9   FF                   ???                ;%11111111
07DA   FF                   ???                ;%11111111
07DB   FF                   ???                ;%11111111
07DC   FF                   ???                ;%11111111
07DD   FF                   ???                ;%11111111
07DE   FF                   ???                ;%11111111
07DF   FF                   ???                ;%11111111
07E0   FF                   ???                ;%11111111
07E1   FF                   ???                ;%11111111
07E2   FF                   ???                ;%11111111
07E3   FF                   ???                ;%11111111
07E4   FF                   ???                ;%11111111
07E5   FF                   ???                ;%11111111
07E6   FF                   ???                ;%11111111
07E7   FF                   ???                ;%11111111
07E8   FF                   ???                ;%11111111
07E9   FF                   ???                ;%11111111
07EA   FF                   ???                ;%11111111
07EB   FF                   ???                ;%11111111
07EC   FF                   ???                ;%11111111
07ED   FF                   ???                ;%11111111
07EE   FF                   ???                ;%11111111
07EF   FF                   ???                ;%11111111
07F0   FF                   ???                ;%11111111
07F1   FF                   ???                ;%11111111
07F2   FF                   ???                ;%11111111
07F3   FF                   ???                ;%11111111
07F4   FF                   ???                ;%11111111
07F5   FF                   ???                ;%11111111
07F6   FF                   ???                ;%11111111
07F7   FF                   ???                ;%11111111
07F8   FF                   ???                ;%11111111
07F9   FF                   ???                ;%11111111
07FA   FF                   ???                ;%11111111
07FB   FF                   ???                ;%11111111
07FC   FF                   ???                ;%11111111
07FD   FF                   ???                ;%11111111
07FE   FF                   ???                ;%11111111
07FF   FF                   ???                ;%11111111
                            .END

;auto-generated symbols and labels
 L0039        $39
 L003F        $3F
 L009C        $9C
 L00CA        $CA
 L00DB        $DB
 LOGO_POS     $E1
 L013E      $013E
 L017C      $017C
 L01D9      $01D9
 L01E4      $01E4
 L022B      $022B
 L0224      $0224
 L0243      $0243
 L0260      $0260
 L026A      $026A
 L02A4      $02A4
 L02B4      $02B4
 L02AC      $02AC
 L029A      $029A
 L02C2      $02C2
 L02F8      $02F8
 L032E      $032E
 L02CF      $02CF
 L02D6      $02D6
 L02F0      $02F0
 L0306      $0306
 L030F      $030F
 L0327      $0327
 L036D      $036D
 L039B      $039B
 L03AF      $03AF
 L0413      $0413
 L0559      $0559
 L0561      $0561
 L057F      $057F
 L0587      $0587
 L05A5      $05A5
 L05AD      $05AD
 L05CB      $05CB
 L05D3      $05D3
 L05F1      $05F1
 L05F9      $05F9
 L0617      $0617
 L061F      $061F
 L063D      $063D
 L0645      $0645
 L0663      $0663
 L066B      $066B
 L0720      $0720
 L0695      $0695
 L0736      $0736
 L070E      $070E
 L0722      $0722
 L075C      $075C
 L0766      $0766
 L0770      $0770
 L077C      $077C
 L0786      $0786
 L0790      $0790
 L07A2      $07A2
 L07AB      $07AB
 L07BD      $07BD
 

