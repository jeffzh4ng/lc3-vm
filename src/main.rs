fn main() {
    // The LC-3 has 65,536 (2^16) each of which stores a 16-bit value.
    let memory: [u16; 65536];

    // The LC-3 has 10 register in total, each of which is 16 bits.
    // - 8 general purpose registers (R0-R7)
    // - 1 program counter (PC) register
    // - 1 condition flag (COND) register

    enum Register {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        RPc,
        RCong,
        RCount,
    }

    let register: [u16; 10];

    enum Op {
        BR,    /* branch */
        ADD,   /* add  */
        LD,    /* load */
        ST,    /* store */
        JSR,   /* jump register */
        AND,   /* bitwise and */
        LDR,   /* load register */
        STR,   /* store register */
        RTI,   /* unused */
        NOT,   /* bitwise not */
        LDI,   /* load indirect */
        STI,   /* store indirect */
        JMP,   /* jump */
        RES,   /* reserved (unused) */
        LEA,   /* load effective address */
        TRAP    /* execute trap */
    }

    enum ConditionFlag {
        POS,
        ZRO,
        NEG,
    }
    println!("Hello, world!");
}
