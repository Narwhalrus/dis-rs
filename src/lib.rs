mod dis;

pub use dis::parse;
pub use dis::parse_as_version;
pub use dis::parse_v6 as parse_v6_pdus;
pub use dis::parse_v7 as parse_v7_pdus;

pub use dis::v6::parse_header as parse_v6_header;
pub use dis::v6::parse_multiple_header as parse_v6_headers;

/*
TODO:
V Finish EntityState model and builder functions (such as Appearance impl)
V Tests for parsing EntityState PDU
V Tests for parsing Other PDU
- Tests for parsing incomplete input / some santity checks on pdu lengths
- Writing headers and pdus to buffer/network
- Build DIS v7 Header; model, builder, parser
- Incorporate versions of enumeration document into the lib (domain types, country codes, etc). Possibly set version to use as config option; default to latest.

TESTS:
- Build Other PDU
V Read Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

ISSUES:
- Decide on use of 'Unspecified' enum values
- From/TryFrom impls for PDUs parsing?

*/