use serde_x12::to_string;
use validator::Validate;
use crate::v004010::*;

#[test]
fn parse_315() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: "P".to_string(),
            _16: ">".to_string(),
            ..Default::default()
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                    ..Default::default()
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        ..Default::default()
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _09: Some("202N".to_string()),
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    ..Default::default()
                }),
                sg: vec![],
                loop_r4: vec![_315LoopR4{
                    r4: R4 {
                        _01: "L".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USMEM".to_string()),
                        _04: Some("BNSF MEMPHIS RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                },_315LoopR4{
                    r4: R4 {
                        _01: "E".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USDAL".to_string()),
                        _04: Some("BNSF ALLIANCE RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                }],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let str = serde_x12::to_string(&x).unwrap();
    println!("{}", str);
    let new_input = str.replace("\r\n", "");
    let new_input = new_input.replace("\n", "");
    let obj: Transmission<_315> = serde_x12::from_str(&new_input).unwrap();
    println!("{:?}", obj);
    assert_eq!(x, obj);
}

#[test]
fn test_315() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: "P".to_string(),
            _16: ">".to_string(),
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _01: None,
                    _02: None,
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _06: Some("".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _10: Some("".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        _03: None,
                        _04: None,
                        _05: None,
                        _06: None,
                        _07: None,
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _02: None,
                    _03: None,
                    _04: None,
                    _05: None,
                    _06: None,
                    _07: None,
                    _08: None,
                    _09: Some("202N".to_string()),
                    _10: None,
                    _11: None,
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    _14: None,
                    _15: None,
                    _16: None,
                }),
                sg: vec![],
                loop_r4: vec![_315LoopR4{
                    r4: R4 {
                        _01: "L".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USMEM".to_string()),
                        _04: Some("BNSF MEMPHIS RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _06: Some("".to_string()),
                        _07: Some("".to_string()),
                        _08: Some("US".to_string()),
                    },
                    dtm: None,
                },_315LoopR4{
                    r4: R4 {
                        _01: "E".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USDAL".to_string()),
                        _04: Some("BNSF ALLIANCE RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _06: Some("".to_string()),
                        _07: Some("".to_string()),
                        _08: Some("US".to_string()),
                    },
                    dtm: None,
                }],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let serialized = serde_x12::to_string(&x).unwrap();
    let original = r#"ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
ST*315*00001~
B4***VA*20220901*0807**GMCU*609413*E**LOCKBOURNE*CI*7~
N9*BM*21001ASK5V9U~
N9*BN*1NAN910141~
N9*EQ*GMCU6094137~
Q2*9330141********202N***L*MARIM~
R4*L*UN*USMEM*BNSF MEMPHIS RAMP*US***US~
R4*E*UN*USDAL*BNSF ALLIANCE RAMP*US***US~
SE*9*00001~
GE*1*1~
IEA*1*000000001~
"#;
    assert_eq!(serialized, original);
}

#[test]
fn test_315_defaults() {
    let x = Transmission {
        isa: ISA {
            _01: "00".to_string(),
            _02: "          ".to_string(),
            _03: "00".to_string(),
            _04: "          ".to_string(),
            _05: "ZZ".to_string(),
            _06: "SOURCE         ".to_string(),
            _07: "ZZ".to_string(),
            _08: "TARGET         ".to_string(),
            _09: "220524".to_string(),
            _10: "1120".to_string(),
            _11: "U".to_string(),
            _12: "00401".to_string(),
            _13: "000000001".to_string(),
            _14: "0".to_string(),
            _15: "P".to_string(),
            _16: ">".to_string(),
            ..Default::default()
        },
        functional_group: vec![FunctionalGroup {
            gs: GS {
                _01: "QO".to_string(),
                _02: "SOURCE".to_string(),
                _03: "TARGET".to_string(),
                _04: "20220524".to_string(),
                _05: "1600".to_string(),
                _06: "1".to_string(),
                _07: "X".to_string(),
                _08: "004010".to_string(),
            },
            segments: vec![_315 {
                st: ST {
                    _01: "315".to_string(),
                    _02: "00001".to_string(),
                },
                b4: B4 {
                    _03: Some("VA".to_string()),
                    _04: Some("20220901".to_string()),
                    _05: Some("0807".to_string()),
                    _06: Some("".to_string()),
                    _07: Some("GMCU".to_string()),
                    _08: Some("609413".to_string()),
                    _09: Some("E".to_string()),
                    _11: Some("LOCKBOURNE".to_string()),
                    _12: Some("CI".to_string()),
                    _13: Some("7".to_string()),
                    ..Default::default()
                },
                n9: vec![
                    N9 {
                        _01: "BM".to_string(),
                        _02: "21001ASK5V9U".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "BN".to_string(),
                        _02: "1NAN910141".to_string(),
                        ..Default::default()
                    },
                    N9 {
                        _01: "EQ".to_string(),
                        _02: "GMCU6094137".to_string(),
                        ..Default::default()
                    },
                ],
                q2: Some(Q2 {
                    _01: "9330141".to_string(),
                    _09: Some("202N".to_string()),
                    _12: Some("L".to_string()),
                    _13: Some("MARIM".to_string()),
                    ..Default::default()
                }),
                sg: vec![],
                loop_r4: vec![_315LoopR4{
                    r4: R4 {
                        _01: "L".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USMEM".to_string()),
                        _04: Some("BNSF MEMPHIS RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                },_315LoopR4{
                    r4: R4 {
                        _01: "E".to_string(),
                        _02: Some("UN".to_string()),
                        _03: Some("USDAL".to_string()),
                        _04: Some("BNSF ALLIANCE RAMP".to_string()),
                        _05: Some("US".to_string()),
                        _08: Some("US".to_string()),
                        ..Default::default()
                    },
                    dtm: None,
                }],
                v9: None,
                se: SE {
                    _01: "9".to_string(),
                    _02: "00001".to_string(),
                },
            }],
            ge: GE {
                _01: "1".to_string(),
                _02: "1".to_string(),
            },
        }],
        iea: IEA {
            _01: "1".to_string(),
            _02: "000000001".to_string(),
        },
    };
    let serialized = serde_x12::to_string(&x).unwrap();
    let original = r#"ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
ST*315*00001~
B4***VA*20220901*0807**GMCU*609413*E**LOCKBOURNE*CI*7~
N9*BM*21001ASK5V9U~
N9*BN*1NAN910141~
N9*EQ*GMCU6094137~
Q2*9330141********202N***L*MARIM~
R4*L*UN*USMEM*BNSF MEMPHIS RAMP*US***US~
R4*E*UN*USDAL*BNSF ALLIANCE RAMP*US***US~
SE*9*00001~
GE*1*1~
IEA*1*000000001~
"#;
    assert_eq!(serialized, original);
}

#[test]
fn test_310() {
    let edi = r#"ISA*00*          *00*          *ZZ*SOURCE         *02*TARGET         *220101*1449*U*00401*000011566*0*P*>~GS*IO*SOURCE*TARGET*20220101*1449*61716*X*004010~ST*310*35353~B3*3*IDENTIFIER123*IDENTIFIER123*MX**20220830*00****WHT*20220830*PP~B2A*00*BL~N9*BN*1XXX011114*BOOKING NUMBER~N9*BM*IDENTIFIER123*BILL OF LADING NUMBER~N9*R1*MSK-BL-1.0*INTERNAL GUIDELINE VERSION NUMBER~N9*VT*9V8896*VESSEL CALL-SIGN~V1*9786774*POLAR ECUADOR*SG*234N****L~Y2*1***45G1~N1*SH*MADERAS ARAUCO S.A.*25*940924821~N3*EL GOLF 150~N4*LAS CONDES**7550107*CL~N1*CN*ARAUCO NORTH AMERICA INC*25*100992847~N3*PERIMETER CENTER TER NE 400~N4*ATLANTA*GA*30346*US~N1*N1*GEODIS USA INC*25*12323242~N3*LACROSS RD 4995~N4*NORTH CHARLESTON*SC*29406*US~N1*CA*WHATEVER*2*WHT~N3*ESPLANADEN 50~N4*COPENHAGEN K**1098*DK~R4*L*UN*CLSVE*SAN VICENTE*CL***BI~DTM*140*20220830*003000*LT~R4*D*UN*USBAL*BALTIMORE*US***MD~DTM*140*20220101*120000*LT~R4*T*UN*PAMIT*MANZANILLO*PA~DTM*140*20220101*130000*LT~C8***WHT CODE: 4411.1400~C8***EMISION SWB.~C8***CHARGE_TYPE-BASIC FREIGHT; PAYER-SHIPPER; TERM-PREPAID;~LX*1~N7*TCNU*6849731*17007*G*3810*28690**31.32*X*S*CN*****M*K*1****45G1~M7*MLCL0008628~L0*1***17007*G*31.32*X*36*PKG**K~L5*1*1 X 40'HC 21 PACKAGES WITH 31.320 MCUB AND 16.837*2313432*Z~L5*1*NET WEIGHT . MOULDINGS OF MEDIUM DENSITY FIBERBO~L5*1*ARD ( MECHANICALLY WORKED (WITH SURFACED COVER ED~L5*1*) . REF.INT: 111111 . ALSO CONSIGNEE: ATTN: AW~L3*17007*G*******42.42*X*1*K~L1*1*3371*FR*316100****COF***P******1*NR~C3*USD~L1*2*875*FR*87900****BUA***P******1*NR~C3*USD~L1*3*29*FR*2800*******P******1*NR~C3*USD~SE*49*32353~GE*1*61916~IEA*1*000061216~"#;
    let obj:Transmission<_310> = serde_x12::from_str(edi).unwrap();
}