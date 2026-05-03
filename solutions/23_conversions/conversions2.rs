// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// Frank the fairy would like to buy some truffles from Grace the gnome, a
// world-renowned chocolatier. The truffles are priced in GnomeCoin though, and
// Frank only has FairyCredit. Help Frank by providing a `From` implementation
// to convert his FairyCredit to GnomeCoin. At the current exchange rate, one
// FairyCredit is valued at 100 GnomeCoin.

#[derive(Debug)]
struct FairyCredit(u32);

#[derive(Debug, PartialEq)]
struct GnomeCoin(u64);

impl From<FairyCredit> for GnomeCoin {
    fn from(value: FairyCredit) -> Self {
        Self(value.0 as u64 * 100)
    }
}

// Note that we shouldn't provide the opposite conversion: from GnomeCoin to
// FairyCredits. That's because less than 100 GnomeCoins cannot be represented
// as FairyCredits, which would make the conversion lossy. The `From` trait is
// only appropriate for infallible and lossless conversions.

fn main() {
    // Use the `from` function.
    let g1 = GnomeCoin::from(FairyCredit(12));
    println!("{g1:?}");

    // Since `From` is implemented for GnomeCoin, we are able to use `Into`.
    let g2: GnomeCoin = FairyCredit(9).into();
    println!("{g2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let g = GnomeCoin::from(FairyCredit(12));
        assert_eq!(g, GnomeCoin(1200));
    }

    #[test]
    fn test_into() {
        let g: GnomeCoin = FairyCredit(9).into();
        assert_eq!(g, GnomeCoin(900));
    }
}
