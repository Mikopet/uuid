#[cfg(any(feature = "v4", feature = "v7"))]
pub(crate) fn u128() -> u128 {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = [0u8; 16];

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        u128::from_ne_bytes(bytes)
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}

#[cfg(any(feature = "v1", feature = "v6"))]
pub(crate) fn u16() -> u16 {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = [0u8; 2];

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        u16::from_ne_bytes(bytes)
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}

#[cfg(feature = "v7")]
pub(crate) fn u64() -> u64 {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = [0u8; 8];

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        u64::from_ne_bytes(bytes)
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}
