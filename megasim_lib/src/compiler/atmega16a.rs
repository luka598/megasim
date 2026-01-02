use std::collections::HashMap;

pub fn gen_symbols() -> HashMap<String, i64> {
    let mut symbols: HashMap<String, i64> = HashMap::new();

    // 1. Working Registers (Page 11)
    for i in 0..32 {
        symbols.insert(format!("r{}", i), i);
    }

    // 2. Memory Constants
    symbols.insert("ramend".into(), 0x045F); // Page 17
    symbols.insert("flashend".into(), 0x1FFF); // Page 16 (8K words)
    symbols.insert("eend".into(), 0x01FF); // Page 18 (512 bytes)
    symbols.insert("pagesize".into(), 64); // Page 254 (words)

    // 3. I/O Registers (Mapping: I/O + 0x20)
    // Data from Register Summary (Page 319)
    let io = [
        ("twbr", 0x00),
        ("twsr", 0x01),
        ("twar", 0x02),
        ("twdr", 0x03),
        ("adcl", 0x04),
        ("adch", 0x05),
        ("adcsra", 0x06),
        ("admux", 0x07),
        ("acsr", 0x08),
        ("ubrrl", 0x09),
        ("ucsrb", 0x0A),
        ("ucsra", 0x0B),
        ("udr", 0x0C),
        ("spcr", 0x0D),
        ("spsr", 0x0E),
        ("spdr", 0x0F),
        ("pind", 0x10),
        ("ddrd", 0x11),
        ("portd", 0x12),
        ("pinc", 0x13),
        ("ddrc", 0x14),
        ("portc", 0x15),
        ("pinb", 0x16),
        ("ddrb", 0x17),
        ("portb", 0x18),
        ("pina", 0x19),
        ("ddra", 0x1A),
        ("porta", 0x1B),
        ("eecr", 0x1C),
        ("eedr", 0x1D),
        ("eearl", 0x1E),
        ("eearh", 0x1F),
        ("ucsrc", 0x20),
        ("ubrrh", 0x20),
        ("wdtcr", 0x21),
        ("assr", 0x22),
        ("ocr2", 0x23),
        ("tcnt2", 0x24),
        ("tccr2", 0x25),
        ("icr1l", 0x26),
        ("icr1h", 0x27),
        ("ocr1bl", 0x28),
        ("ocr1bh", 0x29),
        ("ocr1al", 0x2A),
        ("ocr1ah", 0x2B),
        ("tcnt1l", 0x2C),
        ("tcnt1h", 0x2D),
        ("tccr1b", 0x2E),
        ("tccr1a", 0x2F),
        ("sfior", 0x30),
        ("osccal", 0x31),
        ("ocdr", 0x31),
        ("tcnt0", 0x32),
        ("tccr0", 0x33),
        ("mcucsr", 0x34),
        ("mcucr", 0x35),
        ("twcr", 0x36),
        ("spmcr", 0x37),
        ("tifr", 0x38),
        ("timsk", 0x39),
        ("gifr", 0x3A),
        ("gicr", 0x3B),
        ("ocr0", 0x3C),
        ("spl", 0x3D),
        ("sph", 0x3E),
        ("sreg", 0x3F),
    ];
    for (name, addr) in io {
        symbols.insert(name.into(), addr + 0x20);
    }

    // 4. Bit Symbols (Exhaustive from Pages 9-212)

    // SREG (Page 9)
    let sreg_bits = [
        ("c", 0),
        ("z", 1),
        ("n", 2),
        ("v", 3),
        ("s", 4),
        ("h", 5),
        ("t", 6),
        ("i", 7),
    ];
    for (n, v) in sreg_bits {
        symbols.insert(n.into(), v);
    }

    // MCUCR (Page 35, 66)
    symbols.insert("isc00".into(), 0);
    symbols.insert("isc01".into(), 1);
    symbols.insert("isc10".into(), 2);
    symbols.insert("isc11".into(), 3);
    symbols.insert("sm0".into(), 4);
    symbols.insert("sm1".into(), 5);
    symbols.insert("se".into(), 6);
    symbols.insert("sm2".into(), 7);

    // MCUCSR (Page 41, 67)
    symbols.insert("porf".into(), 0);
    symbols.insert("extrf".into(), 1);
    symbols.insert("borf".into(), 2);
    symbols.insert("wdrf".into(), 3);
    symbols.insert("jtrf".into(), 4);
    symbols.insert("isc2".into(), 6);
    symbols.insert("jtd".into(), 7);

    // GICR (Page 46, 67)
    symbols.insert("ivce".into(), 0);
    symbols.insert("ivsel".into(), 1);
    symbols.insert("int2".into(), 5);
    symbols.insert("int0".into(), 6);
    symbols.insert("int1".into(), 7);

    // GIFR (Page 68)
    symbols.insert("intf2".into(), 5);
    symbols.insert("intf0".into(), 6);
    symbols.insert("intf1".into(), 7);

    // TIMSK (Page 82, 109, 128)
    symbols.insert("toie0".into(), 0);
    symbols.insert("ocie0".into(), 1);
    symbols.insert("toie1".into(), 2);
    symbols.insert("ocie1b".into(), 3);
    symbols.insert("ocie1a".into(), 4);
    symbols.insert("ticie1".into(), 5);
    symbols.insert("toie2".into(), 6);
    symbols.insert("ocie2".into(), 7);

    // TIFR (Page 82, 110, 128)
    symbols.insert("tov0".into(), 0);
    symbols.insert("ocf0".into(), 1);
    symbols.insert("tov1".into(), 2);
    symbols.insert("ocf1b".into(), 3);
    symbols.insert("ocf1a".into(), 4);
    symbols.insert("icf1".into(), 5);
    symbols.insert("tov2".into(), 6);
    symbols.insert("ocf2".into(), 7);

    // TCCR0 (Page 79-81)
    symbols.insert("cs00".into(), 0);
    symbols.insert("cs01".into(), 1);
    symbols.insert("cs02".into(), 2);
    symbols.insert("wgm01".into(), 3);
    symbols.insert("com00".into(), 4);
    symbols.insert("com01".into(), 5);
    symbols.insert("wgm00".into(), 6);
    symbols.insert("foc0".into(), 7);

    // TCCR1A (Page 105)
    symbols.insert("wgm10".into(), 0);
    symbols.insert("wgm11".into(), 1);
    symbols.insert("foc1b".into(), 2);
    symbols.insert("foc1a".into(), 3);
    symbols.insert("com1b0".into(), 4);
    symbols.insert("com1b1".into(), 5);
    symbols.insert("com1a0".into(), 6);
    symbols.insert("com1a1".into(), 7);

    // TCCR1B (Page 107)
    symbols.insert("cs10".into(), 0);
    symbols.insert("cs11".into(), 1);
    symbols.insert("cs12".into(), 2);
    symbols.insert("wgm12".into(), 3);
    symbols.insert("wgm13".into(), 4);
    symbols.insert("ices1".into(), 6);
    symbols.insert("icnc1".into(), 7);

    // TCCR2 (Page 125)
    symbols.insert("cs20".into(), 0);
    symbols.insert("cs21".into(), 1);
    symbols.insert("cs22".into(), 2);
    symbols.insert("wgm21".into(), 3);
    symbols.insert("com20".into(), 4);
    symbols.insert("com21".into(), 5);
    symbols.insert("wgm20".into(), 6);
    symbols.insert("foc2".into(), 7);

    // ASSR (Page 127)
    symbols.insert("tcr2ub".into(), 0);
    symbols.insert("ocr2ub".into(), 1);
    symbols.insert("tcn2ub".into(), 2);
    symbols.insert("as2".into(), 3);

    // ADCSRA (Page 210)
    symbols.insert("adps0".into(), 0);
    symbols.insert("adps1".into(), 1);
    symbols.insert("adps2".into(), 2);
    symbols.insert("adie".into(), 3);
    symbols.insert("adif".into(), 4);
    symbols.insert("adate".into(), 5);
    symbols.insert("adsc".into(), 6);
    symbols.insert("aden".into(), 7);

    // ADMUX (Page 208-209)
    symbols.insert("mux0".into(), 0);
    symbols.insert("mux1".into(), 1);
    symbols.insert("mux2".into(), 2);
    symbols.insert("mux3".into(), 3);
    symbols.insert("mux4".into(), 4);
    symbols.insert("adlar".into(), 5);
    symbols.insert("refs0".into(), 6);
    symbols.insert("refs1".into(), 7);

    // ACSR (Page 194)
    symbols.insert("acis0".into(), 0);
    symbols.insert("acis1".into(), 1);
    symbols.insert("acic".into(), 2);
    symbols.insert("acie".into(), 3);
    symbols.insert("aci".into(), 4);
    symbols.insert("aco".into(), 5);
    symbols.insert("acbg".into(), 6);
    symbols.insert("acd".into(), 7);

    // Port/DDR bits (0-7)
    for i in 0..8 {
        symbols.insert(format!("pa{}", i), i);
        symbols.insert(format!("dda{}", i), i);
        symbols.insert(format!("pb{}", i), i);
        symbols.insert(format!("ddb{}", i), i);
        symbols.insert(format!("pc{}", i), i);
        symbols.insert(format!("ddc{}", i), i);
        symbols.insert(format!("pd{}", i), i);
        symbols.insert(format!("ddd{}", i), i);
    }

    symbols
}