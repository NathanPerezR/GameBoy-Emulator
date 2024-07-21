mod cart;
use crate::cart::Cart;

fn main() {
    let mut cart: Cart = Cart::default();

    cart.cart_read("/Users/nathanperez/repo/GameBoy-Emulator/Roms/Vegas-Stakes.gb");
}
