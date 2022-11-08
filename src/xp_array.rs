#[derive(Eq, PartialEq, Debug, Copy)]
pub struct Vault {
    x: i32,
    y: i32,
}

impl Clone for Vault {
    fn clone(&self) -> Self {
        return *self;
    }
}

#[cfg(test)]
mod tests {
    use super::Vault;

    #[test]
    fn array_0() {
        let xs: [i32; 0] = [];
        let ys: [i32; 0] = xs;

        assert_eq!(xs, ys);
    }

    #[test]
    fn array_1() {
        let xs: [i32; 1] = [1];
        let ys: [i32; 1] = xs;

        assert_eq!(xs, ys);
    }

    #[test]
    fn array_33() {
        let xs: [i32; 33] = [42; 33];
        let ys: [i32; 33] = xs;

        assert_eq!(xs, ys);
    }

    #[test]
    fn array_of_vault_0() {
        let xs: [Vault; 0] = [];
        let ys: [Vault; 0] = xs;

        assert_eq!(xs, ys);
    }

    #[test]
    fn array_of_vault_1_0() {
        let xs: [Vault; 1] = [Vault { x: 42, y: 43 }];
        let ys: [Vault; 1] = xs;

        assert_eq!(xs, ys);
    }
}
