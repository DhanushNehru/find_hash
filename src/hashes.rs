//! This file is auto-generated
//! Do not edit manually.

use crate::hash_info::{HashInfo, Prototype};
use regex::Regex;
use once_cell::sync::Lazy;

pub static PROTOTYPES: Lazy<Vec<Prototype>> = Lazy::new(|| vec![
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{4}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CRC-16",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "CRC-16-CCITT",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "FCS-16",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{8}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Adler-32",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "CRC-32B",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "FCS-32",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "GHash-32-3",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "GHash-32-5",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "FNV-132",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Fletcher-32",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Joaat",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "ELF-32",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "XOR-32",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{6}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CRC-24",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$crc32\$)?([a-f0-9]{8}.)?[a-f0-9]{8}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CRC-32",
                hashcat: Some(11500),
                john: Some("crc32"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\+[a-z0-9\/.]{12}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Eggdrop IRC Bot",
                hashcat: None,
                john: Some("bfegg"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/.]{12}[.26AEIMQUYcgkosw]{1}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "DES(Unix)",
                hashcat: Some(1500),
                john: Some("descrypt"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Traditional DES",
                hashcat: Some(1500),
                john: Some("descrypt"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "DEScrypt",
                hashcat: Some(1500),
                john: Some("descrypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MySQL323",
                hashcat: Some(200),
                john: Some("mysql"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Half MD5",
                hashcat: Some(5100),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "FNV-164",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "CRC-64",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{16}:[a-f0-9]{0,30}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Oracle H: Type (Oracle 7+), DES(Oracle)",
                hashcat: Some(3100),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/.]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco-PIX(MD5)",
                hashcat: Some(2400),
                john: Some("pix-md5"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\([a-z0-9\/+]{20}\)$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Lotus Notes/Domino 6",
                hashcat: Some(8700),
                john: Some("dominosec"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^_[a-z0-9\/.]{19}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "BSDi Crypt",
                hashcat: Some(12400),
                john: Some("bsdicrypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{24}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CRC-96(ZIP)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PKZIP Master Key",
                hashcat: Some(20500),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PKZIP Master Key (6 byte optimization)",
                hashcat: Some(20510),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"^\$keepass\$\*1\*50000\*(0|1)\*([a-f0-9]{32})\*([a-f0-9]{64})\*([a-f0-9]{32})\*([a-f0-9]{64})\*1\*(192|1360)\*([a-f0-9]{384})$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keepass 1 AES / without keyfile",
                hashcat: Some(13400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"^\$keepass\$\*1\*6000\*(0|1)\*([a-f0-9]{32})\*([a-f0-9]{64})\*([a-f0-9]{32})\*([a-f0-9]{64})\*1\*(192|1360)\*([a-f0-9]{2720})\*1\*64\*([a-f0-9]{64})$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keepass 1 Twofish / with keyfile",
                hashcat: Some(13400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"^\$keepass\$\*2\*6000\*222(\*[a-f0-9]{64}){2}(\*[a-f0-9]{32}){1}(\*[a-f0-9]{64}){2}\*1\*64(\*[a-f0-9]{64}){1}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keepass 2 AES / with keyfile",
                hashcat: Some(13400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"^\$keepass\$\*2\*6000\*222\*(([a-f0-9]{32,64})(\*)?)+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keepass 2 AES / without keyfile",
                hashcat: Some(13400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/.]{24}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Crypt16",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MD5",
                hashcat: Some(0),
                john: Some("raw-md5"),
                extended: false,
                description: Some("Used for Linux Shadow files."),
            },
            HashInfo {
                name: "MD4",
                hashcat: Some(900),
                john: Some("raw-md4"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Double MD5",
                hashcat: Some(2600),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Tiger-128",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-256(128)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512(128)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Lotus Notes/Domino 5",
                hashcat: Some(8600),
                john: Some("lotus5"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "md5(md5(md5($pass)))",
                hashcat: Some(3500),
                john: None,
                extended: true,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "md5(uppercase(md5($pass)))",
                hashcat: Some(4300),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(sha1($pass))",
                hashcat: Some(4400),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(utf16($pass))",
                hashcat: None,
                john: Some("dynamic_29"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md4(utf16($pass))",
                hashcat: None,
                john: Some("dynamic_33"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(md4($pass))",
                hashcat: None,
                john: Some("dynamic_34"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)(?:\$haval\$)?[a-f0-9]{32,64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Haval-128",
                hashcat: None,
                john: Some("haval-128-4"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)(?:\$ripemd\$)?[a-f0-9]{32,40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RIPEMD-128",
                hashcat: None,
                john: Some("ripemd-128"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "LM",
                hashcat: Some(3000),
                john: Some("lm"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)(?:\$dynamic_39\$)?[a-f0-9]{32}\$[a-z0-9]{1,32}\$?[a-z0-9]{1,500}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "net-md5",
                hashcat: None,
                john: Some("dynamic_39"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[a-z0-9]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Skype",
                hashcat: Some(23),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "ZipMonster",
                hashcat: None,
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(md5(md5($pass)))",
                hashcat: Some(3500),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(uppercase(md5($pass)))",
                hashcat: Some(4300),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(sha1($pass))",
                hashcat: Some(4400),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($pass.$salt)",
                hashcat: Some(10),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($salt.$pass)",
                hashcat: Some(20),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(unicode($pass).$salt)",
                hashcat: Some(30),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($salt.unicode($pass))",
                hashcat: Some(40),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-MD5 (key = $pass)",
                hashcat: Some(50),
                john: Some("hmac-md5"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-MD5 (key = $salt)",
                hashcat: Some(60),
                john: Some("hmac-md5"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(md5($salt).$pass)",
                hashcat: Some(3610),
                john: None,
                extended: true,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "md5($salt.md5($pass))",
                hashcat: Some(3710),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($pass.md5($salt))",
                hashcat: Some(3720),
                john: None,
                extended: true,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "WebEdition CMS",
                hashcat: Some(3721),
                john: None,
                extended: false,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "md5($username.0.$pass)",
                hashcat: Some(4210),
                john: None,
                extended: true,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "md5($salt.$pass.$salt)",
                hashcat: Some(3800),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5(md5($pass).md5($salt))",
                hashcat: Some(3910),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($salt.md5($salt.$pass))",
                hashcat: Some(4010),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($salt.md5($pass.$salt))",
                hashcat: Some(4110),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md4($salt.$pass)",
                hashcat: None,
                john: Some("dynamic_31"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md4($pass.$salt)",
                hashcat: None,
                john: Some("dynamic_32"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "md5($salt.pad16($pass))",
                hashcat: None,
                john: Some("dynamic_39"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[a-z0-9]{56}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PrestaShop",
                hashcat: Some(11000),
                john: None,
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$md2\$)?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MD2",
                hashcat: None,
                john: Some("md2"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$snefru\$)?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Snefru-128",
                hashcat: None,
                john: Some("snefru-128"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$NT\$)?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "NTLM",
                hashcat: Some(1000),
                john: Some("nt"),
                extended: false,
                description: Some("Often used in Windows Active Directory."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^([^\\\/:*?"<>|]{1,20}:)?[a-f0-9]{32}(:[^\\\/:*?"<>|]{1,20})?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Domain Cached Credentials",
                hashcat: Some(1100),
                john: Some("mscash"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^([^\\\/:*?"<>|]{1,20}:)?(\$DCC2\$10240#[^\\\/:*?"<>|]{1,20}#)?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Domain Cached Credentials 2",
                hashcat: Some(2100),
                john: Some("mscash2"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{SHA\}[a-z0-9\/+]{27}=$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-1(Base64)",
                hashcat: Some(101),
                john: Some("nsldap"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Netscape LDAP SHA",
                hashcat: Some(101),
                john: Some("nsldap"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$1\$[a-z0-9\/.]{0,8}\$[a-z0-9\/.]{22}(:.*)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MD5 Crypt",
                hashcat: Some(500),
                john: Some("md5crypt"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Cisco-IOS(MD5)",
                hashcat: Some(500),
                john: Some("md5crypt"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "FreeBSD MD5",
                hashcat: Some(500),
                john: Some("md5crypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Lineage II C4",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$H\$[a-z0-9\/.]{31}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "phpBB v3.x",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Wordpress v2.6.0/2.6.1",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PHPass' Portable Hash",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$P\$[a-z0-9\/.]{31}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Wordpress ≥ v2.6.2",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Joomla ≥ v2.5.18",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PHPass' Portable Hash",
                hashcat: Some(400),
                john: Some("phpass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[a-z0-9]{2}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "osCommerce",
                hashcat: Some(21),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "xt:Commerce",
                hashcat: Some(21),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$apr1\$[a-z0-9\/.]{0,8}\$[a-z0-9\/.]{22}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MD5(APR)",
                hashcat: Some(1600),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Apache MD5",
                hashcat: Some(1600),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "md5apr1",
                hashcat: Some(1600),
                john: None,
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{smd5\}[a-z0-9$\/.]{31}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "AIX(smd5)",
                hashcat: Some(6300),
                john: Some("aix-smd5"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:.{5}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "IP.Board ≥ v2+",
                hashcat: Some(2811),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:.{8}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MyBB ≥ v1.2+",
                hashcat: Some(2811),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9]{34}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CryptoCurrency(Adress)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{40}(:.+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-1",
                hashcat: Some(100),
                john: Some("raw-sha1"),
                extended: false,
                description: Some("Used for checksums.[link=https://en.wikipedia.org/wiki/SHA-1]See more[/link]"),
            },
            HashInfo {
                name: "Double SHA-1",
                hashcat: Some(4500),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "RIPEMD-160",
                hashcat: Some(6000),
                john: Some("ripemd-160"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-160 (3 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_190"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-160 (4 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_200"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-160 (5 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_210"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-192 (3 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_220"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-192 (4 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_230"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-192 (5 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_240"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-224 (4 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_260"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-224 (5 rounds)",
                hashcat: Some(6000),
                john: Some("dynamic_270"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-160",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Tiger-160",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "HAS-160",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "LinkedIn",
                hashcat: Some(190),
                john: Some("raw-sha1-linkedin"),
                extended: false,
                description: Some("Hashcat mode is only supported in oclHashcat."),
            },
            HashInfo {
                name: "Skein-256(160)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512(160)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "MangosWeb Enhanced CMS",
                hashcat: None,
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1(sha1(sha1($pass)))",
                hashcat: Some(4600),
                john: None,
                extended: true,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
            HashInfo {
                name: "sha1(md5($pass))",
                hashcat: Some(4700),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1($pass.$salt)",
                hashcat: Some(110),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1($salt.$pass)",
                hashcat: Some(120),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1(unicode($pass).$salt)",
                hashcat: Some(130),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1($salt.unicode($pass))",
                hashcat: Some(140),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA1 (key = $pass)",
                hashcat: Some(150),
                john: Some("hmac-sha1"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA1 (key = $salt)",
                hashcat: Some(160),
                john: Some("hmac-sha1"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha1($salt.$pass.$salt)",
                hashcat: Some(4710),
                john: None,
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MySQL5.x",
                hashcat: Some(300),
                john: Some("mysql-sha1"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "MySQL4.1",
                hashcat: Some(300),
                john: Some("mysql-sha1"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco-IOS(SHA-256)",
                hashcat: Some(5700),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{SSHA\}[a-z0-9\/+]{38}==$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SSHA-1(Base64)",
                hashcat: Some(111),
                john: Some("nsldaps"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Netscape LDAP SSHA",
                hashcat: Some(111),
                john: Some("nsldaps"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "nsldaps",
                hashcat: Some(111),
                john: Some("nsldaps"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9=]{47}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Fortigate(FortiOS)",
                hashcat: Some(7000),
                john: Some("fortigate"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{48}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Haval-192",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Tiger-192",
                hashcat: None,
                john: Some("tiger"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "SHA-1(Oracle)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "OSX v10.4",
                hashcat: Some(122),
                john: Some("xsha"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "OSX v10.5",
                hashcat: Some(122),
                john: Some("xsha"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "OSX v10.6",
                hashcat: Some(122),
                john: Some("xsha"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{51}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Palshop CMS",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9]{51}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CryptoCurrency(PrivateKey)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{ssha1\}[0-9]{2}\$[a-z0-9$\/.]{44}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "AIX(ssha1)",
                hashcat: Some(6700),
                john: Some("aix-ssha1"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x0100[a-f0-9]{48}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MSSQL(2005)",
                hashcat: Some(132),
                john: Some("mssql05"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "MSSQL(2008)",
                hashcat: Some(132),
                john: Some("mssql05"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$md5,rounds=[0-9]+\$|\$md5\$rounds=[0-9]+\$|\$md5\$)[a-z0-9\/.]{0,16}(\$|\$\$)[a-z0-9\/.]{22}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Sun MD5 Crypt",
                hashcat: Some(3300),
                john: Some("sunmd5"),
                extended: false,
                description: Some("Hashcat mode is only supported in hashcat-legacy."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{56}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-224",
                hashcat: Some(1300),
                john: Some("raw-sha224"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "sha224($salt.$pass)",
                hashcat: None,
                john: Some("dynamic_51"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224($pass.$salt))",
                hashcat: None,
                john: Some("dynamic_52"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224(sha224($pass))",
                hashcat: None,
                john: Some("dynamic_53"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224(sha224_raw($pass))",
                hashcat: None,
                john: Some("dynamic_54"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224(sha224($pass).$salt)",
                hashcat: None,
                john: Some("dynamic_55"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224($salt.sha224($pass))",
                hashcat: None,
                john: Some("dynamic_56"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224(sha224($salt).sha224($pass))",
                hashcat: None,
                john: Some("dynamic_57"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha224(sha224($pass).sha224($pass))",
                hashcat: None,
                john: Some("dynamic_58"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "Haval-224",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "SHA3-224",
                hashcat: Some(17300),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-256(224)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512(224)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-224",
                hashcat: None,
                john: Some("dynamic_330"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Keccak-224",
                hashcat: Some(17700),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$2[abxy]?|\$2)\$[0-9]{2}\$[a-z0-9\/.]{53}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Blowfish(OpenBSD)",
                hashcat: Some(3200),
                john: Some("bcrypt"),
                extended: false,
                description: Some("Can be used in Linux Shadow Files."),
            },
            HashInfo {
                name: "Woltlab Burning Board 4.x",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "bcrypt",
                hashcat: Some(3200),
                john: Some("bcrypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$y\$[.\/A-Za-z0-9]+\$[.\/a-zA-Z0-9]+\$[.\/A-Za-z0-9]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "yescrypt",
                hashcat: None,
                john: Some("On systems that use libxcrypt, you may use --format=crypt to use JtR in passthrough mode which uses the system's crypt function."),
                extended: false,
                description: Some("Can be used in Linux Shadow Files in modern Linux distributions like Ubuntu 22.04, Debian 11, Fedora 35. On hashcat this is not yet implemented, please vote (👍 \"thumbs up\") on this issue: https://github.com/hashcat/hashcat/issues/2816."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{40}:[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Android PIN",
                hashcat: Some(5800),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(S:)?[a-f0-9]{40}(:)?[a-f0-9]{20}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Oracle 11g/12c",
                hashcat: Some(112),
                john: Some("oracle11"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$bcrypt-sha256\$(2[axy]|2)\,[0-9]+\$[a-z0-9\/.]{22}\$[a-z0-9\/.]{31}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "bcrypt(SHA-256)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:.{3}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "vBulletin < v3.8.5",
                hashcat: Some(2611),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:.{30}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "vBulletin ≥ v3.8.5",
                hashcat: Some(2711),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$snefru\$)?[a-f0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Snefru-256",
                hashcat: None,
                john: Some("snefru-256"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{64}(:.+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-256",
                hashcat: Some(1400),
                john: Some("raw-sha256"),
                extended: false,
                description: Some("256-bit key and is a good partner-function for AES. Can be used in Shadow files."),
            },
            HashInfo {
                name: "RIPEMD-256",
                hashcat: None,
                john: Some("dynamic_140"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-256 (3 rounds)",
                hashcat: None,
                john: Some("dynamic_140"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-256 (4 rounds)",
                hashcat: None,
                john: Some("dynamic_290"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Haval-256 (5 rounds)",
                hashcat: None,
                john: Some("dynamic_300"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "GOST R 34.11-94",
                hashcat: Some(6900),
                john: Some("gost"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "GOST CryptoPro S-Box",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Blake2b-256",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "SHA3-256",
                hashcat: Some(17400),
                john: Some("dynamic_380"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PANAMA",
                hashcat: None,
                john: Some("dynamic_320"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "BLAKE2-256",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "BLAKE2-384",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-256",
                hashcat: None,
                john: Some("skein-256"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512(256)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Ventrilo",
                hashcat: None,
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256($pass.$salt)",
                hashcat: Some(1410),
                john: Some("dynamic_62"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256($salt.$pass)",
                hashcat: Some(1420),
                john: Some("dynamic_61"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(sha256($pass))",
                hashcat: Some(1420),
                john: Some("dynamic_63"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(sha256_raw($pass)))",
                hashcat: Some(1420),
                john: Some("dynamic_64"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(sha256($pass).$salt)",
                hashcat: Some(1420),
                john: Some("dynamic_65"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256($salt.sha256($pass))",
                hashcat: Some(1420),
                john: Some("dynamic_66"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(sha256($salt).sha256($pass))",
                hashcat: Some(1420),
                john: Some("dynamic_67"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(sha256($pass).sha256($pass))",
                hashcat: Some(1420),
                john: Some("dynamic_68"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256(unicode($pass).$salt)",
                hashcat: Some(1430),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha256($salt.unicode($pass))",
                hashcat: Some(1440),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA256 (key = $pass)",
                hashcat: Some(1450),
                john: Some("hmac-sha256"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA256 (key = $salt)",
                hashcat: Some(1460),
                john: Some("hmac-sha256"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[a-z0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Joomla < v2.5.18",
                hashcat: Some(11),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SAM(LM_Hash:NT_Hash)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$chap\$0\*)?[a-f0-9]{32}[\*:][a-f0-9]{32}(:[0-9]{2})?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MD5(Chap)",
                hashcat: Some(4800),
                john: Some("chap"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "iSCSI CHAP Authentication",
                hashcat: Some(4800),
                john: Some("chap"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$episerver\$\*0\*[a-z0-9\/=+]+\*[a-z0-9\/=+]{27,28}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "EPiServer 6.x < v4",
                hashcat: Some(141),
                john: Some("episerver"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{ssha256\}[0-9]{2}\$[a-z0-9$\/.]{60}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "AIX(ssha256)",
                hashcat: Some(6400),
                john: Some("aix-ssha256"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{80}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RIPEMD-320",
                hashcat: None,
                john: Some("dynamic_150"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$episerver\$\*1\*[a-z0-9\/=+]+\*[a-z0-9\/=+]{42,43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "EPiServer 6.x ≥ v4",
                hashcat: Some(1441),
                john: Some("episerver"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x0100[a-f0-9]{88}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MSSQL(2000)",
                hashcat: Some(131),
                john: Some("mssql"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{96}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-384",
                hashcat: Some(10800),
                john: Some("raw-sha384"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "SHA3-384",
                hashcat: None,
                john: Some("dynamic_390"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512(384)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-1024(384)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "sha384($salt.$pass)",
                hashcat: None,
                john: Some("dynamic_71"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384($pass.$salt)",
                hashcat: None,
                john: Some("dynamic_72"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384(sha384($pass))",
                hashcat: None,
                john: Some("dynamic_73"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384(sha384_raw($pass))",
                hashcat: None,
                john: Some("dynamic_74"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384(sha384($pass).$salt)",
                hashcat: None,
                john: Some("dynamic_75"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384($salt.sha384($pass))",
                hashcat: None,
                john: Some("dynamic_76"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384(sha384($salt).sha384($pass))",
                hashcat: None,
                john: Some("dynamic_77"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha384(sha384($pass).sha384($pass))",
                hashcat: None,
                john: Some("dynamic_78"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "Skein-384",
                hashcat: None,
                john: Some("dynamic_350"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{SSHA512\}[a-z0-9\/+]{96}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SSHA-512(Base64)",
                hashcat: Some(1711),
                john: Some("ssha512"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "LDAP(SSHA-512)",
                hashcat: Some(1711),
                john: Some("ssha512"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{ssha512\}[0-9]{2}\$[a-z0-9\/.]{16,48}\$[a-z0-9\/.]{86}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "AIX(ssha512)",
                hashcat: Some(6500),
                john: Some("aix-ssha512"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{128}(:.+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-512",
                hashcat: Some(1700),
                john: Some("raw-sha512"),
                extended: false,
                description: Some("Used in Bitcoin Blockchain and Shadow Files."),
            },
            HashInfo {
                name: "Keccak-512",
                hashcat: Some(1800),
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Whirlpool",
                hashcat: Some(6100),
                john: Some("whirlpool"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Salsa10",
                hashcat: None,
                john: None,
                extended: false,
                description: Some("Not considered a hash function.[link = https://bugs.php.net/bug.php?id=60783]See more[/link]"),
            },
            HashInfo {
                name: "Salsa20",
                hashcat: None,
                john: None,
                extended: false,
                description: Some("Not considered a hash function.[link = https://bugs.php.net/bug.php?id=60783]See more[/link]"),
            },
            HashInfo {
                name: "Blake2",
                hashcat: Some(600),
                john: Some("raw-blake2"),
                extended: false,
                description: Some("Used in Wireguard, Zcash, IPFS and more.[link = https://en.wikipedia.org/wiki/BLAKE_(hash_function)#Users_of_BLAKE2]See more[/link]"),
            },
            HashInfo {
                name: "SHA3-512",
                hashcat: Some(17600),
                john: Some("raw-sha3"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-512",
                hashcat: None,
                john: Some("skein-512"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Skein-1024(512)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "sha512($pass.$salt)",
                hashcat: Some(1710),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha512($salt.$pass)",
                hashcat: Some(1720),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha512(unicode($pass).$salt)",
                hashcat: Some(1730),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "sha512($salt.unicode($pass))",
                hashcat: Some(1740),
                john: None,
                extended: true,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA512 (key = $pass)",
                hashcat: Some(1750),
                john: Some("hmac-sha512"),
                extended: true,
                description: None,
            },
            HashInfo {
                name: "BLAKE2-224",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
            HashInfo {
                name: "HMAC-SHA512 (key = $salt)",
                hashcat: Some(1760),
                john: Some("hmac-sha512"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keccak-256",
                hashcat: Some(17800),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{96}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Keccak-384",
                hashcat: Some(17900),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{136}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "OSX v10.7",
                hashcat: Some(1722),
                john: Some("xsha512"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x0200[a-f0-9]{136}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MSSQL(2012)",
                hashcat: Some(1731),
                john: Some("mssql12"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "MSSQL(2014)",
                hashcat: Some(1731),
                john: Some("mssql12"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$ml\$[0-9]+\$[a-f0-9]{64}\$[a-f0-9]{128}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "OSX v10.8",
                hashcat: Some(7100),
                john: Some("pbkdf2-hmac-sha512"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "OSX v10.9",
                hashcat: Some(7100),
                john: Some("pbkdf2-hmac-sha512"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{256}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Skein-1024",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^grub\.pbkdf2\.sha512\.[0-9]+\.([a-f0-9]{128,2048}\.|[0-9]+\.)?[a-f0-9]{128}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "GRUB 2",
                hashcat: Some(7200),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^sha1\$[a-z0-9]+\$[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(SHA-1)",
                hashcat: Some(124),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{49}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Citrix Netscaler",
                hashcat: Some(8100),
                john: Some("citrix_ns10"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$S\$[a-z0-9\/.]{52}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Drupal > v7.x",
                hashcat: Some(7900),
                john: Some("drupal7"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$5\$(rounds=[0-9]+\$)?[a-z0-9\/.]{0,16}\$[a-z0-9\/.]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-256 Crypt",
                hashcat: Some(7400),
                john: Some("sha256crypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x[a-f0-9]{4}[a-f0-9]{16}[a-f0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Sybase ASE",
                hashcat: Some(8000),
                john: Some("sybasease"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$6\$(rounds=[0-9]+\$)?[a-z0-9\/.]{0,16}\$[a-z0-9\/.]{86}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-512 Crypt",
                hashcat: Some(1800),
                john: Some("sha512crypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$sha\$[a-z0-9]{1,16}\$([a-f0-9]{32}|[a-f0-9]{40}|[a-f0-9]{64}|[a-f0-9]{128}|[a-f0-9]{140})$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Minecraft(AuthMe Reloaded)",
                hashcat: Some(20711),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^sha256\$[a-z0-9]+\$[a-f0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(SHA-256)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^sha384\$[a-z0-9]+\$[a-f0-9]{96}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(SHA-384)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^crypt1:[a-z0-9+=]{12}:[a-z0-9+=]{12}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Clavister Secure Gateway",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{112}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco VPN Client(PCF-File)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{1329}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft MSTSC(RDP-File)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[^\\\/:*?"<>|]{1,20}[:]{2,3}([^\\\/:*?"<>|]{1,20})?:[a-f0-9]{48}:[a-f0-9]{48}:[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "NetNTLMv1-VANILLA / NetNTLMv1+ESS",
                hashcat: Some(5500),
                john: Some("netntlm"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^([^\\\/:*?"<>|]{1,20}\\)?[^\\\/:*?"<>|]{1,20}[:]{2,3}([^\\\/:*?"<>|]{1,20}:)?[^\\\/:*?"<>|]{1,20}:[a-f0-9]{32}:[a-f0-9]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "NetNTLMv2",
                hashcat: Some(5600),
                john: Some("netntlmv2"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$(krb5pa|mskrb5)\$(23)?\$.+\$[a-f0-9]{1,}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5 AS-REQ Pre-Auth",
                hashcat: Some(7500),
                john: Some("krb5pa-md5"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$scram\$[0-9]+\$[a-z0-9\/.]{16}\$sha-1=[a-z0-9\/.]{27},sha-256=[a-z0-9\/.]{43},sha-512=[a-z0-9\/.]{86}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SCRAM Hash",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{40}:[a-f0-9]{0,32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Redmine Project Management Web App",
                hashcat: Some(4521),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^([^$]+)?\$[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SAP CODVN B (BCODE)",
                hashcat: Some(7700),
                john: Some("sapb"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(.+)?\$[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SAP CODVN F/G (PASSCODE)",
                hashcat: Some(7800),
                john: Some("sapg"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(.+\$)?[a-z0-9\/.+]{30}(:.+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Juniper Netscreen/SSG(ScreenOS)",
                hashcat: Some(22),
                john: Some("md5ns"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^0x(?:[a-f0-9]{60}|[a-f0-9]{40})$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "EPi",
                hashcat: Some(123),
                john: None,
                extended: false,
                description: Some("Hashcat mode is no longer supported."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{40}:[^*]{1,25}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SMF ≥ v1.1",
                hashcat: Some(121),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$wbb3\$\*1\*)?[a-f0-9]{40}[:*][a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Woltlab Burning Board 3.x",
                hashcat: Some(8400),
                john: Some("wbb3"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{130}(:[a-f0-9]{40})?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "IPMI2 RAKP HMAC-SHA1",
                hashcat: Some(7300),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{32}:[0-9]+:[a-z0-9_.+-]+@[a-z0-9-]+\.[a-z0-9-.]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Lastpass",
                hashcat: Some(6800),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/.]{16}([:$].{1,})?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco-ASA(MD5)",
                hashcat: Some(2410),
                john: Some("asa-md5"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$vnc\$\*[a-f0-9]{32}\*[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "VNC",
                hashcat: None,
                john: Some("vnc"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9]{32}(:([a-z0-9-]+\.)?[a-z0-9-.]+\.[a-z]{2,7}:.+:[0-9]+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "DNSSEC(NSEC3)",
                hashcat: Some(8300),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(user-.+:)?\$racf\$\*.+\*[a-f0-9]{16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RACF",
                hashcat: Some(8500),
                john: Some("racf"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$3\$\$[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "NTHash(FreeBSD Variant)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$sha1\$[0-9]+\$[a-z0-9\/.]{0,64}\$[a-z0-9\/.]{28}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SHA-1 Crypt",
                hashcat: Some(15100),
                john: Some("sha1crypt"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{70}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "hMailServer",
                hashcat: Some(1421),
                john: Some("hmailserver"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[:\$][AB][:\$]([a-f0-9]{1,8}[:\$])?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MediaWiki",
                hashcat: Some(3711),
                john: Some("mediawiki"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{140}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Minecraft(xAuth)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pbkdf2(-sha1)?\$[0-9]+\$[a-z0-9\/.]+\$[a-z0-9\/.]{27}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2-SHA1(Generic)",
                hashcat: Some(20400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pbkdf2-sha256\$[0-9]+\$[a-z0-9\/.]+\$[a-z0-9\/.]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2-SHA256(Generic)",
                hashcat: Some(20300),
                john: Some("pbkdf2-hmac-sha256"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pbkdf2-sha512\$[0-9]+\$[a-z0-9\/.]+\$[a-z0-9\/.]{86}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2-SHA512(Generic)",
                hashcat: Some(20200),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$p5k2\$[0-9]+\$[a-z0-9\/+=-]+\$[a-z0-9\/+-]{27}=$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2(Cryptacular)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$p5k2\$[0-9]+\$[a-z0-9\/.]+\$[a-z0-9\/.]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2(Dwayne Litzenberger)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{FSHP[0123]\|[0-9]+\|[0-9]+\}[a-z0-9\/+=]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Fairly Secure Hashed Password",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$PHPS\$.+\$[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PHPS",
                hashcat: Some(2612),
                john: Some("phps"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[0-9]{4}:[a-f0-9]{16}:[a-f0-9]{2080}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "1Password(Agile Keychain)",
                hashcat: Some(6600),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{64}:[a-f0-9]{32}:[0-9]{5}:[a-f0-9]{608}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "1Password(Cloud Keychain)",
                hashcat: Some(8200),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{256}:[a-f0-9]{256}:[a-f0-9]{16}:[a-f0-9]{16}:[a-f0-9]{320}:[a-f0-9]{16}:[a-f0-9]{40}:[a-f0-9]{40}:[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "IKE-PSK MD5",
                hashcat: Some(5300),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{256}:[a-f0-9]{256}:[a-f0-9]{16}:[a-f0-9]{16}:[a-f0-9]{320}:[a-f0-9]{16}:[a-f0-9]{40}:[a-f0-9]{40}:[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "IKE-PSK SHA1",
                hashcat: Some(5400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/+]{27}=$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PeopleSoft",
                hashcat: Some(133),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^crypt\$[a-f0-9]{5}\$[a-z0-9\/.]{13}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(DES Crypt Wrapper)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$django\$\*1\*)?pbkdf2_sha256\$[0-9]+\$[a-z0-9]+\$[a-z0-9\/+=]{44}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(PBKDF2-HMAC-SHA256)",
                hashcat: Some(10000),
                john: Some("django"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^pbkdf2_sha1\$[0-9]+\$[a-z0-9]+\$[a-z0-9\/+=]{28}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(PBKDF2-HMAC-SHA1)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^bcrypt(\$2[axy]|\$2)\$[0-9]{2}\$[a-z0-9\/.]{53}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(bcrypt)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^md5\$[a-f0-9]+\$[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(MD5)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{PKCS5S2\}[a-z0-9\/+]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2(Atlassian)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^md5[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PostgreSQL MD5",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\([a-z0-9\/+]{49}\)$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Lotus Notes/Domino 8",
                hashcat: Some(9100),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^SCRYPT:[0-9]{1,}:[0-9]{1}:[0-9]{1}:[a-z0-9:\/+=]{1,}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "scrypt",
                hashcat: Some(8900),
                john: None,
                extended: false,
                description: Some("Used in Dogecoin and Litecoin."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$8\$[a-z0-9\/.]{14}\$[a-z0-9\/.]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco Type 8",
                hashcat: Some(9200),
                john: Some("cisco8"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$9\$[a-z0-9\/.]{14}\$[a-z0-9\/.]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco Type 9",
                hashcat: Some(9300),
                john: Some("cisco9"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$office\$\*2007\*[0-9]{2}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office 2007",
                hashcat: Some(9400),
                john: Some("office"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$office\$\*2010\*[0-9]{6}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office 2010",
                hashcat: Some(9500),
                john: Some("office"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\\$office\\$2016\\$[0-9]\\$[0-9]{6}\\$[^$]{24}\\$[^$]{88}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office 2016 - SheetProtection",
                hashcat: Some(25300),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$office\$\*2013\*[0-9]{6}\*[0-9]{3}\*[0-9]{2}\*[a-z0-9]{32}\*[a-z0-9]{32}\*[a-z0-9]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office 2013",
                hashcat: Some(9600),
                john: Some("office"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$fde\$[0-9]{2}\$[a-f0-9]{32}\$[0-9]{2}\$[a-f0-9]{32}\$[a-f0-9]{3072}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Android FDE ≤ 4.3",
                hashcat: Some(8800),
                john: Some("fde"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$krb5tgs\$23\$\*[^*]*\*\$[a-f0-9]{32}\$[a-f0-9]{64,40960}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5 TGS-REP etype 23",
                hashcat: Some(13100),
                john: Some("krb5tgs"),
                extended: false,
                description: Some("Used in Windows Active Directory."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$oldoffice\$[01]\*[a-f0-9]{32}\*[a-f0-9]{32}\*[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office ≤ 2003 (MD5+RC4)",
                hashcat: Some(9700),
                john: Some("oldoffice"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Microsoft Office ≤ 2003 (MD5+RC4) collider-mode #1",
                hashcat: Some(9710),
                john: Some("oldoffice"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$oldoffice\$[34]\*[a-f0-9]{32}\*[a-f0-9]{32}\*[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Office ≤ 2003 (SHA1+RC4)",
                hashcat: Some(9800),
                john: Some("oldoffice"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "Microsoft Office ≤ 2003 (SHA1+RC4) collider-mode #1",
                hashcat: Some(9810),
                john: Some("oldoffice"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$oldoffice\$[34]\*[a-f0-9]{32}\*[a-f0-9]{32}\*[a-f0-9]{40}:[a-f0-9]{10}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MS Office ⇐ 2003 $3, SHA1 + RC4, collider #2",
                hashcat: Some(9820),
                john: Some("oldoffice"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$radmin2\$)?[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RAdmin v2.x",
                hashcat: Some(9900),
                john: Some("radmin"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\{x-issha,\s[0-9]{4}\}[a-z0-9\/+=]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SAP CODVN H (PWDSALTEDHASH) iSSHA-1",
                hashcat: Some(10300),
                john: Some("saph"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$cram_md5\$[a-z0-9\/+=-]+\$[a-z0-9\/+=-]{52}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "CRAM-MD5",
                hashcat: Some(10200),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{16}:2:4:[a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SipHash",
                hashcat: Some(10100),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-f0-9]{4,}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco Type 7",
                hashcat: None,
                john: None,
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[a-z0-9\/.]{13,}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "BigCrypt",
                hashcat: None,
                john: Some("bcrypt"),
                extended: true,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$cisco4\$)?[a-z0-9\/.]{43}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Cisco Type 4",
                hashcat: None,
                john: Some("cisco4"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^bcrypt_sha256\$\$(2[axy]|2)\$[0-9]+\$[a-z0-9\/.]{53}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Django(bcrypt-SHA256)",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$postgres\$.[^\*]+[*:][a-f0-9]{1,32}[*:][a-f0-9]{32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PostgreSQL Challenge-Response Authentication (MD5)",
                hashcat: Some(11100),
                john: Some("postgres"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$siemens-s7\$[0-9]{1}\$[a-f0-9]{40}\$[a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Siemens-S7",
                hashcat: None,
                john: Some("siemens-s7"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$pst\$)?[a-f0-9]{8}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Microsoft Outlook PST",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^sha256[:$][0-9]+[:$][a-z0-9\/+=]+[:$][a-z0-9\/+]{32,128}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PBKDF2-HMAC-SHA256(PHP)",
                hashcat: Some(10900),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^(\$dahua\$)?[a-z0-9]{8}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Dahua",
                hashcat: None,
                john: Some("dahua"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$mysqlna\$[a-f0-9]{40}[:*][a-f0-9]{40}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MySQL Challenge-Response Authentication (SHA1)",
                hashcat: Some(11200),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$pdf\$1\*[2|3]\*[0-9]{2}\*[-0-9]{1,6}\*[0-9]\*[0-9]{2}\*[a-f0-9]{32,32}\*[0-9]{2}\*[a-f0-9]{64}\*[0-9]{2}\*[a-f0-9]{64}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PDF 1.1 - 1.3 (Acrobat 2 - 4)",
                hashcat: Some(10400),
                john: Some("pdf"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "PDF 1.1 - 1.3 (Acrobat 2 - 4), collider #1",
                hashcat: Some(10410),
                john: Some("pdf"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$pdf\$1\*[2|3]\*[0-9]{2}\*[-0-9]{1,6}\*[0-9]\*[0-9]{2}\*[a-f0-9]{32}\*[0-9]{2}\*[a-f0-9]{64}\*[0-9]{2}\*[a-f0-9]{64}:[a-f0-9]{10}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PDF 1.1 - 1.3 (Acrobat 2 - 4), collider #2",
                hashcat: Some(10420),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pdf\$[24]\*[34]\*128\*[0-9-]{1,5}\*1\*(16|32)\*[a-f0-9]{32,64}\*32\*[a-f0-9]{64}\*(8|16|32)\*[a-f0-9]{16,64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PDF 1.4 - 1.6 (Acrobat 5 - 8)",
                hashcat: Some(10500),
                john: Some("pdf"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$pdf\$5\*[5|6]\*[0-9]{3}\*[-0-9]{1,6}\*[0-9]\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PDF 1.7 Level 3 (Acrobat 9)",
                hashcat: Some(10600),
                john: Some("pdf"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$pdf\$5\*[5|6]\*[0-9]{3}\*[-0-9]{1,6}\*[0-9]\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}\*[0-9]{1,4}\*[a-f0-9]{0,1024}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PDF 1.7 Level 8 (Acrobat 10 - 11)",
                hashcat: Some(10700),
                john: Some("pdf"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5asrep\$23\$[^:]+:[a-f0-9]{32,32}\$[a-f0-9]{64,40960}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5 AS-REP etype 23",
                hashcat: Some(18200),
                john: Some("krb5pa-sha1"),
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5tgs\$17\$[^$]{1,512}\$[^$]{1,512}\$[^$]{1,4}?\$?[a-f0-9]{1,32}\$[a-f0-9]{64,40960}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5 TGS-REP etype 17 (AES128-CTS-HMAC-SHA1-96)",
                hashcat: Some(19600),
                john: None,
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5tgs\$18\$[^$]{1,512}\$[^$]{1,512}\$[^$]{1,4}?\$?[a-f0-9]{1,32}\$[a-f0-9]{64,40960}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5 TGS-REP etype 18 (AES256-CTS-HMAC-SHA1-96)",
                hashcat: Some(19700),
                john: None,
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5pa\$17\$[^$]{1,512}\$[^$]{1,512}\$[a-f0-9]{104,112}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5, etype 17, Pre-Auth",
                hashcat: Some(19800),
                john: None,
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5pa\$17\$[^$]{1,512}\$[^$]{1,512}\$[^$]{0,512}\$[a-f0-9]{104,112}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5, etype 17, Pre-Auth (with salt)",
                hashcat: None,
                john: Some("krb5pa-sha1"),
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5pa\$18\$[^$]{1,512}\$[^$]{1,512}\$[^$]{0,512}\$[a-f0-9]{104,112}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5, etype 18, Pre-Auth (with salt)",
                hashcat: None,
                john: Some("krb5pa-sha1"),
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$krb5pa\$18\$[^$]{1,512}\$[^$]{1,512}\$[a-f0-9]{104,112}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Kerberos 5, etype 18, Pre-Auth",
                hashcat: Some(19900),
                john: None,
                extended: false,
                description: Some("Used for Windows Active Directory"),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$bitcoin\$[0-9]{2,4}\$[a-f0-9$]{250,350}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Bitcoin / Litecoin",
                hashcat: Some(11300),
                john: Some("bitcoin"),
                extended: false,
                description: Some("Use Bitcoin2John to extract the hash for cracking."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$ethereum\$[a-z0-9*]{150,250}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Ethereum Wallet, PBKDF2-HMAC-SHA256",
                hashcat: Some(15600),
                john: Some("ethereum-opencl"),
                extended: false,
                description: Some("Use ethereum2john to crack."),
            },
            HashInfo {
                name: "Ethereum Pre-Sale Wallet, PBKDF2-HMAC-SHA256",
                hashcat: Some(16300),
                john: Some("ethereum-presale-opencl"),
                extended: false,
                description: Some("Use ethereum2john to crack."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$monero\$(0)\*[a-f0-9]{32,3196}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Monero",
                hashcat: None,
                john: Some("monero"),
                extended: false,
                description: Some("Use monero2john to crack."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$electrum\$[1-3]\*[a-f0-9]{32,32}\*[a-f0-9]{32,32}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Electrum Wallet (Salt-Type 1-3)",
                hashcat: Some(16600),
                john: Some("electrum"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$electrum\$4\*[a-f0-9]{1,66}\*[a-f0-9]{128,32768}\*[a-f0-9]{64,64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Electrum Wallet (Salt-Type 4)",
                hashcat: Some(21700),
                john: Some("electrum"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$electrum\$5\*[a-f0-9]{66,66}\*[a-f0-9]{2048,2048}\*[a-f0-9]{64,64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Electrum Wallet (Salt-Type 5)",
                hashcat: Some(21800),
                john: Some("electrum"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$ab\$[0-9]{1}\*[0-9]{1}\*[0-9]{1,6}\*[a-f0-9]{128}\*[a-f0-9]{128}\*[a-f0-9]{32}\*[a-f0-9]{192}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Android Backup",
                hashcat: Some(18900),
                john: Some("androidbackup"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$zip2\$\*[0-9]{1}\*[0-9]{1}\*[0-9]{1}\*[a-f0-9]{16,32}\*[a-f0-9]{1,6}\*[a-f0-9]{1,6}\*[a-f0-9]+\*[a-f0-9]{20}\*\$\/zip2\$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "WinZip",
                hashcat: Some(13600),
                john: Some("zip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$itunes_backup\$\*[0-9]{1,2}\*[a-f0-9]{80}\*[0-9]{1,6}\*[a-f0-9]{40}\*[0-9]{0,10}\*[a-f0-9]{0,40}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "iTunes backup >= 10.0",
                hashcat: Some(14800),
                john: Some("itunes-backup"),
                extended: false,
                description: None,
            },
            HashInfo {
                name: "iTunes backup < 10.0",
                hashcat: Some(14700),
                john: Some("itunes-backup"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$telegram\$[a-f0-9*]{99}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Telegram Mobile App Passcode (SHA256)",
                hashcat: Some(22301),
                john: Some("Telegram"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\\$telegram\\$1\\*4000\\*[a-f0-9]{64}\\*[a-f0-9]{576}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Telegram Desktop 1.3.9",
                hashcat: None,
                john: Some("telegram"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\\$telegram\\$2\\*100000\\*[a-f0-9]{64}\\*[a-f0-9]{576}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Telegram Desktop >= 2.1.14-beta / 2.2.0",
                hashcat: None,
                john: Some("telegram"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$BLAKE2\$[a-f0-9]{128}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "BLAKE2b-512",
                hashcat: Some(600),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$oldoffice\$[a-f0-9*]{100}:[a-f0-9]{10}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MS Office ⇐ 2003 $0/$1, MD5 + RC4, collider #2",
                hashcat: Some(9720),
                john: Some("oldoffice"),
                extended: false,
                description: Some("Use office2john to grab the hash."),
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$office\$2016\$[0-9]\$[0-9]{6}\$[^$]{24}\$[^$]{88}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "MS Office 2016 - SheetProtection",
                hashcat: Some(25300),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$7z\$[0-9]\$[0-9]{1,2}\$[0-9]{1}\$[^$]{0,64}\$[0-9]{1,2}\$[a-f0-9]{32}\$[0-9]{1,10}\$[0-9]{1,6}\$[0-9]{1,6}\$[a-f0-9]{2,}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "7-zip",
                hashcat: Some(11600),
                john: Some("7z"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$zip3\$\*[0-9]\*[0-9]\*256\*[0-9]\*[a-f0-9]{0,32}\*[a-f0-9]{288}\*[0-9]\*[0-9]\*[0-9]\*[^\s]{0,64}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SecureZIP AES-256",
                hashcat: Some(23003),
                john: Some("securezip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$zip3\$\*[0-9]\*[0-9]\*192\*[0-9]\*[a-f0-9]{0,32}\*[a-f0-9]{288}\*[0-9]\*[0-9]\*[0-9]\*[^\s]{0,64}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SecureZIP AES-192",
                hashcat: Some(23002),
                john: Some("securezip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$zip3\$\*[0-9]\*[0-9]\*128\*[0-9]\*[a-f0-9]{0,32}\*[a-f0-9]{288}\*[0-9]\*[0-9]\*[0-9]\*[^\s]{0,64}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "SecureZIP AES-128",
                hashcat: Some(23001),
                john: Some("securezip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pkzip2?\$(1)\*[0-9]{1}\*[0-9]{1}\*[0-9a-f]{1,3}\*[0-9a-f]{1,8}\*[0-9a-f]{1,4}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*(8)\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[a-f0-9]+\*\$\/pkzip2?\$$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PKZIP (Compressed)",
                hashcat: Some(17200),
                john: Some("pkzip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pkzip2?\$(1)\*[0-9]{1}\*[0-9]{1}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}\*(0)\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[a-f0-9]+\*\$\/pkzip2?\$$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PKZIP (Uncompressed)",
                hashcat: Some(17210),
                john: Some("pkzip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pkzip2?\$([2-8])\*[0-9]{1}(\*[0-9]{1}\*[0-9a-f]{1,3}\*([^0*][0-9a-f]{0,2})\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[0-9a-f]+)+\*(8)\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[a-f0-9]+\*\$\/pkzip2?\$$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PKZIP (Compressed Multi-File)",
                hashcat: Some(17220),
                john: Some("pkzip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pkzip2?\$([2-8])\*[0-9]{1}(\*[0-9]{1}\*[0-9a-f]{1,8}\*([0-9a-f]{1,8})\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[0-9a-f]+)+\*([08])\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[a-f0-9]+\*\$\/pkzip2?\$$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PKZIP (Mixed Multi-File)",
                hashcat: Some(17225),
                john: Some("pkzip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$pkzip2?\$([2-8])\*[0-9]{1}(\*[0-9]{1}\*[0-9a-f]{1,3}\*[0-9a-f]{1,8}\*[0-9a-f]{1,8}(\*[0-9a-f]{1,8})?\*[0-9a-f]{1,8}\*[0-9a-f]+)+\*\$\/pkzip2?\$$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "PKZIP (Mixed Multi-File Checksum-Only)",
                hashcat: Some(17230),
                john: Some("pkzip"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$argon2i\$v=19\$m=[0-9]{1,6},t=[0-9]{1,2},p=[0-9]{1,2}\$[^$]+\$[^\s]{6,134}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Argon2i",
                hashcat: None,
                john: Some("argon2"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$argon2id\$v=19\$m=[0-9]{1,6},t=[0-9]{1,2},p=[0-9]{1,2}\$[^$]+\$[^\s]{6,134}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Argon2id",
                hashcat: None,
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$argon2d\$v=19\$m=[0-9]{1,6},t=[0-9]{1,2},p=[0-9]{1,2}\$[^$]+\$[^\s]{6,134}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Argon2d",
                hashcat: None,
                john: Some("argon2"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$bitlocker\$[0-9]\$[0-9]{2}\$[a-f0-9]{32}\$[a-f0-9]{7}\$[a-f0-9]{2}\$[a-f0-9]{24}\$[a-f0-9]{2}\$[a-f0-9]{120}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "BitLocker",
                hashcat: Some(22100),
                john: Some("bitlocker"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)\$racf\$\*.{1,}\*[A-F0-9]{16}"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RACF",
                hashcat: Some(8500),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$sshng\$4\$16\$[0-9]{32}\$1232\$[a-f0-9]{2464}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RSA/DSA/EC/OpenSSH Private Keys ($4$)",
                hashcat: Some(22941),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$RAR3\$\*(1)\*[0-9a-f]{1,16}\*[0-9a-f]{1,8}\*[0-9a-f]{1,16}\*[0-9a-f]{1,16}\*[01]\*([0-9a-f]+|[^*]{1,64}\*[0-9a-f]{1,16})\*30$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RAR3-p (Uncompressed)",
                hashcat: Some(23700),
                john: Some("rar"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$RAR3\$\*(1)\*[0-9a-f]{1,16}\*[0-9a-f]{1,8}\*[0-9a-f]{1,16}\*[0-9a-f]{1,16}\*[01]\*([0-9a-f]+|[^*]{1,64}\*[0-9a-f]{1,16})\*(31|32|33|34|35)$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RAR3-p (Compressed)",
                hashcat: Some(23800),
                john: Some("rar"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$RAR3\$\*0\*[0-9a-f]{1,16}\*[0-9a-f]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RAR3-hp",
                hashcat: Some(12500),
                john: Some("rar"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$rar5\$[0-9a-f]{1,2}\$[0-9a-f]{1,32}\$[0-9a-f]{1,2}\$[0-9a-f]{1,32}\$[0-9a-f]{1,2}\$[0-9a-f]{1,16}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "RAR5",
                hashcat: Some(13000),
                john: Some("rar5"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$keepass\$\*1\*\d+\*\d\*[0-9a-f]{32}\*[0-9a-f]{64}\*[0-9a-f]{32}\*[0-9a-f]{64}\*\d\*[^*]*(\*[0-9a-f]+)?$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "KeePass 1 AES (without keyfile)",
                hashcat: Some(13400),
                john: Some("KeePass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$keepass\$\*1\*\d+\*\d\*[0-9a-f]{32}\*[0-9a-f]{64}\*[0-9a-f]{32}\*[0-9a-f]{64}\*\d\*[^*]*(\*[0-9a-f]+)?\*\d+\*\d+\*[0-9a-f]{64}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "KeePass 1 TwoFish (with keyfile)",
                hashcat: Some(13400),
                john: Some("KeePass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$keepass\$\*2\*\d+\*\d+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "KeePass 2 AES (without keyfile)",
                hashcat: Some(13400),
                john: Some("KeePass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$keepass\$\*2\*\d+\*\d+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+\*[0-9a-f]+\*\d+\*\d+\*[0-9a-f]+$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "KeePass 2 AES (with keyfile)",
                hashcat: Some(13400),
                john: Some("KeePass"),
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^\$odf\$\*1\*1\*100000\*32\*[a-f0-9]{64}\*16\*[a-f0-9]{32}\*16\*[a-f0-9]{32}\*0\*[a-f0-9]{2048}$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "Open Document Format (ODF) 1.2 (SHA-256, AES)",
                hashcat: Some(18400),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)^[A-Za-z0-9-_]*\.[A-Za-z0-9-_]*\.[A-Za-z0-9-_]*$"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "JWT (JSON Web Token)",
                hashcat: Some(16500),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
    Prototype {
        regex: Regex::new(r##"(?i)WPA\*0[12]\*([0-9a-fA-F]+)\*"##).expect("Invalid Regex"),
        modes: vec![
            HashInfo {
                name: "WPA-PBKDF2-PMKID+EAPOL",
                hashcat: Some(22000),
                john: None,
                extended: false,
                description: None,
            },
        ],
    },
]);
