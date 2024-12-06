use bdk_integration; // Import the newly created file containing the BDK integration code

fn main() {
    let scan_privkey = [0x01, 0x02, 0x03, 0x04, 0x05]; // Replace with your actual scan private key
    let spend_privkey = [0x06, 0x07, 0x08, 0x09, 0x0a]; // Replace with your actual spend private key

    let silent_payment_address = bdk_integration::derive_silent_payment_address(scan_privkey, spend_privkey);

    println!("{}", silent_payment_address);
}
